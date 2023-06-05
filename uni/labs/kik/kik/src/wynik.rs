use crate::zawartosc_pola::ZawartoscPola;

#[derive(PartialEq, Debug)]
pub enum Wynik {
    Remis, Kolko, Krzyzyk, GraNiezakonczona
}

impl Wynik {
    pub fn wygrana(z: ZawartoscPola) -> Wynik {
        match z {
            ZawartoscPola::Kolko => Wynik::Kolko,
            ZawartoscPola::Krzyzyk =>
            Wynik::Krzyzyk,
            ZawartoscPola::Puste => 
            panic!("wygrana pola Pustego niedopuszczalna!"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::zawartosc_pola::ZawartoscPola;
    use super::Wynik;
    
    #[test]
    fn test1() {
        assert_eq!(Wynik::wygrana(ZawartoscPola::Kolko), Wynik::Kolko);
    }
    
    #[test]
    #[should_panic]
    fn test2() {
        Wynik::wygrana(ZawartoscPola::Puste);
    }
    
}
