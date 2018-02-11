//! Native Rust VMs for Quake 3
//!
//! The [id Tech 3 engine](https://en.wikipedia.org/wiki/Id_Tech_3)
//! can load its game modules (`qagame`, `cgame` and `ui`) both as
//! QVM (Quake Virtual Machine, see [`quake3-qvm` crate](https://crates.io/crates/quake3-qvm)) files and
//! as shared libraries.
//! This crate enables you to write such a native module with Rust code.

//#![feature(use_extern_macros)]

pub extern crate libc;
pub extern crate lazy_static;

// https://github.com/rust-lang/rust/issues/29638#issuecomment-298517765
pub use lazy_static::*;

/// Engine's syscall function type.
///
/// For communication from VM to the engine's syscall handler for this module, e.g. `qagame` → `SV_GameSystemCalls`.
///
/// NOTE: The function is not really variadic, the actual number of arguments is an implementation detail.
/// See `VM_DllSyscall` in ioquake3's [qcommon/vm.c](https://github.com/ioquake/ioq3/blob/master/code/qcommon/vm.c).
pub type Syscall = extern "C" fn(arg: libc::intptr_t, ...) -> libc::intptr_t;

/// Interface for native QVM implementations.
// TODO: Find a better name than `QVM`
pub trait QVM: 'static + Sync + Send {
    /// Initialization function.
    ///
    /// `syscall` is a callback into the engine.
    fn dll_entry(syscall: Syscall) -> Box<Self> where Self: Sized;

    /// QVM dispatcher function.
    ///
    /// Engine calls this for game logic.
    fn vm_main(&self,
               command: libc::c_int,
               arg0: libc::c_int,
               arg1: libc::c_int,
               arg2: libc::c_int,
               arg3: libc::c_int,
               arg4: libc::c_int,
               arg5: libc::c_int,
               arg6: libc::c_int,
               arg7: libc::c_int,
               arg8: libc::c_int,
               arg9: libc::c_int,
               arg10: libc::c_int,
               arg11: libc::c_int)
               -> libc::intptr_t;
}

/// Creates the required plumbing to use an `impl QVM` as a native shared library.
///
/// # Examples
///
/// Add the following section to your `Cargo.toml`:
///
/// ```toml
/// [lib]
/// name = "q3hi"
/// crate-type = ["cdylib"]
/// ```
///
/// Then implement a QVM by using the macro as such:
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
/// /// See ioquake3's [game/g_public.h](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h)
/// const G_ERROR: libc::intptr_t = 1;
/// const GAME_INIT: libc::c_int = 0;
/// const GAME_SHUTDOWN: libc::c_int = 1;
///
/// impl QVM for HelloQuake3 {
///    fn dll_entry(syscall: Syscall) -> Box<HelloQuake3> {
///        Box::new(HelloQuake3 { syscall: syscall })
///    }
///
///    fn vm_main(&self,
///               command: libc::c_int,
///               arg0: libc::c_int,
///               arg1: libc::c_int,
///               arg2: libc::c_int,
///               arg3: libc::c_int,
///               arg4: libc::c_int,
///               arg5: libc::c_int,
///               arg6: libc::c_int,
///               arg7: libc::c_int,
///               arg8: libc::c_int,
///               arg9: libc::c_int,
///               arg10: libc::c_int,
///               arg11: libc::c_int)
///               -> libc::intptr_t {
///        match command {
///            GAME_INIT => {
///                (self.syscall)(G_ERROR, CString::new("Hello, World!").unwrap().as_ptr());
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
/// native_qvm!(HelloQuake3);
/// # }
/// ```
///
/// Finally build the QVM, put it in the right place for Quake 3 and load it:
///
/// ```sh
/// cargo build
/// mkdir -p ~/.q3a/rust/
/// cp target/debug/libq3hi.so ~/.q3a/rust/qagamex86_64.so
/// ioq3ded +set fs_game rust +set vm_game 0 +map q3dm6
/// ```
#[macro_export]
macro_rules! native_qvm {
    ($ty:ident) => {
        $crate::lazy_static! {
            static ref _QVM_IMPL: std::sync::Arc<std::sync::RwLock<Option<Box<QVM>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
        }

        #[doc(hidden)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn dllEntry(syscall: Syscall) {
            let mut QVM_IMPL = _QVM_IMPL.write().unwrap();
            *QVM_IMPL = Some($ty::dll_entry(syscall));
        }

        #[doc(hidden)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn vmMain(command: $crate::libc::c_int,
                                 arg0: $crate::libc::c_int,
                                 arg1: $crate::libc::c_int,
                                 arg2: $crate::libc::c_int,
                                 arg3: $crate::libc::c_int,
                                 arg4: $crate::libc::c_int,
                                 arg5: $crate::libc::c_int,
                                 arg6: $crate::libc::c_int,
                                 arg7: $crate::libc::c_int,
                                 arg8: $crate::libc::c_int,
                                 arg9: $crate::libc::c_int,
                                 arg10: $crate::libc::c_int,
                                 arg11: $crate::libc::c_int)
                                 -> $crate::libc::intptr_t {
            let data = _QVM_IMPL.read().unwrap();
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
