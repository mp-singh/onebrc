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
    pub fn update(&mut self, temp: i16) {
        self.sum += temp;
        self.count += 1;
        if self.min > temp {
            self.min = temp;
            return;
        }
        if self.max < temp {
            self.max = temp;
        }
    }
}
