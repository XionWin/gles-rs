use libc::{c_char, c_float, c_int, c_uint};

use crate::{def::StringName, ffi, GfxProgram};
use std::{
    ffi::{CStr, CString},
    str::from_utf8,
};

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

pub fn gen_vertex_arrays(n: c_int, array: *mut c_uint) {
    unsafe {
        crate::ffi::glGenVertexArrays(n, array);
    }
}

pub fn bind_vertex_array(array_id: c_uint) {
    unsafe {
        crate::ffi::glBindVertexArray(array_id);
    }
}

pub fn gen_buffers(n: c_int) -> Vec<c_uint> {
    let mut buffer = std::vec::from_elem(0, n as _);
    unsafe {
        crate::ffi::glGenBuffers(n, buffer.as_mut_ptr());
    }
    buffer
}

pub fn bind_buffer(target: crate::def::BufferTarget, buffer_id: c_uint) {
    unsafe {
        crate::ffi::glBindBuffer(target as _, buffer_id);
    }
}

pub fn buffer_data<T>(
    target: crate::def::BufferTarget,
    data: &[T],
    usage: crate::def::BufferUsageHint,
) {
    unsafe {
        crate::ffi::glBufferData(
            target as _,
            (data.len() * std::mem::size_of::<T>()) as ffi::GLsizeiptr,
            data.as_ptr() as *const ffi::GLvoid,
            usage as _,
        );
    }
}

pub fn get_attrib_location(program_id: c_uint, name: &str) -> c_uint {
    let mut buffer = name.bytes().collect::<Vec<u8>>();
    buffer.push(b'\0');
    match unsafe { crate::ffi::glGetAttribLocation(program_id, buffer.as_ptr()) } {
        value if value >= 0 => value as c_uint,
        _ => panic!("GLES get_attrib_location error"),
    }
}

pub fn vertex_attrib_pointer_f32(
    index: c_uint,
    size: c_int,
    normalized: bool,
    stride: c_int,
    offset: c_uint,
) {
    unsafe {
        crate::ffi::glVertexAttribPointer(
            index,
            size,
            crate::ffi::GL_FLOAT,
            normalized as _,
            stride,
            offset as _,
        )
    }
}

pub fn enable_vertex_attrib_array(index: c_uint) {
    unsafe {
        crate::ffi::glEnableVertexAttribArray(index);
    }
}

pub fn draw_arrays(begin_mode: crate::def::BeginMode, first: c_int, count: c_int) {
    unsafe {
        crate::ffi::glDrawArrays(begin_mode as _, first, count);
    }
}
