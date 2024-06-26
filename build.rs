/* 
Copyright (c) 2024  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/nswnd

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::path::PathBuf;
use nscfg::{ match_cfg, target_cfg };

const CBIND_FOLDER : &str = "dep/c";

fn main() {

    match_cfg! {
        linux => {
            // Create X11 C-bindings
            create_x11_bindings();

        },
        _ => {
            panic!("OS not supported!");
        }

    }

}

target_cfg! {
    linux => {
        /// Create X11 c-binding for Rust
        /// 
        /// TODO:Search for file path instead of absolute path.
        fn create_x11_bindings() {
            // Write the bindings to the $OUT_DIR/x11.rs file.
            let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("x11.rs");

            // Tell cargo to link to X11 library
            println!("cargo:rustc-link-search=/usr/include/X11");
            println!("cargo:rustc-link-lib=X11");

            let binding = bindgen::Builder::default()
                .clang_args(&["-I/usr/include/X11"])    // Include path for Clang
                .header(format!("{}/x11.h", CBIND_FOLDER))
                .layout_tests(false)                    // Deactivate tests
                .disable_nested_struct_naming()         // Deactivate struct renaming
                .c_naming(true)                         // Preserve c names
                .prepend_enum_name(false)               // Do not add type to enum name
                .disable_name_namespacing()             // Do not add namespace to name
                .generate_comments(false)               // Deactivate comments
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))  // Invalidate on any changes
                .generate()
                .expect("Unable to generate X11 bindings!");

            binding.write_to_file(out_path).expect("Unable to write X11 bindings!");
        }
    }
}
