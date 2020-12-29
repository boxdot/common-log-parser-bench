#[macro_use]
extern crate lazy_static;

pub mod nom;
pub mod regex;

#[allow(dead_code)]
pub struct Labels<'t> {
    ip: &'t str,
    user: &'t str,
    frank: &'t str,
    date_time: &'t str,
    request: &'t str,
    response_code: u16,
    size: u32,
}

#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();

    std::mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub extern "C" fn nom_parse(input_ptr: *mut u8, input_len: usize) -> usize {
    let data = unsafe {
        let ptr = input_ptr as *const u8;
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, input_len))
    };
    let p = nom::CommonLogParser { input: &data };
    p.count()
}
