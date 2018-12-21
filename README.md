# aether-primitives - a software radio framework powered by rust
[![Latest Version](https://img.shields.io/crates/v/aether_primitives.svg)](https://crates.io/crates/aether_primitives)
[![Documentation](https://docs.rs/aether_primitives/badge.svg)](https://docs.rs/crate/aether_primitives)
![License](https://img.shields.io/crates/l/aether_primitives.svg)
## What is aether?
Aether is designed to ease development of SDR applications by providing convenient (low-level) building blocks for common operations.  

## Examples
Core operations are implemented in the form of the VecOps trait implemented for Vecs/Slices of the C compatible [num::Complex<f32>](https://docs.rs/num-complex/latest/num_complex/type.Complex32.html) (```cf32``` for short).  

```rust
// #[macro_use] // includes the assert_evm macro
// extern crate aether_primitives;
// use aether_primitives::{cf32, vecops::VecOps};
// The main sample type is cf32 which is a type alias for num::Complex<f32>
let mut v = vec![cf32::new(2.0, 2.0); 100];
let twos = v.clone();
let ones = vec![cf32::new(1.0, 1.0); 100];

let correct = vec![cf32::new(1.0, -1.0); 100];

v.vec_div(&twos)
    .vec_mul(&twos)
    .vec_zero() // zero the vector
    .vec_add(&ones)
    .vec_sub(&twos)
    .vec_clone(&ones)
    .vec_mutate(|c| c.im = -1.0) 
    .vec_conj()
    .vec_mirror(); // mirror swaps elements around the midpoint of the array

/// ensure each element's error vector magnitude vs the correct vector is below -80dB
assert_evm!(&v, &correct, -80.0); 
```

## Design Decisions
* The base versions will be written in idiomatic rust  
* Optimisations and unsafe speedups will be hidden behind feature flags  
* The actual version of the num-traits and num-complex crates are not pinned by aether because multiple concurrent versions of the same trait are incompatible.  
This can cause type level incompatibility if there are dependencies which expose different versions of the same type to the user.
Hence the version is not pinned as cargo will usually try to build the same version of num-complex and num-traits for the biggest set of dependencies (within their version constraints), thus reducing the probability of this happening.
* For performance reasons the use of dynamic dispatch (Trait objects ```dyn <Trait>```) will be avoided.

## Implemented functionality
- Macros:
    - assert_evm!: check if elements of both vectors have a certain error vector magnitude relative to each other (given in dBm)
- Vecops: Helpers for operations of vectors/slices of cf32
    - Element wise operations: add, subtract, divide, multiply, complex conjugate, mutate
    . Mirror: Swap elements around mid of vector (for even length vectros)
    - Zero entire vector, copy elements over from another vector
    - FEATURE: Perform (i)FFTs using new or existing fourier transform instance (enabled via ```fft_chfft```)
- Sequence: Helpers for binary pseudo-random sequence generation (M-Sequences)
    - expand: Expand a seed value into an initialisation vector for a Pseudo-random sequence
    - generate: Generate a pseudo random sequence
- Sampling
    - linear interpolation
    - even downsampling
- FFT: FEATURE ```fft_chfft```
    - perform fast fourier transforms (forward/backward) on slices/vecs of cf32 with different scaling factors
    - Supported fft implementations: [chfft](https://github.com/chalharu/chfft)
- File
    - binary file writing and reading for arbitrary structs
    - csv file writing and reading for arbitrary structs
- Channel
    - Noise generation
- Plot: FEATURE ```plot```; requires an installed version of ```gnuplot```
    - Constellation diagram
    - Time sequence plot
    - Comparison plot of two sequences
    - Waterfall plot with a given fft size (requires ```fft_chfft```)
- Utils
    - Conversion from and to dB

- Benches: benchmarks for most operations in aether using the criterion.rs framework
    - downsampling, interpolation, fft

## TODO
- [ ] Pull out choice of FFT ([RustFFT](https://github.com/awelkie/RustFFT) vs [chfft](https://github.com/chalharu/chfft)) via wrappers
    - [x] Implement wrapper for chfft
    - [ ] Implement wrapper for RustFFT
         - Issue: cf32 incompatible with RustFFTs version of cf32 (maybe add some shady casts since the structs are the same)
- [ ] Add vec_align! macro to create vecs aligned for SIMD instructions
- [ ] Ungrowable Vecs
    - maybe derefs to slice for convenience
- [ ] Add VecStats (f32,cf32)
    - Min(index),Max(index),Mean(index),Power
- [ ] Add VecOps Features
    - [x] FFTs : vec_fft,vec_ifft, vec_rifft, vec_rfft, vec_rifft; rifft/rfft reuse an existing instance of fft::Cfft currently supported by building with ```fft_chfft``` enabled.  
    - [ ] Feature: use [faster](https://github.com/AdamNiederer/faster) once it works on stable again
    - [ ] Feature: use [VOLK](https://libvolk.org) for ops
        - Add tests to ensure generated code is correctly aligned - should be ensured since cf32 (2x4 bytes) is 8 bytes. VOLK [prefers](https://libvolk.org/doxygen/concepts_terms_and_techniques.html) 32byte alignment /libfftw [prefers](http://www.fftw.org/fftw3_doc/SIMD-alignment-and-fftw_005fmalloc.html) 16 byte alignment
        - must also hook into the vec_align macro
- [ ] Add Correlation by Freq. Domain Convolution
- [ ] Add FIR

## License
[Mozilla Public License 2.0](LICENSE)