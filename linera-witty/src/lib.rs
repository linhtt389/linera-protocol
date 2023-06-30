// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Linera Witty
//!
//! This crate allows generating [WIT] files and host side code to interface with WebAssembly guests
//! that adhere to the [WIT] interface format. The source of truth for the generated code and WIT
//! files is the Rust source code.
//!
//! [WIT]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

mod primitive_types;