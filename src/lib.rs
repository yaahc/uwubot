//! Uwuifying discord bot for conveniently uwuifying text with friends
#![doc(html_root_url = "https://docs.rs/uwubot/0.3.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    rust_2018_idioms,
    missing_docs,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![allow(clippy::try_err)]

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
