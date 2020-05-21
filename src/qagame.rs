//! Code for the `qagame` Quake 3 module
//!
//! The `qagame` module contains the server-side logic of Quake 3, e.g.:
//!
//! * player physics
//! * game types
//! * weapons
//! * bot A.I.
//!
//! Unlike the other modules, it does not have to be present on the game client,
//! i.e. players do not have to download it.

use crate::{ffi, Syscall};
use std::ffi::CString;

/// System traps provided by the engine
///
/// See `gameImport_t` in [ioquake3's `game/g_public.h`](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h).
#[repr(C)]
// TODO: Should these be shortened and renamed, e.g. `Print` and `Error` instead of `G_PRINT` and `G_ERROR`?
#[allow(non_camel_case_types)]
pub enum Imports {
    /// Print a message to qconsole
    G_PRINT = 0,
    /// Shutdown the game with an error message
    G_ERROR = 1,
}

impl From<Imports> for isize {
    fn from(import: Imports) -> Self {
        match import {
            Imports::G_PRINT => 0,
            Imports::G_ERROR => 1,
        }
    }
}

/// Functions exported by the module
///
/// See `gameExport_t` in [ioquake3's `game/g_public.h`](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h).
#[repr(C)]
#[derive(Debug)]
// TODO: Should these be shortened and renamed, e.g. `Init` and `Shutdown` instead of `GAME_INIT` and `GAME_SHUTDOWN`?
#[allow(non_camel_case_types)]
pub enum Exports {
    /// Initialize module upon loading a level
    GAME_INIT = 0,
    /// Shutdown module upon loading another level, mod etc.
    GAME_SHUTDOWN = 1,
    /// Connect or reject a client
    GAME_CLIENT_CONNECT = 2,
    /// Client finished connecting
    GAME_CLIENT_BEGIN = 3,
    /// Userinfo data for client changed
    GAME_CLIENT_USERINFO_CHANGED = 4,
    /// Disconnect a client
    GAME_CLIENT_DISCONNECT = 5,
    /// Client text command
    GAME_CLIENT_COMMAND = 6,
    /// Run client frame
    GAME_CLIENT_THINK = 7,
    /// Run server frame
    GAME_RUN_FRAME = 8,
    /// Server console text command
    GAME_CONSOLE_COMMAND = 9,
    /// Run bot frame
    BOTAI_START_FRAME = 10,
}

impl std::convert::TryFrom<ffi::c_int> for Exports {
    type Error = &'static str;

    fn try_from(cmd: ffi::c_int) -> Result<Self, Self::Error> {
        match cmd {
            0 => Ok(Self::GAME_INIT),
            1 => Ok(Self::GAME_SHUTDOWN),
            _ => Err("Unknown command"),
        }
    }
}

/// `qagame` specific wrapper around generic [`Syscall`](Syscall)
///
/// See [ioquake3's `game/g_syscalls.c`](https://github.com/ioquake/ioq3/blob/master/code/game/g_syscalls.c).
pub struct Syscalls {
    syscall: Syscall,
}

impl Syscalls {
    /// See `dllEntry` in [ioquake3's `game/g_syscalls.c`](https://github.com/ioquake/ioq3/blob/master/code/game/g_syscalls.c).
    pub fn new(syscall: Syscall) -> Self {
        Self { syscall }
    }

    /// See `trap_Error` in [ioquake3's `game/g_syscalls.c`](https://github.com/ioquake/ioq3/blob/master/code/game/g_syscalls.c).
    pub fn error<T: Into<Vec<u8>>>(&self, text: T) {
        let msg = CString::new(text).unwrap();
        (self.syscall)(Imports::G_ERROR.into(), msg.as_ptr());
    }
}

/// See `vmMain` in [ioquake3's `game/g_main.c`](https://github.com/ioquake/ioq3/blob/master/code/game/g_main.c).
pub trait Module: 'static + Sync + Send {
    /// See `dllEntry` in [ioquake3's `game/g_syscalls.`](https://github.com/ioquake/ioq3/blob/master/code/game/g_syscalls.c)
    fn dll_entry(syscalls: Syscalls) -> Box<Self>
    where
        Self: Sized;

    /// See `G_InitGame` in [ioquake3's `game/g_main.c`](https://github.com/ioquake/ioq3/blob/master/code/game/g_main.c).
    fn init(&self, level_time: ffi::c_int, random_seed: ffi::c_int, restart: bool);

    /// See `G_ShutdownGame` in [ioquake3's `game/g_main.c`](https://github.com/ioquake/ioq3/blob/master/code/game/g_main.c).
    fn shutdown(&self, restart: bool);
}

/// Create a [NativeVM](::NativeVM) impl for the id Quake 3 `qagame` module
#[macro_export]
macro_rules! game_module {
    ($ty:ident) => {
        struct ModuleWrapper {
            module: Box<$crate::qagame::Module>,
        }

        use $crate::NativeVM;
        impl $crate::NativeVM for ModuleWrapper {
            fn dll_entry(syscall: $crate::Syscall) -> Box<Self> {
                Box::new(ModuleWrapper {
                    module: $ty::dll_entry($crate::qagame::Syscalls::new(syscall)),
                })
            }

            fn vm_main(
                &self,
                command: $crate::ffi::c_int,
                arg0: $crate::ffi::c_int,
                arg1: $crate::ffi::c_int,
                arg2: $crate::ffi::c_int,
                _arg3: $crate::ffi::c_int,
                _arg4: $crate::ffi::c_int,
                _arg5: $crate::ffi::c_int,
                _arg6: $crate::ffi::c_int,
                _arg7: $crate::ffi::c_int,
                _arg8: $crate::ffi::c_int,
                _arg9: $crate::ffi::c_int,
                _arg10: $crate::ffi::c_int,
                _arg11: $crate::ffi::c_int,
            ) -> $crate::ffi::intptr_t {
                use std::convert::TryFrom;

                match $crate::qagame::Exports::try_from(command) {
                    Ok($crate::qagame::Exports::GAME_INIT) => {
                        self.module.init(arg0, arg1, arg2 != 0);
                        0
                    }
                    Ok($crate::qagame::Exports::GAME_SHUTDOWN) => {
                        self.module.shutdown(arg0 != 0);
                        0
                    }
                    Ok(command) => todo!("Game command {:?} not implemented", command),

                    Err(command) => panic!("Unknown game command {:?}", command),
                }
            }
        }

        $crate::native_vm!(ModuleWrapper);
    };
}
