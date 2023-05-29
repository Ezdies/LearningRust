use std::ops::Add;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct PoraDnia {
    hh: u32,
    mm: u32,
    ss: u32,
}

impl PoraDnia {
    fn new(hh: u32, mm: u32, ss: u32) -> PoraDnia {
        PoraDnia { hh, mm, ss }
    }
    fn h(&self) -> u32 {
        self.hh
    }
    fn m(&self) -> u32 {
        self.mm
    }
    fn s(&self) -> u32 {
        self.ss
    }
}

impl Add for PoraDnia {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            hh: self.hh + rhs.hh,
            mm: self.mm + rhs.mm,
            ss: self.ss + rhs.ss,
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
}
