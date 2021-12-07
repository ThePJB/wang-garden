use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x * scalar, self.y * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x / scalar, self.y / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
    pub fn normalize(&self) -> Vec2 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec2, t: f32) -> Vec2 { Vec2::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t)) }
    pub fn rotate(&self, radians: f32) -> Vec2 { 
        Vec2::new(
            self.x * radians.cos() - self.y * radians.sin(), 
            self.x * radians.sin() + self.y * radians.cos()
        ) 
    }
    
    pub fn spread(&self, amount: f32) -> Vec2 {
        if amount == 0.0 {return *self};
        let roll = rand::thread_rng().gen_range(-amount..amount);
        return self.rotate(roll);
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: f32) -> Vec2 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        self.mul_scalar(-1.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{x, y, z} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z).sqrt() }
    pub fn square_distance(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z }
    pub fn normalize(&self) -> Vec3 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec3, t: f32) -> Vec3 { Vec3::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t), self.z*(1.0-t) + other.z*(t)) }
    pub fn dist(&self, other: Vec3) -> f32 {(*self - other).magnitude().sqrt()}
    pub fn with_w(&self, w: f32) -> Vec4 {
        Vec4{x: self.x, y: self.y, z: self.z, w}
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self.mul_scalar(-1.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4{x, y, z, w} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec4 { Vec4::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec4 { Vec4::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt() }
    pub fn square_distance(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w}
    pub fn normalize(&self) -> Vec4 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec4, t: f32) -> Vec4 { Vec4::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t), self.z*(1.0-t) + other.z*(t), self.w*(1.0-t) + other.w*t) }
    pub fn dist(&self, other: Vec4) -> f32 {(*self - other).magnitude().sqrt()}
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, _rhs: Vec4) -> Vec4 {
        Vec4 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z, w: self.w - _rhs.w }
    }
}

impl std::ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, _rhs: Vec4) -> Vec4 {
        Vec4 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z, w: self.w + _rhs.w}
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, _rhs: f32) -> Vec4 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, _rhs: Vec4) -> Vec4 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, _rhs: f32) -> Vec4 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        self.mul_scalar(-1.0)
    }
}