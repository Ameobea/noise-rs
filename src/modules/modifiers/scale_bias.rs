// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that applies a scaling factor and a bias to the output value
/// from the source module.
///
/// The module retrieves the output value from the source module, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseModule<T, U>,

    /// Scaling factor to apply to the output value from the source module.
    /// The default value is 1.0.
    pub scale: U,

    /// Bias to apply to the scaled output value from the source module.
    /// The default value is 0.0.
    pub bias: U,
}

impl<'a, T, U> ScaleBias<'a, T, U>
    where U: Float,
{
    pub fn new(source: &'a NoiseModule<T, U>) -> ScaleBias<'a, T, U> {
        ScaleBias {
            source: source,
            scale: U::one(),
            bias: U::zero(),
        }
    }

    pub fn set_scale(self, scale: U) -> ScaleBias<'a, T, U> {
        ScaleBias { scale: scale, ..self }
    }

    pub fn set_bias(self, bias: U) -> ScaleBias<'a, T, U> {
        ScaleBias { bias: bias, ..self }
    }
}

impl<'a, T, U> NoiseModule<T, U> for ScaleBias<'a, T, U>
    where U: Float,
{
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: T) -> U {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: T) -> U {
        (self.source.get(point) * self.scale) + self.bias
    }
}
