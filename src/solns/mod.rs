pub mod soln;

pub type Name = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Temperature {
    pub name: Name,
    pub min: i16,
    pub sum: i16,
    pub max: i16,
    pub count: u16,
}
impl Temperature {
    pub fn new(name: Vec<u8>, temp: i16) -> Self {
        Self {
            name,
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
