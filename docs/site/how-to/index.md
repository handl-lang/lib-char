# How-to guides

Task recipes. Each assumes sibling checkouts of this repo and of the platform
repos it patches.

## Write a backend

Implement the contract over your encoding and document which reading of `is_whitespace`/`is_alpha` you mean; do not special-case callers.

## Read an algorithm on top

Assume only what the axioms state — char indices, 1-char strings, semantic predicate names — so the same `str-handl` code runs on every backend.
