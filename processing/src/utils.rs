use std::{ffi::CStr, os::raw::c_char};

pub unsafe fn sanitize_c_str(string: *const c_char) -> String {
  let c_str = CStr::from_ptr(string);
  let str_slice = c_str.to_str().unwrap();
  String::from(str_slice)
}

pub fn to_c_str(string: &str) -> *const c_char {
  let c_str = std::ffi::CString::new(string).unwrap();
  let ptr = c_str.as_ptr();
  std::mem::forget(c_str);
  ptr
}