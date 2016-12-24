# Cortex M(4) Hardware (Awesome) Library

This library provides a (wafer) thin layer over basic Cortex-M (targetting 4) functionality. Enough to initialize Vectors/Memory and little else.

This code is derived from [Zinc.rs](https://github.com/hackndev/zinc) - many thanks to them.

## Setup

Generally you'll include this in your own project 
```toml
[dependencies.cortex_hal]
git = "https://github.com/archaelus/cortex_hal"
```

You can then call:
```rust
extern crate cortex_hal;

cortex_hal::mem::init();
```
from your startup routine.

## Building

If you want to build this library as a standalone rlib:
```
$ xargo build
```

