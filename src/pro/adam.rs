use pyo3::prelude::*;
use crate::license::ensure_pro()?;

#[pyclass]
pub struct Adam {
    lr: f64,
    beta1: f64,
    beta2: f64,
    epsilon: f64,
    m: f64,
    v: f64,
    t: u32,
}

#[pymethods]
impl Adam {
    #[new]
    pub fn new(lr: f64, beta1: f64, beta2: f64, epsilon: f64) -> Self {
        ensure_pro();
        Self { lr, beta1, beta2, epsilon, m: 0.0, v: 0.0, t: 0 }
    }

    pub fn step(&mut self, value: f64) -> f64 {
        ensure_pro();
        self.t += 1;
        let grad = value;
        self.m = self.beta1 * self.m + (1.0 - self.beta1) * grad;
        self.v = self.beta2 * self.v + (1.0 - self.beta2) * grad * grad;
        let m_hat = self.m / (1.0 - self.beta1.powi(self.t as i32));
        let v_hat = self.v / (1.0 - self.beta2.powi(self.t as i32));
        value - self.lr * m_hat / (v_hat.sqrt() + self.epsilon)
    }
}
