// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:quux
fn test00_start(ch: chan_t<int>, message: int) { send(ch, message); }

type task_id = int;
type port_id = int;

enum chan_t<T> = {task: task_id, port: port_id};

fn send<T: Owned>(ch: chan_t<T>, data: T) { die!(); }

fn main() { die!(~"quux"); }
