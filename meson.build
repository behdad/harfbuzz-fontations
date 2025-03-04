project('my_harfbuzz_ffi', ['c', 'rust'],
  version: '0.1.0',
  default_options : [
    'buildtype=debugoptimized',
  ],
)

# 1) Find HarfBuzz on the system.
harfbuzz_dep = dependency('harfbuzz', required: true)

# 2) Build the Rust library.
#    - We compile a static_library or shared_library with Meson’s built-in Rust support.
#    - rust_abi = 'c' ensures we produce C-callable symbols (equivalent to `cdylib` or `staticlib` in Cargo).
rust_lib = static_library(
  'my_hb_ffi',               # library name
  'src/lib.rs',              # your Rust source
  dependencies: [harfbuzz_dep],
  rust_abi: 'c',
  install: true,            # install if desired
)

# 3) Install a C header so other code can call your Rust functions.
install_headers('my_harfbuzz_ffi.h',
  subdir : 'my_harfbuzz_ffi'
)

# 4) Optional: build a small test C executable that links with the Rust library + HarfBuzz
executable('demo',
  'main.c',
  link_with: rust_lib,
  dependencies: [harfbuzz_dep],
  install: false
)

