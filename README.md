
# Alea
Rust port of Johannes Baagøe's Alea PRNG.
[![Crates.io](https://img.shields.io/crates/v/alea-js?style=flat-square)](https://crates.io/crates/alea-js)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)


This project contains three functions:

**Alea**: a rust port of Alea, guaranteed to output the exact values as the original 0.9 version of javascript Alea

**Mash**: Alea's string hasher

**AleaFast**: a rust port of Alea optimized for performance, however it may differ from the original on extreme seeds or heavy rng rolls
