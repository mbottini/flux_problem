# Flux Problem

## Mike Bottini

# Building

```bash
cargo build
```

If you haven't installed the Rust compiler, install Rustup from
[rustup.rs](https://rustup.rs/) and then do

```bash
rustup default stable
```

Currently, this is a library and does not build an executable. Later on, an
executable will use this library to run a simulation with provided parameters.

# Running Unit Tests

```bash
cargo test
```

# Generating Documentation

```bash
cargo doc
```

# Background

This is an attempt to De-Academicize some MATLAB code by my brother. Modern
programming is pretty aggressive about using type-driven development and other
nice abstractions to organize programs. Unfortunately, physics academia appears
to know none of these techniques, and this runs into problems when you have to
teach undergrads how to do complicated computations.

Examples of better organization techniques:

* The use of structs to organize what is typically represented as a Big Array Of
Numbers into something with a little more semantic meaning.
* The use of functions to organize what is typically written as a Big Nasty
While Loop.
* The use of function, struct, and module-level doc comments in favor of the
typical inline comments, which are often noisy (but are still better than
nothing).
* TODO: The use of the iterator algebra to perform pairwise comparison to find
the fixed point of the calculation. You know, the actual computation.

After doing this, we can look at how to translate some of these concepts back
into MATLAB. Failing that, we can explore other languages. I've heard very nice
things about Julia, but I also know a lot of Python.

# Architecture

As with most Rust projects, the actual "entry point" of the library is [`lib.rs`](./src/lib.rs),
which is just a bunch of declarations for the various other modules in the library.

[`material.rs`](./src/material.rs) is a replacement for `Initialize_Material.m`.

[`piece_materials.rs`](./src/piece_materials.rs) is a replacement for
`Initialize_Grid.m`. The original implementation creates a linspace of X and Y
values and then assigns an index from a vector of materials. Instead, we can
name these materials.

[`geometry.rs`](./src/geometry.rs) is a replacement for `Initialize_Geometry.m`.
It also provides a method for determining which `Material` is in any particular
pair of coordinates.

[`phi_state.rs`](./src/phi_state.rs) (TODO!!!) is going to be the meat of the
program. It defines a 2D array of values, along with a `Geometry` field, and
will contain methods for producing the next `tick` in a function. We're then
going to take advantage of Rust's iterator algebra to produce a certain number
of ticks and test each pair of ticks for convergence.