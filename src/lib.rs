//! Native Rust libs as Quake 3 modules
//!
//! The [id Tech 3 engine](https://en.wikipedia.org/wiki/Id_Tech_3)
//! can load its game modules (`qagame`, `cgame` and `ui`) both as
//! QVM (Quake Virtual Machine, see [`quake3-qvm` crate](https://crates.io/crates/quake3-qvm)) files and
//! as shared libraries.
//! This crate enables you to write such a native module with Rust code.
//! Take a look at [`native_vm!`](native_vm) to get started.

extern crate lazy_static;
extern crate libc;

// https://github.com/rust-lang/rust/issues/29638#issuecomment-298517765
pub use lazy_static::*;

pub mod ffi {
    pub use libc::intptr_t;
    pub use libc::c_int;
}

pub mod qagame;

/// Engine's syscall function type
///
/// For communication from module to the engine's syscall handler for this module, e.g. `qagame` → `SV_GameSystemCalls`.
///
/// NOTE: The function is not really variadic, the actual number of arguments is an implementation detail.
/// See `VM_DllSyscall` in [ioquake3's `qcommon/vm.c`](https://github.com/ioquake/ioq3/blob/master/code/qcommon/vm.c).
pub type Syscall = extern "C" fn(arg: ffi::intptr_t, ...) -> ffi::intptr_t;

/// Raw FFI interface for shared library modules
///
/// To use an implementation of this, it needs to be wrapped into a shared library with [`native_vm!`](native_vm).
// TODO: Find a better name. It's no VM, just a "module"
pub trait NativeVM: 'static + Sync + Send {
    /// Initialization function
    ///
    /// `syscall` is a generic callback into the engine for this module.
    ///
    /// See `Sys_LoadGameDll` in [ioquake3's `sys/sys_main.c`](https://github.com/ioquake/ioq3/blob/master/code/sys/sys_main.c).
    fn dll_entry(syscall: Syscall) -> Box<Self>
    where
        Self: Sized;

    /// Module dispatcher function
    ///
    /// Engine calls this for module logic, e.g. `GAME_INIT`.
    ///
    /// See `VM_Call` in [ioquake3's `qcommon/vm.c`](https://github.com/ioquake/ioq3/blob/master/code/qcommon/vm.c).
    /// See `vm_s.entryPoint` in [ioquake3's `qcommon/vm_local.h`](https://github.com/ioquake/ioq3/blob/master/code/qcommon/vm_local.h).
    fn vm_main(
        &self,
        command: ffi::c_int,
        arg0: ffi::c_int,
        arg1: ffi::c_int,
        arg2: ffi::c_int,
        arg3: ffi::c_int,
        arg4: ffi::c_int,
        arg5: ffi::c_int,
        arg6: ffi::c_int,
        arg7: ffi::c_int,
        arg8: ffi::c_int,
        arg9: ffi::c_int,
        arg10: ffi::c_int,
        arg11: ffi::c_int,
    ) -> ffi::intptr_t;
}

/// Create required `extern "C" fn`s to load a [`impl NativeVM`](NativeVM) as shared library
///
/// Can only be used once per Rust lib. Each module (`qagame` etc.) needs its own shared library.
///
/// # Examples
///
/// Also see `examples/hello.rs` of this crate.
///
/// Add the following section to your `Cargo.toml`:
///
/// ```toml
/// [lib]
/// name = "q3hi"
/// crate-type = ["cdylib"]
/// ```
///
/// Then implement a module by using the macro as such:
///
/// ```rust
/// // Needed for re-exported lazy_static! macro
/// #![feature(use_extern_macros)]
///
/// #[macro_use]
/// extern crate quake3_native_vm;
///
/// use std::ffi::CString;
/// use quake3_native_vm::*;
///
/// struct HelloQuake3 {
///    syscall: Syscall,
/// }
///
///
/// /// See [ioquake3's `game/g_public.h`](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h)
/// const G_ERROR: ffi::intptr_t = 1;
/// const GAME_INIT: ffi::c_int = 0;
/// const GAME_SHUTDOWN: ffi::c_int = 1;
///
/// impl NativeVM for HelloQuake3 {
///    fn dll_entry(syscall: Syscall) -> Box<HelloQuake3> {
///        Box::new(HelloQuake3 { syscall: syscall })
///    }
///
///    fn vm_main(&self,
///               command: ffi::c_int,
///               arg0: ffi::c_int,
///               arg1: ffi::c_int,
///               arg2: ffi::c_int,
///               arg3: ffi::c_int,
///               arg4: ffi::c_int,
///               arg5: ffi::c_int,
///               arg6: ffi::c_int,
///               arg7: ffi::c_int,
///               arg8: ffi::c_int,
///               arg9: ffi::c_int,
///               arg10: ffi::c_int,
///               arg11: ffi::c_int)
///               -> ffi::intptr_t {
///        match command {
///            GAME_INIT => {
///                let msg = CString::new("Hello, World!").unwrap();
///                (self.syscall)(G_ERROR, msg.as_ptr());
///                unreachable!()
///            }
///            GAME_SHUTDOWN => {
///                // Just return a dummy value here for clean shutdown
///                0
///            },
///            _ => panic!("Game command not implemented"),
///        }
///    }
/// }
///
/// # fn main() {
/// native_vm!(HelloQuake3);
/// # }
/// ```
///
/// Finally build the shared library, put it in the right place for Quake 3 and load it:
///
/// ```sh
/// cargo build
/// mkdir -p ~/.q3a/rust/
/// cp target/debug/libq3hi.so ~/.q3a/rust/qagamex86_64.so
/// ioq3ded +set fs_game rust +set vm_game 0 +map q3dm6
/// ```
///
/// See `Sys_LoadGameDll` in [ioquake3's `sys/sys_main.c`](https://github.com/ioquake/ioq3/blob/master/code/sys/sys_main.c).
#[macro_export]
macro_rules! native_vm {
    ($ty:ident) => {
        use std::sync::{Arc, RwLock};

        $crate::lazy_static! {
            static ref _VM_IMPL: Arc<RwLock<Option<Box<NativeVM>>>> = Arc::new(RwLock::new(None));
        }

        #[doc(hidden)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn dllEntry(syscall: Syscall) {
            let mut VM_IMPL = _VM_IMPL.write().unwrap();
            *VM_IMPL = Some($ty::dll_entry(syscall));
        }

        #[doc(hidden)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn vmMain(command: $crate::ffi::c_int,
                                 arg0: $crate::ffi::c_int,
                                 arg1: $crate::ffi::c_int,
                                 arg2: $crate::ffi::c_int,
                                 arg3: $crate::ffi::c_int,
                                 arg4: $crate::ffi::c_int,
                                 arg5: $crate::ffi::c_int,
                                 arg6: $crate::ffi::c_int,
                                 arg7: $crate::ffi::c_int,
                                 arg8: $crate::ffi::c_int,
                                 arg9: $crate::ffi::c_int,
                                 arg10: $crate::ffi::c_int,
                                 arg11: $crate::ffi::c_int)
                                 -> $crate::ffi::intptr_t {
            let data = _VM_IMPL.read().unwrap();
            data.as_ref().unwrap().vm_main(command,
                               arg0,
                               arg1,
                               arg2,
                               arg3,
                               arg4,
                               arg5,
                               arg6,
                               arg7,
                               arg8,
                               arg9,
                               arg10,
                               arg11)
        }
    }
}
