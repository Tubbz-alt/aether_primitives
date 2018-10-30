extern crate num_complex;

/// Shorthand for Complex<f32>
/// Default sample type

#[allow(non_camel_case_types)]
pub type cf32 = num_complex::Complex32;

/// Error Vector Magnitude assertion
/// Checks each element and panics if an element in the ```actual```
/// EVM = 10 log (P_error/P_ref) => Error vector in relation to the actually expected signal in dB.
/// The error vector is defined as the vector between the reference symbol and the actually received signal.
/// We achieve this by computing the norm of (actual-ref)
#[macro_export]
macro_rules! assert_evm {
    ($actual:expr, $ref:expr, $evm_limit_db:expr) => {
        for (idx, (act, re)) in $actual.iter().zip($ref).enumerate() {
            let evm = (act - re).norm();
            let limit = re.norm() * ($evm_limit_db / 10f32).powi(10);

            if evm > limit {
                let evm_db = evm.log10()*10f32;
                panic!(
                    "EVM limit exceeded: Got {}({}dB) > limit {}({}dB) for element {}. Actual {}, Expected {}",
                    evm,evm_db,limit, $evm_limit_db, idx, act, re
                );
            }
        }
    };
}

pub mod vecops;

#[cfg(test)]
mod test {
    use super::cf32;
    
    #[test]
    fn evm_correct() {
        let refr = vec![
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
        ];
        let act = vec![
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(0.9f32, 0f32),
        ];
        // error should be <= 0.1
        assert_evm!(act, refr, (-10.0));
    }

    #[test]
    #[should_panic]
    fn evm_fail() {
        let refr = vec![
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
        ];
        let act = vec![
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(1f32, 0f32),
            cf32::new(0.9f32, 0f32),
        ];
        // error should be <= 0.0
        assert_evm!(act, refr, (0.0));
    }

}
