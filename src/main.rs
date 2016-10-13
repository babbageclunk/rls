// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(question_mark)]
#![feature(rustc_private)]
#![feature(proc_macro)]

#[macro_use]
extern crate hyper;
extern crate rls_analysis as analysis;
extern crate rls_vfs as vfs;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::sync::Arc;

mod actions_ls;
mod actions_http;
mod actions_common;
mod build;
mod ide;
mod server;
mod lsproto;

#[cfg(test)]
mod test;

pub fn main() {
    use std::env;

    let analysis = Arc::new(analysis::AnalysisHost::new(analysis::Target::Debug));
    let vfs = Arc::new(vfs::Vfs::new());
    let build_queue = Arc::new(build::BuildQueue::new(vfs.clone()));

    let args: Vec<String> = env::args().collect();

    if args.contains(&"--http".to_string()) {
        server::run_server(analysis, vfs, build_queue);
    } else {
        lsproto::run_server(analysis, vfs, build_queue);
    }
}
