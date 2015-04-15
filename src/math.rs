use std::ops::{Add, Sub, Mul};

pub struct Vector2{
    x: f32,
    y: f32,
}

pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Matrix3{
    m: [f32; 9],
}

impl Vector2{
    pub fn new() -> Vector2{
        Vector2{
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Add for Vector2{
    type Output = Vector2;
    fn add(self,other: Vector2) -> Vector2{
        Vector2{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2{
    type Output = Vector2;
    fn sub(self,other: Vector2) -> Vector2{
        Vector2{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Vector2{
    type Output = Vector2;
    fn mul(self,other: Vector2) -> Vector2{
        Vector2{
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Matrix3{
    pub fn new() -> Matrix3{
        Matrix3{
            m :[1.0,0.0,0.0,
            0.0,1.0,0.0,
            0.0,0.0,1.0],
        }
    }
}

