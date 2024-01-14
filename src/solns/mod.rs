pub mod soln1;
pub mod soln2;

#[derive(Debug)]
pub struct Temperature {
    pub min: f32,
    pub sum: f32,
    pub max: f32,
    pub count: u32,
}
impl Temperature {
    pub fn new(min: f32, sum: f32, max: f32) -> Self {
        Self {
            min,
            sum,
            max,
            count: 1,
        }
    }
    pub fn mean(&self) -> f32 {
        self.sum / self.count as f32
    }
}
