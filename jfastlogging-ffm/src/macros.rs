#[macro_export]
macro_rules! log_message {
    ($logger:ident, $msg_ptr:ident, $msg_len:ident, $method:ident) => {{
        if $logger.is_null() || $msg_ptr.is_null() {
            return -1;
        }
        let logger = unsafe { &mut *$logger };
        let msg = get_option_str($msg_ptr, $msg_len).unwrap_or("");
        logger.$method(msg);
        0
    }};
}
