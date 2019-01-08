use std::ops::{Add, Mul, Sub};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> f32 {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> f64 {
        self.sqrt()
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> f32 {
        0.0
    }
}

impl Zero for f64 {
    fn zero() -> f64 {
        0.0
    }
}

pub trait VectorComponent:
    Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Sqrt
    + Zero
    + Copy
    + PartialEq
    + Send
    + Sync
{
}

impl<T> VectorComponent for T where
    T: Add<T, Output = T>
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + Sqrt
        + Zero
        + Copy
        + PartialEq
        + Send
        + Sync
{
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3<T>
where
    T: VectorComponent,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: VectorComponent,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vector3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T> Add for Vector3<T>
where
    T: VectorComponent,
{
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vector3<T>
where
    T: VectorComponent,
{
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul for Vector3<T>
where
    T: VectorComponent,
{
    type Output = Vector3<T>;

    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
