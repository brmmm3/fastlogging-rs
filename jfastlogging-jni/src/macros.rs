#[macro_export]
macro_rules! log_message {
    ($env:ident, $logger:ident, $method:ident, $message:ident) => {{
        enter_jni($env, |env| {
            let message: String = JString::to_string(&$message);
            if let Err(err) = $logger.$method(&message) {
                env.throw(err.to_string()).unwrap();
                return Ok(-1);
            }
            Ok(0)
        })
    }};
}
