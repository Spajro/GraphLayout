#[derive(Copy, Clone)]
pub struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub struct Vector {
    pub(crate) first: Position,
    pub(crate) second: Position,
}

#[derive(Copy, Clone)]
pub struct NormalizedVector {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub fn diff(vector: Vector) -> NormalizedVector {
    NormalizedVector {
        x: vector.second.x - vector.first.x,
        y: vector.second.y - vector.first.y,
    }
}

pub fn scale(vector: NormalizedVector, scale: f64) -> NormalizedVector {
    NormalizedVector {
        x: (scale * (vector.x as f64)) as i32,
        y: (scale * (vector.y as f64)) as i32,
    }
}

pub fn add(position: Position, vector: NormalizedVector) -> Position {
    Position {
        x: position.x + vector.x,
        y: position.y + vector.y,
    }
}

pub fn join(vector1: NormalizedVector, vector2: NormalizedVector) -> NormalizedVector {
    NormalizedVector {
        x: vector1.x + vector2.x,
        y: vector1.y + vector2.y,
    }
}

mod tests{
    use crate::math2d::{NormalizedVector, scale};

    #[test]
    fn scale_positive_test(){
        let nv=NormalizedVector{ x: 100, y: 100 };
        let snv=scale(nv,0.5);
        assert_eq!(50,snv.x);
        assert_eq!(50,snv.y);
    }

    #[test]
    fn scale_negative_test(){
        let nv=NormalizedVector{ x: -100, y: -100 };
        let snv=scale(nv,0.5);
        assert_eq!(-50,snv.x);
        assert_eq!(-50,snv.y);
    }
}