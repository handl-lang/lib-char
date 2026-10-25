# Explanation

## Why this crate exists

A string library written against `std::char` is welded to Rust. Writing it
against an axiom set makes the *semantics* a choice of backend: ASCII-only and
full-Unicode are both legitimate, they are simply different `char`
implementations. The contract says nothing about encoding (UTF-8/16/32) or about
Unicode vs ASCII vs locale on purpose — that is the backend's decision to
document.

## Where it sits

This crate belongs to the `engine-*` family. Plugins, kits and hosts reach it
through the kit's operation slot, so the engine can be swapped without touching a
language definition.

See the [reference](/lib-char/reference/) for the API and the
[how-to guides](/lib-char/how-to/) for tasks.
