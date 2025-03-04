// src/lib.rs

use std::os::raw::{c_char, c_void};
use std::ptr::{null_mut};
use std::ffi::CStr;

// HarfBuzz type stubs.
// In a real project, you'd generate these via bindgen from hb.h.
pub type hb_bool_t = i32;
pub type hb_codepoint_t = u32;
pub type hb_position_t = i32;
pub type hb_destroy_func_t = Option<extern "C" fn(user_data: *mut c_void)>;

#[repr(C)]
pub struct hb_font_t { _unused: [u8; 0], }
#[repr(C)]
pub struct hb_font_funcs_t { _unused: [u8; 0], }

// Callback signature (example: horizontal advance)
pub type hb_font_get_glyph_h_advance_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    glyph: hb_codepoint_t,
    user_data: *mut c_void
) -> hb_position_t;

// Some HarfBuzz C functions (in reality you'd import many more).
extern "C" {
    fn hb_font_funcs_create() -> *mut hb_font_funcs_t;
    fn hb_font_funcs_set_glyph_h_advance_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_h_advance_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t
    );
    fn hb_font_set_funcs(
        font: *mut hb_font_t,
        klass: *mut hb_font_funcs_t,
        font_data: *mut c_void,
        destroy: hb_destroy_func_t
    );
}

// A small struct to hold your custom data. Could parse a real font, etc.
#[repr(C)]
pub struct MyFontData {
    pub scale: i32,
}

#[no_mangle]
pub extern "C" fn my_font_data_destroy(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr as *mut MyFontData); }
    }
}

// This is the callback called by HarfBuzz to get the horizontal advance.
#[no_mangle]
pub extern "C" fn my_get_glyph_h_advance(
    _font: *mut hb_font_t,
    font_data: *mut c_void,
    glyph: hb_codepoint_t,
    _user_data: *mut c_void
) -> hb_position_t {
    assert!(!font_data.is_null());
    let mydata = unsafe {
        // Convert the raw pointer back to a reference
        &*(font_data as *mut MyFontData)
    };
    // For demonstration, just do `glyph + scale`
    let advance = glyph as i32 + mydata.scale;
    advance
}

// Create an hb_font_funcs_t with our custom callback
#[no_mangle]
pub extern "C" fn my_create_hb_font_funcs() -> *mut hb_font_funcs_t {
    let ffuncs = unsafe { hb_font_funcs_create() };
    if ffuncs.is_null() {
        return null_mut();
    }


    unsafe {
        hb_font_funcs_set_glyph_h_advance_func(
            ffuncs,
            my_get_glyph_h_advance,
            null_mut(),
            Some(my_font_data_destroy),
        );
    }
    ffuncs
}

// Helper to attach these funcs to a real hb_font_t
#[no_mangle]
pub extern "C" fn my_attach_funcs_to_font(
    font: *mut hb_font_t,
    ffuncs: *mut hb_font_funcs_t
) {
    if font.is_null() || ffuncs.is_null() {
        return;
    }
    // Example: put some scale in the user data
    let data = MyFontData { scale: 100 };
    let data_ptr = Box::into_raw(Box::new(data)) as *mut c_void;
    assert!(!data_ptr.is_null());
    unsafe {
        // Pass null user_data because we store everything inside the ffuncs
        hb_font_set_funcs(font, ffuncs, data_ptr, None);
    }
}
