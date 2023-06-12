#[macro_use]
extern crate rental;

use libloading as lib;
use quake3_native_vm::{
    ffi, native_vm, DllEntry, NativeVM, Syscall, VmMain, DLLENTRY_EXPORT_NAME, VMMAIN_EXPORT_NAME,
};

struct ProxyModule {
    proxy_vm_main: rent_libloading::RentSymbol<VmMain>,
}

rental! {
    mod rent_libloading {
        use libloading;

        #[rental(deref_suffix)]
        pub(crate) struct RentSymbol<S: 'static> {
            lib: Box<libloading::Library>,
            sym: libloading::Symbol<'lib, S>,
        }
    }
}

impl NativeVM for ProxyModule {
    fn dll_entry(syscall: Syscall) -> Box<Self> {
        let lib = unsafe { lib::Library::new("target/debug/examples/hello").unwrap() };
        println!("qagame: {:?}", lib);

        let dll_entry: lib::Symbol<DllEntry> = unsafe { lib.get(DLLENTRY_EXPORT_NAME).unwrap() };
        println!("dllEntry: {:?}", dll_entry);
        dll_entry(syscall);

        match rent_libloading::RentSymbol::try_new(Box::new(lib), |lib| unsafe {
            lib.get(VMMAIN_EXPORT_NAME)
        }) {
            Ok(proxy_vm_main) => return Box::new(Self { proxy_vm_main }),
            Err(e) => panic!("couldn't rent vmMain Symbol: {}", e.0),
        }
    }

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
    ) -> ffi::intptr_t {
        (self.proxy_vm_main)(
            command, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
        )
    }
}

native_vm!(ProxyModule);
