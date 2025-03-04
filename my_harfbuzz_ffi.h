#ifndef MY_HARFBUZZ_FFI_H
#define MY_HARFBUZZ_FFI_H

#include <hb.h> // For hb_font_t, etc. Make sure the include path is set up

#ifdef __cplusplus
extern "C" {
#endif

// These functions are exported by our Rust library:
hb_font_funcs_t *my_create_hb_font_funcs(void);
void my_attach_funcs_to_font(hb_font_t *font, hb_font_funcs_t *funcs);

#ifdef __cplusplus
} // extern "C"
#endif

#endif

