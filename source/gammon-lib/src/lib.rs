pub mod interface;

pub mod republish {
    pub use serde_json;
    pub use chrono;
}

#[macro_export]
macro_rules! unenum{
    ($s: expr, $m: pat => $v: expr) => {
        match $s {
            $m => Some($v),
            _ => None,
        }
    }
}

/// Explicitly communicate the block return type to the compiler via unreachable
/// code.
#[macro_export]
macro_rules! ta_res{
    ($t: ty) => {
        if false {
            fn unreachable_value<T>() -> T {
                panic!();
            }
            return std:: result:: Result::< $t,
            loga::Error > ::Ok(unreachable_value());
        }
    }
}

/// Explicitly capturing async closure - clones elements in the second parens into
/// the closure. Anything else will be moved.
#[macro_export]
macro_rules! cap_fn{
    (($($a: pat_param), *)($($cap: ident), *) {
        $($t: tt) *
    }) => {
        {
            $(let $cap = $cap.clone();) * move | $($a),
            *| {
                $(let $cap = $cap.clone();) * async move {
                    $($t) *
                }
            }
        }
    };
}
