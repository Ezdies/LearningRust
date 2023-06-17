pub struct Plansza {
    
}

impl Plansza {
    
}

#[cfg(test)]
mod tests {
    use super::Plansza;
    
    #[test]
    fn test1() {
        let p = Plansza::new();
        assert_eq!(p.zawartosc_pola(0, 0), ZawartoscPola::Puste);
    }
    
    #[test]
    fn test2() {
        let p = Plansza::new();
        p.wykonaj_ruch(1, 2, ZawartoscPola::Kolko);
        assert_eq!(p.zawartosc_pola(1, 2), ZawartoscPola::Kolko);
        assert_eq!(p.zawartosc_pola(2, 1), ZawartoscPola::Puste);
    }
    
    #[test]
    fn test3() {
        let p = Plansza::new();
        p.wykonaj_ruch(1, 2, ZawartoscPola::Kolko);
        assert_eq!(p.ruch_poprawny(1, 2, ZawartoscPola::Krzyzyk), false);
    }
    
    #[test]
    fn test4() {
        let p = Plansza::new();
        assert_eq!(p.wynik(), Wynik::GraNiezakonczona);
        p.wykonaj_ruch(0, 0, ZawartoscPola::Kolko);
        assert_eq!(p.wynik(), Wynik::GraNiezakonczona);
        p.wykonaj_ruch(1, 1, ZawartoscPola::Kolko);
        assert_eq!(p.wynik(), Wynik::GraNiezakonczona);
        p.wykonaj_ruch(2, 2, ZawartoscPola::Kolko);
        assert_eq!(p.wynik(), Wynik::Kolko);
    }
}
