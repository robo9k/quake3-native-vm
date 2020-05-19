use libloading as lib;
use quake3_native_vm::{
    ffi, native_vm, DllEntry, NativeVM, Syscall, VmMain, DLLENTRY_EXPORT_NAME, VMMAIN_EXPORT_NAME,
};

struct ProxyModule {
    proxy_lib: lib::Library,
}

impl NativeVM for ProxyModule {
    fn dll_entry(syscall: Syscall) -> Box<Self> {
        let lib = lib::Library::new("target/debug/examples/hello").unwrap();
        println!("qagame: {:?}", lib);

        let dll_entry: lib::Symbol<DllEntry> = unsafe { lib.get(DLLENTRY_EXPORT_NAME).unwrap() };
        println!("dllEntry: {:?}", dll_entry);
        dll_entry(syscall);

        Box::new(Self { proxy_lib: lib })
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
        let vm_main: lib::Symbol<VmMain> =
            unsafe { self.proxy_lib.get(VMMAIN_EXPORT_NAME).unwrap() };
        println!("vmMain: {:?}", vm_main);
        vm_main(
            command, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
        )
    }
}

native_vm!(ProxyModule);
