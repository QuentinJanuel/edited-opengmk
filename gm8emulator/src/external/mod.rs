pub mod audio;
pub mod time;
pub mod logger;
pub mod input;
pub mod renderer;

use std::{
    sync::{Arc, Mutex, MutexGuard},
    pin::Pin,
    future::Future,
};
use lazy_static::lazy_static;

pub type Fut<T = ()> = Pin<Box<dyn Future<Output = T>>>;

pub trait External {
    fn audio(&self) -> Arc<dyn audio::Audio>;
    fn time(&self) -> Arc<dyn time::Time>;
    fn logger(&self) -> Arc<dyn logger::Logger>;
    fn input(&self) -> Arc<dyn input::Input>;
    fn renderer(&self) -> Arc<Mutex<dyn renderer::Renderer>>;
}

type TExternal = Arc<Mutex<Option<Arc<dyn External + Sync + Send>>>>;

lazy_static! {
    static ref EXTERNAL: TExternal = Arc::new(Mutex::from(None));
}

fn external() -> Arc<dyn External> {
    EXTERNAL
        .lock()
        .expect("External lock poisoned")
        .as_ref()
        .expect("External not set")
        .clone()
}

pub fn init(external: impl External + Send + Sync + 'static) {
    let mut glob_ext = EXTERNAL
        .lock()
        .expect("External lock poisoned");
    *glob_ext = Some(Arc::new(external));
}

pub fn audio() -> Arc<dyn audio::Audio> {
    external().audio()
}

pub fn time() -> Arc<dyn time::Time> {
    external().time()
}

pub fn input() -> Arc<dyn input::Input> {
    external().input()
}

pub fn renderer() -> Arc<Mutex<dyn renderer::Renderer>> {
    external().renderer().clone()
}

pub fn logger() -> Arc<dyn logger::Logger> {
    external().logger()
}

#[macro_export]
macro_rules! ext_log {
    ($($arg:tt)*) => {
        $crate::external::logger()
            .log(&format!($($arg)*))
    }
}

#[macro_export]
macro_rules! ext_elog {
    ($($arg:tt)*) => {
        $crate::external::logger()
            .err(&format!($($arg)*))
    }
}
