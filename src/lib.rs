pub trait RoundTiesToPosInf {
    fn round_ties_to_positive_infinity(self) -> Self;
}

macro_rules! rt2pi_impl {
    ($t: ty) => {
        impl RoundTiesToPosInf for $t {
            // http://stackoverflow.com/a/28124775/155423
            fn round_ties_to_positive_infinity(self) -> Self {
                let x = self;
                let y = x.floor();
                if x == y {
                    x
                } else {
                    let z = (2.0 * x - y).floor();
                    z.abs() * x.signum() // Should use copysign, but not stably-available
                }
            }
        }
    };
}

rt2pi_impl!(f32);
rt2pi_impl!(f64);
