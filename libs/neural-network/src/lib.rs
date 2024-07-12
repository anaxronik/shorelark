#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}

#[cfg(test)]
mod neuron_tests {
    use super::*;

    #[test]
    fn test_propagate_positive_output() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![0.2, 0.8],
        };
        let inputs = [1.0, 0.5];
        let output = neuron.propagate(&inputs);
        let expected_output = 0.5 + (1.0 * 0.2 + 0.5 * 0.8); // Expected: 0.5 + 0.6 = 1.1
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_propagate_negative_output() {
        let neuron = Neuron {
            bias: -1.0,
            weights: vec![0.2, 0.8],
        };
        let inputs = [1.0, 0.5];
        let output = neuron.propagate(&inputs);
        assert!(
            output >= 0.0,
            "Output should be non-negative due to ReLU activation."
        );
    }
}
