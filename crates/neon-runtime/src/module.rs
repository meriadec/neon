//! A helper function for initializing a module.

use std::os::raw::c_void;
use raw::Local;

extern "C" {

    /// Creates a new `v8::HandleScope` and calls `callback` provided with the argument signature
    /// `(kernal, exports, scope)`.
    #[link_name = "Neon_Module_ExecKernel"]
    pub fn exec_kernel(kernel: *mut c_void, callback: extern fn(*mut c_void, *mut c_void, *mut c_void), exports: Local, scope: *mut c_void);

    #[link_name = "Neon_Module_GetVersion"]
    pub fn get_version() -> i32;

}
