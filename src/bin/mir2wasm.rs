#![feature(rustc_private, custom_attribute)]
#![allow(unused_attributes)]

extern crate env_logger;
extern crate getopts;
extern crate mir2wasm;
extern crate rustc;
extern crate rustc_driver;

use mir2wasm::trans;
use rustc::session::Session;
use rustc_driver::{driver, CompilerCalls};

struct MiriCompilerCalls;

impl<'a> CompilerCalls<'a> for MiriCompilerCalls {
    fn build_controller(
        &mut self,
        _: &Session,
        _: &getopts::Matches
    ) -> driver::CompileController<'a> {
        let mut control = driver::CompileController::basic();

        control.after_analysis.stop = rustc_driver::Compilation::Stop;
        control.after_analysis.callback = Box::new(|state| {
            state.session.abort_if_errors();
            trans::trans_crate(state.tcx.unwrap(), state.mir_map.unwrap())
                .unwrap(); // FIXME
        });

        control
    }
}

#[miri_run]
fn main() {
    env_logger::init().unwrap();

    let mut args: Vec<String> = std::env::args().collect();
    args.push("--target=arm-unknown-linux-gnueabi".to_string());
    rustc_driver::run_compiler(&args, &mut MiriCompilerCalls);
}
