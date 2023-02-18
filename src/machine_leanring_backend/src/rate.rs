#[derive(Clone)]
pub struct LearningRate {
    pub rate: f32,
    pub min: f32,
    pub step: f32,
}

impl LearningRate {
    pub fn update(&mut self) {
        self.rate = f32::max(self.rate - self.step, self.min)
    }
}