// Cortex HAL from Zinc.
// Portions copyright:
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

#![feature(const_fn)]
#![feature(asm)]
#![deny(missing_docs)]
#![no_std]
#![feature(plugin)]
#![feature(core_intrinsics)]
#![plugin(ioreg)]

/*! Common peripheral definitions for all ARM Cortex M* family members
*/

#[macro_use] extern crate volatile_cell;

pub mod mem;
pub mod isr;

pub mod nvic;
pub mod scb;

pub mod systick;
pub mod mpu;
pub mod irq;

