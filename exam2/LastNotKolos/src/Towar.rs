#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Towar {
    nazwa: String,
    cena: f32,
}

impl Towar {
    pub fn new(nazwa: String, cena: f32) -> Towar {
        Towar { nazwa, cena }
    }

    pub fn zwroc_nazwe(&self) -> String {
        self.nazwa.clone()
    }

    pub fn zwroc_cene(&self) -> f32 {
        self.cena
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn correct_towar_construction() {
        let nazwa = "chleb".to_string();
        let cena = 2.99;
        let towar = Towar::new("chleb".to_string(), 2.99);
        assert_eq!(nazwa, towar.zwroc_nazwe());
        assert_eq!(cena, towar.zwroc_cene());
    }
}
