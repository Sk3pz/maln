
// σ Σ
/// Takes a number and scales it closer to zero for the lower it is, and
/// closer to one for the higher it is.
/// Returns a number between 0 and 1.
/// The returned value will be NAN (Not a Number) if the input is too high or low.
pub fn sigmoid(x: f64) -> f64 {
    (std::f64::consts::E.powf(x.clone())) / (std::f64::consts::E.powf(x) + 1.0)
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