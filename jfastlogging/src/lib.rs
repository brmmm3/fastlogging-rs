mod def;
pub use def::Java_org_logging_FastLogging_extConfigNew;
mod logging;
mod writer;
pub use logging::*;
mod logger;
pub use logger::*;

#[macro_export]
macro_rules! log_message {
    ($env:ident, $logger:ident, $method:ident, $message:ident) => {{
        let message: String = match $env.get_string(&$message) {
            Ok(s) => s.into(),
            Err(_err) => "null".to_string(),
        };
        if let Err(err) = $logger.$method(message) {
            $env.throw(err.to_string()).unwrap();
            return -1;
        }
        0
    }};
}

#[macro_export]
macro_rules! get_string {
    ($env:ident, $js:ident) => {
        get_string!($env, $js, -1)
    };
    ($env:ident, $js:ident, $ret:expr) => {
        match $env.get_string(&$js) {
            Ok(s) => {
                let s: String = s.into();
                s
            }
            Err(err) => {
                $env.throw(err.to_string()).unwrap();
                return $ret;
            }
        }
    };
}

#[macro_export]
macro_rules! get_pathbuf {
    ($env:ident, $js:ident) => {
        get_pathbuf!($env, $js, -1)
    };
    ($env:ident, $js:ident, $ret:expr) => {
        match $env.get_string(&$js) {
            Ok(s) => {
                let s: String = s.into();
                PathBuf::from(s)
            }
            Err(err) => {
                $env.throw(err.to_string()).unwrap();
                return $ret;
            }
        }
    };
}
