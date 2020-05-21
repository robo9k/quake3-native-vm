use quake3_native_vm::qagame::{Module, Syscalls};
use quake3_native_vm::{ffi, game_module};

struct HelloQuake3 {
    syscalls: Syscalls,
}

impl Module for HelloQuake3 {
    fn dll_entry(syscalls: Syscalls) -> Box<HelloQuake3> {
        Box::new(HelloQuake3 { syscalls: syscalls })
    }

    fn init(&self, level_time: ffi::c_int, random_seed: ffi::c_int, restart: bool) {
        println!(
            "init: level_time={}, random_seed={}, restart={}",
            level_time, random_seed, restart
        );
    }

    fn shutdown(&self, restart: bool) {
        println!("shutdown: restart={}", restart);
    }

    fn client_connect(
        &self,
        client_number: ffi::c_int,
        first_time: bool,
        is_bot: bool,
    ) -> ffi::intptr_t {
        println!(
            "client_connect: client_number={}, first_time={}, is_bot={}",
            client_number, first_time, is_bot
        );
        todo!();
    }

    fn client_think(&self, client_number: ffi::c_int) {
        println!("client_think: client_number={}", client_number);
    }

    fn client_userinfo_changed(&self, client_number: ffi::c_int) {
        println!("client_userinfo_changed: client_number={}", client_number);
    }

    fn client_disconnect(&self, client_number: ffi::c_int) {
        println!("client_disconnect: client_number={}", client_number);
    }

    fn client_begin(&self, client_number: ffi::c_int) {
        println!("client_begin: client_number={}", client_number);
    }

    fn client_command(&self, client_number: ffi::c_int) {
        println!("client_command: client_number={}", client_number);
    }

    fn run_frame(&self, level_time: ffi::c_int) {
        println!("run_frame: level_time={}", level_time);
    }

    fn console_command(&self) -> bool {
        println!("console_command");
        todo!();
    }

    fn botai_start_frame(&self, level_time: ffi::c_int) -> bool {
        println!("botai_start_frame: level_time={}", level_time);
        todo!();
    }
}

game_module!(HelloQuake3);
