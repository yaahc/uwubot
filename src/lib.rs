mod bot;
mod config;
mod uwu;

pub use bot::Bot;
pub use config::Config;

trait ResultExt {
    fn unwrap_or_report(self);
}

impl<E> ResultExt for Result<(), E>
where
    eyre::Report: From<E>,
{
    fn unwrap_or_report(self) {
        if let Err(e) = self {
            let e = eyre::Report::from(e);
            eprintln!("Error: {:?}", e);
        }
    }
}
