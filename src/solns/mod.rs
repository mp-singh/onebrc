pub mod soln;

#[derive(Debug)]
pub struct Temperature {
    pub min: i16,
    pub sum: i16,
    pub max: i16,
    pub count: u32,
}
impl Temperature {
    pub fn new(temp: i16) -> Self {
        Self {
            min: temp,
            sum: temp,
            max: temp,
            count: 1,
        }
    }
    pub fn mean(&self) -> i16 {
        self.sum / self.count as i16
    }
}
