pub trait Logger {
    fn log(&self, msg: &str) -> ();
    fn err(&self, msg: &str) -> ();
}
