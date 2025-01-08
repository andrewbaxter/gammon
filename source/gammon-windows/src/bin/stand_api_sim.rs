use {
    arcade_shared::{
        interface::{
            general::PlayerMeta,
            proto::gamestand::{
                self,
                v1::AbsClientPaymentId,
                GameStandReq,
                STAND_GAME_SOCKET,
            },
        },
        ta_res,
    },
    flowcontrol::shed,
    http::Response,
    http_body_util::BodyExt,
    htwrap::htserve::{
        self,
        handler::{
            async_trait::async_trait,
            HandlerArgs,
        },
        responses::{
            response_503,
            Body,
        },
    },
    loga::{
        Log,
        ResultContext,
    },
    native_windows_derive::NwgUi,
    native_windows_gui::{
        stretch::{
            geometry::Rect,
            style::Dimension,
        },
        Button,
        FlexboxLayout,
        NativeUi,
        Window,
    },
    std::{
        collections::HashMap,
        panic,
        process,
        sync::{
            atomic::{
                AtomicBool,
                Ordering,
            },
            Arc,
            Mutex,
        },
        thread::spawn,
        time::{
            Duration,
            Instant,
        },
    },
    tokio::{
        net::TcpListener,
        select,
        time::sleep,
    },
};

struct GlobalState {
    tapped: AtomicBool,
    reader_on: Mutex<Option<Instant>>,
}

// Gui
const PT_5: Dimension = Dimension::Points(5.0);
const PT_10: Dimension = Dimension::Points(10.0);
const PADDING: Rect<Dimension> = Rect {
    start: PT_10,
    end: PT_10,
    top: PT_10,
    bottom: PT_10,
};
const MARGIN: Rect<Dimension> = Rect {
    start: PT_5,
    end: PT_5,
    top: PT_5,
    bottom: PT_5,
};

#[derive(NwgUi)]
pub struct Ui {
    global: Arc<GlobalState>,
    #[nwg_control(title: "Gammon API simulator")]
    #[nwg_events(OnWindowClose:[nwg::stop_thread_dispatch()])]
    window: Window,
    #[nwg_layout(parent: window, flex_direction: stretch:: style:: FlexDirection:: Row, padding: PADDING)]
    layout: FlexboxLayout,
    #[nwg_control(text: "Tap charged card")]
    #[nwg_layout_item(layout: layout, margin: MARGIN)]
    #[nwg_events(OnButtonClick:[Ui::tap])]
    tap: Button,
}

impl Ui {
    fn tap(&self) {
        if self.global.reader_on.lock().unwrap().is_none() {
            eprintln!("Reader not on currently, turns on when game sends first `ask_payment` API call");
        } else {
            self.global.tapped.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

// Server
struct PaymentState {
    released: bool,
    committed: bool,
    payment: gamestand::latest::Payment,
}

struct ServerState {
    log: Log,
    global: Arc<GlobalState>,
    seen_payments: Mutex<HashMap<AbsClientPaymentId, PaymentState>>,
}

#[async_trait]
impl htserve::handler::Handler<Body> for ServerState {
    async fn handle(&self, args: HandlerArgs<'_>) -> Response<Body> {
        match async {
            ta_res!(Response < Body >);
            let req =
                serde_json::from_slice::<GameStandReq>(
                    &args.body.collect().await.context("Error reading request bytes")?.to_bytes(),
                ).context("Failed to parse json request body")?;

            mod resp {
                use {
                    arcade_shared::interface::proto::gamestand,
                    http::Response,
                    htwrap::htserve::responses::{
                        response_200_json,
                        Body,
                    },
                    serde::Serialize,
                };

                // Private constructor
                pub struct RespToken(());

                pub type Resp = Response<Body>;

                pub trait ReqResp: gamestand::latest::RequestTrait {
                    fn respond(&self) -> fn(Self::Response) -> (RespToken, Resp) {
                        fn acceptor<T: Serialize>(r: T) -> (RespToken, Resp) {
                            return (RespToken(()), response_200_json(gamestand::Response::Ok(r)));
                        }

                        return acceptor::<Self::Response>;
                    }
                }

                impl ReqResp for gamestand::latest::ReqAskPayment { }

                impl ReqResp for gamestand::latest::ReqCommitClearPayments { }

                impl ReqResp for gamestand::latest::ReqReleasePayment { }
            }

            use resp::ReqResp;

            let resp;
            match req {
                gamestand::GameStandReq::V1(r) => match r {
                    gamestand::v1::Request::AskPayment(req) => {
                        let responder = req.respond();
                        {
                            let mut reader_on = self.global.reader_on.lock().unwrap();
                            if reader_on.is_none() {
                                eprintln!("Turning on card reader");
                            }
                            *reader_on = Some(Instant::now());
                        }
                        shed!{
                            if let Some(p) = self.seen_payments.lock().unwrap().get(&req.client_payment_id) {
                                if !p.released {
                                    resp =
                                        responder(
                                            gamestand::latest::RespAskPayment { payment: Some(p.payment.clone()) },
                                        );
                                    eprintln!("Requested payment - id matches previous payment");
                                    break;
                                }
                            }
                            let got_payment =
                                self.global.tapped.fetch_and(false, std::sync::atomic::Ordering::Relaxed);
                            if got_payment {
                                let payment = gamestand::latest::Payment {
                                    client_payment_id: req.client_payment_id.clone(),
                                    player_meta: PlayerMeta {
                                        name: "名無し".to_string(),
                                        three_letter_name: "nns".to_string(),
                                        color: [1., 0., 0.],
                                    },
                                };
                                self.seen_payments.lock().unwrap().insert(req.client_payment_id, PaymentState {
                                    released: false,
                                    committed: false,
                                    payment: payment.clone(),
                                });
                                resp = responder(gamestand::latest::RespAskPayment { payment: Some(payment) });
                                eprintln!("Requested payment - got new payment");
                            } else {
                                resp = responder(gamestand::latest::RespAskPayment { payment: None });
                            }
                        }
                    },
                    gamestand::v1::Request::ReleasePayment(req) => {
                        let responder = req.respond();
                        match self.seen_payments.lock().unwrap().remove(&req.client_payment_id).is_some() {
                            true => eprintln!("Releasing payment - found existing payment"),
                            false => eprintln!("Releasing payment - errror: unknown payment, doing nothing"),
                        }
                        resp = responder(());
                    },
                    gamestand::v1::Request::CommitClearPayments(req) => {
                        let responder = req.respond();
                        {
                            let mut seen_payments = self.seen_payments.lock().unwrap();
                            for id in &req.commit_payment_ids {
                                match seen_payments.get_mut(id) {
                                    Some(s) => {
                                        if s.released {
                                            eprintln!(
                                                "Commit/release payments - error: trying to commit released payment {:?}",
                                                id
                                            );
                                        }
                                        if s.committed {
                                            eprintln!(
                                                "Commit/release payments - error: trying to commit already committed payment {:?}",
                                                id
                                            );
                                        }
                                        s.committed = true;
                                    },
                                    None => {
                                        eprintln!(
                                            "Commit/release payments - error: committing unknown payment {:?}",
                                            id
                                        );
                                    },
                                }
                            }
                            for (id, p) in seen_payments.iter_mut() {
                                if !p.committed {
                                    p.released = true;
                                    eprintln!("Commit/release payments - releasing uncommitted payment {:?}", id);
                                }
                            }
                        }
                        resp = responder(());
                    },
                },
            }
            return Ok(resp.1);
        }.await {
            Ok(r) => r,
            Err(e) => {
                self.log.log_err(loga::WARN, e.context("Error serving response"));
                return response_503();
            },
        }
    }
}

// Main
fn main() {
    {
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            orig_hook(panic_info);
            process::exit(1);
        }));
    }
    let global = Arc::new(GlobalState {
        tapped: AtomicBool::new(false),
        reader_on: Default::default(),
    });
    spawn({
        let global = global.clone();
        move || tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on({
            async move {
                let log = Log::new_root(loga::INFO);
                let listener = TcpListener::bind(STAND_GAME_SOCKET).await.unwrap();
                let state = Arc::new(ServerState {
                    log: log.clone(),
                    global: global.clone(),
                    seen_payments: Default::default(),
                });
                loop {
                    select!{
                        conn = listener.accept() => {
                            let (conn, _peer) = match conn {
                                Ok(c) => c,
                                Err(e) => {
                                    eprintln!("Error accepting connection: {:?}", e);
                                    continue;
                                },
                            };
                            tokio::spawn({
                                let log = log.clone();
                                let state = state.clone();
                                async move {
                                    match htserve::handler::root_handle_http(&log, state, conn).await {
                                        Ok(_) => { },
                                        Err(e) => {
                                            log.log_err(loga::WARN, e.context("Error responding to request"));
                                        },
                                    };
                                }
                            });
                        },
                        _ = sleep(Duration::from_millis(100)) => {
                            {
                                let mut global_reader_on = global.reader_on.lock().unwrap();
                                if let Some(reader_on) = &*global_reader_on {
                                    if reader_on.duration_since(Instant::now()) > Duration::from_secs(2) {
                                        eprintln!("Reader deactivating");
                                        *global_reader_on = None;
                                        global.tapped.store(false, Ordering::Relaxed);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    });
    native_windows_gui::init().unwrap();
    let _ui = Ui::build_ui(Ui {
        global: global,
        window: Default::default(),
        layout: Default::default(),
        tap: Default::default(),
    }).unwrap();
    native_windows_gui::dispatch_thread_events();
}
