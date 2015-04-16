use std::ops::{Add, Sub, Mul};

pub struct Vector2{
    pub x: f32,
    pub y: f32,
}

pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Matrix4{
    pub m: [f32; 16],
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

impl Matrix4{
    pub fn copy(other:& Matrix4)-> Matrix4{
        Matrix4{
            m: other.m,
        }
    }

    pub fn new() -> Matrix4{
        Matrix4{
            m :[1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0],
        }
    }

    pub fn to_ortho(left: f32,right: f32,top: f32, bottom: f32) -> Matrix4{
        let width = right-left;
        let heigth = top-bottom;
        let depth = 100.0 - 0.001;
        Matrix4{
            m: [2.0/width,0.0,0.0,0.0,
            0.0,2.0/width,0.0,0.0,
            0.0,0.0,-2.0/depth,0.0,
            -(right + left)/width,-(top + bottom)/heigth,-(100.0 + 0.001)/depth,1.0],
        }
    }

    pub fn scale(&mut self,scale:& Vector2){
        self.m[0]*=scale.x;
        self.m[5]*=scale.y;
    }
    pub fn offset(&mut self,offset:& Vector2){
        self.m[12]+=offset.x;
        self.m[13]+=offset.y;
    }

    pub fn print (&self){
        for i in 0..4{
            for j in 0..4{
                print!("{} ",self.m[i*4+j]);
            }
            println!("");
        }
    }
}

impl Mul for Matrix4{
    type Output = Matrix4;
    fn mul(self,other: Matrix4)-> Matrix4{
        let mut res = Matrix4{
            m:[0.0; 16],
        };
        for i in 0..4{
            for j in 0..4{
                for k in 0..4{
                    res.m[i*4+j] += self.m[k*4+j] * other.m[i*4+k];
                }
            }
        }
        res
    }
}


