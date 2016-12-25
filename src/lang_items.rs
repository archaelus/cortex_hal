// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/*! Rust Language items (panic fmt / exception handling personality)
 */

use core;

/// These functions are used by the compiler, but not
/// for a bare-bones hello world. These are normally
/// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

/// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

/// Panics end up here.
#[lang = "panic_fmt"]
#[no_mangle]
pub unsafe extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
                                          _file: &'static str,
                                          _line: u32) -> ! {
    loop {}
}
