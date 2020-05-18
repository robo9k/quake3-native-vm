#[macro_use]
extern crate quake3_native_vm;

use quake3_native_vm::ffi;
use quake3_native_vm::qagame::{Module, Syscalls};

struct HelloQuake3 {
    syscalls: Syscalls,
}

impl Module for HelloQuake3 {
    fn dll_entry(syscalls: Syscalls) -> Box<HelloQuake3> {
        Box::new(HelloQuake3 { syscalls: syscalls })
    }

    fn init(&self, level_time: ffi::c_int, random_seed: ffi::c_int, restart: bool) {
        println!("init: level_time={}, random_seed={}, restart={}", level_time, random_seed, restart);

        self.syscalls.error("Hello, World!");
        unreachable!()
    }

    fn shutdown(&self, restart: bool) {
        println!("shutdown: restart={}", restart);
    }
}

game_module!(HelloQuake3);
