// Needed for re-exported lazy_static! macro
#![feature(use_extern_macros)]

#[macro_use]
extern crate quake3_native_vm;

extern crate num_traits;

use std::ffi::CString;
use quake3_native_vm::ffi;

struct HelloQuake3 {
    syscall: Syscall,
}

use quake3_native_vm::*;
use quake3_native_vm::qagame::{Exports, Imports};
use num_traits::{FromPrimitive, ToPrimitive};

impl NativeVM for HelloQuake3 {
    fn dll_entry(syscall: Syscall) -> Box<HelloQuake3> {
        Box::new(HelloQuake3 { syscall: syscall })
    }

    fn vm_main(
        &self,
        command: ffi::c_int,
        _arg0: ffi::c_int,
        _arg1: ffi::c_int,
        _arg2: ffi::c_int,
        _arg3: ffi::c_int,
        _arg4: ffi::c_int,
        _arg5: ffi::c_int,
        _arg6: ffi::c_int,
        _arg7: ffi::c_int,
        _arg8: ffi::c_int,
        _arg9: ffi::c_int,
        _arg10: ffi::c_int,
        _arg11: ffi::c_int,
    ) -> ffi::intptr_t {
        // FIXME: This is not exactly pretty..
        // What I need is "Exports::* from ffi::c_int" and "Imports::* to ffi::intptr_t"
        // and then easy matching and no .unwrap()
        match Exports::from_i32(command) {
            Some(Exports::GAME_INIT) => {
                let msg = CString::new("Hello, World!").unwrap();
                (self.syscall)(Imports::G_ERROR.to_isize().unwrap(), msg.as_ptr());
                unreachable!()
            }
            Some(Exports::GAME_SHUTDOWN) => {
                // Just return a dummy value here for clean shutdown
                0
            }
            _ => panic!("Game command not implemented"),
        }
    }
}

native_vm!(HelloQuake3);
