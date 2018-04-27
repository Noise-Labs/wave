
#[macro_export]
macro_rules! custom_error {
    ($i:expr,$e:expr) => { Err(Err::Error(Context::Code($i,ErrorKind::Custom($e))))};
}