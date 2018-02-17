// Needed for re-exported lazy_static! macro
#![feature(use_extern_macros)]

#[macro_use]
extern crate quake3_native_vm;

use std::ffi::CString;
use quake3_native_vm::ffi;

struct HelloQuake3 {
    syscall: Syscall,
}

use quake3_native_vm::*;

/// See [ioquake3's `game/g_public.h`](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h)
const G_ERROR: ffi::intptr_t = 1;
const GAME_INIT: ffi::c_int = 0;
const GAME_SHUTDOWN: ffi::c_int = 1;

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
        match command {
            GAME_INIT => {
                let msg = CString::new("Hello, World!").unwrap();
                (self.syscall)(G_ERROR, msg.as_ptr());
                unreachable!()
            }
            GAME_SHUTDOWN => {
                // Just return a dummy value here for clean shutdown
                0
            }
            _ => panic!("Game command not implemented"),
        }
    }
}

native_vm!(HelloQuake3);
