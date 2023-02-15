#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Vector {
    pub first: Position,
    pub second: Position,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct StandardVector {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NormalizedVector {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn scale_x(&self, scale: f64) -> Position {
        Position {
            x: (scale * (self.x as f64)) as i32,
            y: self.y,
        }
    }

    pub fn scale_y(&self, scale: f64) -> Position {
        Position {
            x: self.x,
            y: (scale * (self.y as f64)) as i32,
        }
    }

    pub fn add_vector(&self, vector: StandardVector) -> Position {
        Position {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl Vector {
    pub fn to_standard(&self) -> StandardVector {
        StandardVector {
            x: self.second.x - self.first.x,
            y: self.second.y - self.first.y,
        }
    }

    pub fn reversed(&self) -> Vector {
        Vector {
            first: self.second.clone(),
            second: self.first.clone(),
        }
    }
}

impl StandardVector {
    pub fn to_normalized(&self) -> NormalizedVector {
        NormalizedVector {
            x: (self.x as f64) / self.len(),
            y: (self.y as f64) / self.len(),
        }
    }

    pub fn len(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    pub fn add(&self, vector: StandardVector) -> StandardVector {
        StandardVector {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl NormalizedVector {
    pub fn to_standard(&self, length: f64) -> StandardVector {
        StandardVector {
            x: (self.x * length) as i32,
            y: (self.y * length) as i32,
        }
    }
}

#[cfg(test)]
mod normalized_vector_test {
    use crate::vectors::{NormalizedVector, StandardVector};

    #[test]
    fn standard_to_normalized_test() {
        let vec = StandardVector {
            x: 3,
            y: 4,
        };
        let expected = NormalizedVector {
            x: 0.6,
            y: 0.8,
        };
        assert_eq!(expected, vec.to_normalized());
    }

    #[test]
    fn normalized_to_standard_test() {
        let vec = NormalizedVector {
            x: 0.6,
            y: 0.8,
        };
        let expected = StandardVector {
            x: 3,
            y: 4,
        };
        assert_eq!(expected, vec.to_standard(5.0));
    }
}



