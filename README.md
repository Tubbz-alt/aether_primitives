# aether-primitives - software radio primitives

### TODO
- [ ] Pull out choice of FFT ([RustFFT](https://github.com/awelkie/RustFFT), [chfft](https://github.com/chalharu/chfft))
- [x] Add evm_assert!
- [ ] Add Fixed-size cf32 Vecs
    - maybe derefs to slice for convenience
- [ ] Add VecStats
- [ ] Add VecOps
    - vec_scale, vec_div, vec_sub, vec_conj, vec_add, vec_mul, vec_clone, vec_mirror, vec_zero, vec_mutate
    - Feature: use [VOLK](https://libvolk.org) for ops
        - Add tests to ensure generated code is correctly aligned - should be ensured since cf32 (2x4 bytes) is 8 bytes. VOLK [prefers](https://libvolk.org/doxygen/concepts_terms_and_techniques.html) 32byte alignment /libfftw [prefers](http://www.fftw.org/fftw3_doc/SIMD-alignment-and-fftw_005fmalloc.html) 16 byte alignment
    - Feature: use [faster](https://github.com/AdamNiederer/faster) 
    - Optional: vec_norm, vec_fft,vec_ifft, vec_rifft, vec_rfft, vec_rifft
- [ ] Add Correlation by Freq. Domain Convolution
- [ ] Add FIR
- [ ] Add FFT benches

