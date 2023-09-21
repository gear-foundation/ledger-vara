use core::{ffi::c_void, ptr};

#[no_mangle]
extern "C" fn __memmove_chk(dest: *mut c_void, src: *const c_void, len: usize, destlen: usize) {
    assert!(len <= destlen);
    unsafe { ptr::copy_nonoverlapping(src, dest, len) };
}

#[no_mangle]
extern "C" fn __memcpy_chk(dest: *mut c_void, src: *const c_void, len: usize, destlen: usize) {
    assert!(len <= destlen);
    unsafe { ptr::copy_nonoverlapping(src, dest, len) };
}
