#[derive(Copy, Clone)]
pub struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub struct Vector {
    pub(crate) first: Position,
    pub(crate) second: Position,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct StandardVector {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NormalizedVector {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

pub fn to_standard(vector: Vector) -> StandardVector {
    StandardVector {
        x: vector.second.x - vector.first.x,
        y: vector.second.y - vector.first.y,
    }
}

pub fn standard_to_normalized(vector: StandardVector) -> NormalizedVector {
    NormalizedVector {
        x: (vector.x as f64) / standard_len(vector),
        y: (vector.y as f64) / standard_len(vector),
    }
}

pub fn normalized_to_standard(vector: NormalizedVector, length: f64) -> StandardVector {
    StandardVector {
        x: (vector.x * length) as i32,
        y: (vector.y * length) as i32,
    }
}

pub fn standard_len(vector: StandardVector) -> f64 {
    ((vector.x.pow(2) + vector.y.pow(2)) as f64).sqrt()
}

pub fn scale_standard_vector(vector: StandardVector, scale: f64) -> StandardVector {
    StandardVector {
        x: (scale * (vector.x as f64)) as i32,
        y: (scale * (vector.y as f64)) as i32,
    }
}

pub fn scale_position_x(position: Position, scale: f64) -> Position {
    Position {
        x: (scale * (position.x as f64)) as i32,
        y: position.y,
    }
}

pub fn scale_position_y(position: Position, scale: f64) -> Position {
    Position {
        x: position.x,
        y: (scale * (position.y as f64)) as i32,
    }
}

pub fn move_position_by_vector(position: Position, vector: StandardVector) -> Position {
    Position {
        x: position.x + vector.x,
        y: position.y + vector.y,
    }
}

pub fn add_vectors(vector1: StandardVector, vector2: StandardVector) -> StandardVector {
    StandardVector {
        x: vector1.x + vector2.x,
        y: vector1.y + vector2.y,
    }
}

#[cfg(test)]
mod standard_vector_test {
    use crate::vectors::{StandardVector, scale_standard_vector, standard_len};

    #[test]
    fn scale_positive_test() {
        let nv = StandardVector { x: 100, y: 100 };
        let snv = scale_standard_vector(nv, 0.5);
        assert_eq!(50, snv.x);
        assert_eq!(50, snv.y);
    }

    #[test]
    fn scale_negative_test() {
        let nv = StandardVector { x: -100, y: -100 };
        let snv = scale_standard_vector(nv, 0.5);
        assert_eq!(-50, snv.x);
        assert_eq!(-50, snv.y);
    }

    #[test]
    fn standard_vector_len_test() {
        let vec = StandardVector {
            x: 3,
            y: 4,
        };
        assert_eq!(5.0, standard_len(vec));
    }
}

#[cfg(test)]
mod normalized_vector_test {
    use crate::vectors::{normalized_to_standard, NormalizedVector, standard_to_normalized, StandardVector};

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
        assert_eq!(expected, standard_to_normalized(vec));
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
        assert_eq!(expected, normalized_to_standard(vec, 5.0));
    }
}



