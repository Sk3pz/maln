
// σ Σ
/// Takes a number and scales it closer to zero for the lower it is, and
/// closer to one for the higher it is.
/// Returns a number between 0 and 1.
/// The returned value will be NAN (Not a Number) if the input is too high or low.
pub fn sigmoid<F: Into<f64>>(x: F) -> f64 {
    use std::f64::consts::E;
    let x_into = x.into();

    // there are two ways to implement this

    // the way it is normally expressed
    1.0 / (1.0 + E.powf(-x_into))

    // another way to write it without negative number
    //(E.powf(x_into.clone())) / (E.powf(x_into) + 1.0)
}

/// A Neuron stores it's activation number
#[derive(Clone, Debug)]
pub struct Neuron {
    pub activation: f64,
}

impl Neuron {
    /// Creates a new neuron from a value
    pub fn new(x: f64) -> Self {
        Self {
            activation: x
        }
    }

    /// Creates a new neuron from Σ(x)
    pub fn new_sigmoid(x: f64) -> Self {
        Self {
            activation: sigmoid(x)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    neurons: Vec<Neuron>,
}