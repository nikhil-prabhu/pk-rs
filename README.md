# pk-rs

![maintenance-status](https://img.shields.io/badge/maintenance-experimental-blue.svg)
[![crates-badge](https://img.shields.io/crates/v/cloud-detect.svg)](https://crates.io/crates/cloud-detect)
[![License: GPL v3](https://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

High-level Rust wrappers/bindings for PackageKit

## About

This crate aims to provide a high-level Rust wrapper over the [libpackagekit-glib2](https://www.freedesktop.org/software/PackageKit/gtk-doc/lpackagekit-glib2.html) C library.
It does not provide one-to-one bindings, and is just meant to be an easy to use high-level abstraction over the PackageKit API.

**NOTE**: This project is currently highly experimental and is likely to be very unstable/broken for the foreseeable future.
For now, it's just an experiment in working with foreign function interfaces (FFIs) in Rust. However, it _may_ be viable for simple use-cases later on in development.
