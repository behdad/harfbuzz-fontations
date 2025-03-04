#include <stdio.h>
#include <hb.h>
#include "my_harfbuzz_ffi.h"

int main(void)
{
    // 1) Create your custom font funcs
    hb_font_funcs_t *funcs = my_create_hb_font_funcs();
    if (!funcs) {
        fprintf(stderr, "Failed to create custom hb_font_funcs!\n");
        return 1;
    }

    // 2) Create an hb_face / hb_font from some TTF (for example)
    //    For brevity, just an example:
    hb_blob_t* blob = hb_blob_create_from_file("Roboto-Regular.ttf");
    hb_face_t* face = hb_face_create(blob, 0);
    hb_font_t* font = hb_font_create(face);

    // 3) Attach your custom callbacks to the hb_font
    my_attach_funcs_to_font(font, funcs);

    // 4) Now do shaping or testing...
    hb_codepoint_t example_glyph = 42;
    hb_position_t adv = hb_font_get_glyph_h_advance(font, example_glyph);
    printf("Custom H advance for glyph %u => %d\n", example_glyph, adv);

    // Cleanup
    hb_font_destroy(font);
    hb_face_destroy(face);
    hb_blob_destroy(blob);
    // funcs will be freed automatically if it’s properly refcounted, but you can
    // also unref if needed:
    hb_font_funcs_destroy(funcs);

    return 0;
}

