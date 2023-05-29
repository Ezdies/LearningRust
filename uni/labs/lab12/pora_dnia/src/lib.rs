use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct PoraDnia {
    ss: u32,
}

impl PoraDnia {
    fn new(hh: u32, mm: u32, ss: u32) -> PoraDnia {
        PoraDnia {
            ss: hh * 3600 + mm * 60 + ss,
        }
    }
    fn h(&self) -> u32 {
        (self.ss / 3600) % 24
    }
    fn m(&self) -> u32 {
        (self.ss % 3600) / 60
    }
    fn s(&self) -> u32 {
        self.ss % 60
    }
    fn as24h(&self) -> String {
        format!("{}:{}:{}", self.h(), self.m(), self.s())
    }
    fn as12h(&self) -> String {
        if self.h() <= 12 {
            return format!("{}:{}:{}AM", self.h(), self.m(), self.s());
        }
        return format!("{}:{}:{}PM", self.h() - 12, self.m(), self.s());
    }
}

impl Add for PoraDnia {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ss: self.ss + rhs.ss,
        }
    }
}

impl Sub for PoraDnia {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            ss: self.ss - rhs.ss,
        }
    }
}

pub fn is_bigger(a: PoraDnia, b: PoraDnia) -> bool {
    a > b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_construction() {
        let pd = PoraDnia::new(23, 5, 59);
        assert_eq!(pd.h(), 23);
        assert_eq!(pd.m(), 5);
        assert_eq!(pd.s(), 59);
    }
    #[test]

    fn equal() {
        let pd = PoraDnia::new(23, 10, 11);
        let pd2 = PoraDnia::new(23, 10, 11);
        assert_eq!(pd, pd2);
    }

    #[test]
    fn bigger() {
        let pd = PoraDnia::new(23, 10, 11);
        let pd2 = PoraDnia::new(23, 10, 12);
        assert_eq!(is_bigger(pd2, pd), true);
    }

    #[test]
    fn sum() {
        let pd = PoraDnia::new(10, 11, 12);
        let pd2 = PoraDnia::new(10, 11, 12);
        let pd3 = PoraDnia::new(20, 22, 24);
        let pd4 = pd + pd2;
        assert_eq!(pd3, pd4);
    }

    #[test]
    fn diff() {
        let pd = PoraDnia::new(20, 20, 20);
        let pd2 = PoraDnia::new(10, 10, 10);
        let pd3 = PoraDnia::new(10, 10, 10);
        let pd4 = pd - pd2;
        assert_eq!(pd3, pd4);
    }

    #[test]
    fn display_as_24() {
        let pd = PoraDnia::new(21, 37, 37);
        assert_eq!(pd.as24h(), "21:37:37");
    }

    #[test]
    fn display_as_12() {
        let pd = PoraDnia::new(21, 37, 37);
        assert_eq!(pd.as12h(), "9:37:37PM");
    }
}
