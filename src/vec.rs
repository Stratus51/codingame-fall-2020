#[derive(Clone)]
pub struct Vec4<T> {
    data: [T; 4],
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for Vec4<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
                self.data[3] + other.data[3],
            ],
        }
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
                self.data[3] - other.data[3],
            ],
        }
    }
}

impl<T> std::ops::Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> From<[T; 4]> for Vec4<T> {
    fn from(data: [T; 4]) -> Self {
        Self { data }
    }
}

impl From<Vec4<u32>> for Vec4<i32> {
    fn from(data: Vec4<u32>) -> Self {
        Self {
            data: [
                data[0] as i32,
                data[1] as i32,
                data[2] as i32,
                data[3] as i32,
            ],
        }
    }
}

pub trait Abs {
    type Output;
    fn absolute(&self) -> Self;
}

impl Abs for u32 {
    type Output = Self;
    fn absolute(&self) -> Self {
        *self
    }
}

impl Abs for i32 {
    type Output = Self;
    fn absolute(&self) -> Self {
        self.abs()
    }
}

impl Abs for f32 {
    type Output = Self;
    fn absolute(&self) -> Self {
        self.abs()
    }
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl<T> Vec4<T> {
    pub fn norm1(&self) -> T
    where
        T: std::iter::Sum<T> + Abs,
    {
        self.data.iter().map(|n| n.absolute()).sum()
    }
    // fn norm2(&self) -> f32
    // where
    //     f32: std::iter::Sum<T>,
    // {
    //     self.data.iter().map(|v| *v * *v).sum::<f32>().sqrt()
    // }

    pub fn dot_product(&self, other: &Self) -> T
    where
        T: std::ops::Mul<Output = T> + std::iter::Sum<T> + Copy,
    {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a * *b)
            .sum()
    }
}

impl Vec4<i32> {
    pub fn positive(&self) -> Self {
        [
            if self[0] > 0 { self[0] } else { 0 },
            if self[1] > 0 { self[1] } else { 0 },
            if self[2] > 0 { self[2] } else { 0 },
            if self[3] > 0 { self[3] } else { 0 },
        ]
        .into()
    }
}
