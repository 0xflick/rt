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

#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn test_new_f32() {
        let vec: Vector3<f32> = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn test_new_f64() {
        let vec: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn test_zeros() {
        let zeros: Vector3<f32> = Vector3::zero();

        assert_eq!(zeros.x, 0.0);
        assert_eq!(zeros.y, 0.0);
        assert_eq!(zeros.z, 0.0);
    }

    #[test]
    fn test_squared_length() {
        let vec = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert_eq!(vec.squared_length(), 3.0);
    }

    #[test]
    fn test_length() {
        let vec = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(vec.length(), 3.0);
    }

    #[test]
    fn test_ops() {
        let vec1 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let vec2 = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(
            vec1 + vec2,
            Vector3 {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            }
        );
        assert_eq!(
            vec2 - vec1,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
        );
        assert_eq!(
            vec2 * vec2,
            Vector3 {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            }
        );
    }
}
