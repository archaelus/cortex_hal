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

//! Helper functions for memory initialisation.

extern {
    static _data_load: usize; // load address for .data
    static _data: usize; // flash start address for .data
    static _edata: usize; // flash end address for .data
    static _bss: usize; // .bss start address
    static _ebss: usize; // .bss end address
}

/// Helper function to initialize memory.
/// Copies `.data` sections in to RAM and initializes `.bss` sections to zero.
#[inline(always)]
pub fn init() {
    use core::intrinsics::volatile_copy_memory;
    // Copy .data from flash (_data.._edata) to ram (_data_load..)
    unsafe { volatile_copy_memory::<usize>(_data_load as *mut usize,
                                           _data as *const usize,
                                           _edata - _data); }
    use core::intrinsics::volatile_set_memory;
    // Zero the .bss section (_bss.._ebss)
    unsafe { volatile_set_memory::<usize>(_bss as *mut usize,
                                          0,
                                          _ebss - _bss); }
}
