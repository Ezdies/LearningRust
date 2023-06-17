pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod odejmowanie {
    pub fn sub(left: usize, right: usize) -> usize {
        left - right
    }
}

#[cfg(test)]
mod tests {
//     use super::add;
    use crate::add;
//     use crate::odejmowanie::sub;
    use super::odejmowanie::sub;

    #[test]
    fn test_odejmowania() {
        let result = sub(6, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_dodawania() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
