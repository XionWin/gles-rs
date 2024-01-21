use libc::{c_char, c_uint, c_float, c_int};

use crate::{def::StringName, ffi, GfxProgram};
use std::{str::from_utf8, ffi::{CStr, CString}};

pub fn get_string(name: StringName) -> Option<String> {
    unsafe {
        let c_str = ffi::glGetString(name as _);

        if !c_str.is_null() {
            match from_utf8(CStr::from_ptr(c_str).to_bytes()) {
                Ok(s) => Some(s.to_string()),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub fn get_uniform_location(program: &GfxProgram, name: &str) -> c_int {
    unsafe {
        let c_str = CString::new(name).unwrap();
        ffi::glGetUniformLocation(program.id, c_str.as_ptr() as *const c_char)
    }
}

pub fn uniform2f(program: &GfxProgram, name: &str, x: c_float, y: c_float) {
    unsafe { ffi::glUniform2f(get_uniform_location(program, name), x, y) }
}

pub fn viewport(x: c_int, y: c_int, width: c_int, height: c_int) {
    unsafe { ffi::glViewport(x, y, width, height) }
}

pub fn clear(mask: c_uint) {
    unsafe { ffi::glClear(mask) }
}

pub fn clear_color(red: c_float, green: c_float, blue: c_float, alpha: c_float) {
    unsafe { ffi::glClearColor(red, green, blue, alpha) }
}