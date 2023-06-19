use crate::Towar::Towar;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct PozycjaZamowienia {
    towar: Towar,
    ilosc: u32,
}

impl PozycjaZamowienia {
    pub fn new(towar: Towar, ilosc: u32) -> PozycjaZamowienia {
        PozycjaZamowienia { towar, ilosc }
    }
    pub fn zwroc_towar(&self) -> Towar {
        self.towar.clone()
    }
    pub fn ile_sztuk(&self) -> u32 {
        self.ilosc
    }

    pub fn zwroc_wartosc(&self) -> f32 {
        self.ilosc as f32 * self.towar.zwroc_cene()
    }

    pub fn zmien_liczbe_sztuk(&mut self, ilosc: u32) {
        self.ilosc = ilosc
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn correct_pozycja_construction() {
        let towar = Towar::new("chleb".to_string(), 2.99);
        let ilosc = 3;

        let pozycja_zamowienia = PozycjaZamowienia::new(towar.clone(), ilosc);

        assert_eq!(pozycja_zamowienia.zwroc_towar(), towar);
        assert_eq!(pozycja_zamowienia.ile_sztuk(), ilosc);
    }

    #[test]
    fn correct_value() {
        let towar = Towar::new("chleb".to_string(), 2.99);
        let ilosc = 3;

        let pozycja_zamowienia = PozycjaZamowienia::new(towar.clone(), ilosc);

        assert_eq!(pozycja_zamowienia.zwroc_wartosc(), 8.97);
    }
}
