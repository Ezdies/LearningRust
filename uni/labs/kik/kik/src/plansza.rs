use crate::zawartosc_pola::ZawartoscPola;
use crate::wynik::Wynik;

pub type WspolrzednePola = (usize, usize);

pub struct Plansza {
    dane: Vec<Vec<ZawartoscPola>>,
}

impl Plansza {

    pub fn new() -> Plansza {
        Plansza {
            dane: vec![vec![ZawartoscPola::Puste; 3]; 3]
        }
    }
    
    pub fn zawartosc_pola(&self, wsp: WspolrzednePola) -> ZawartoscPola {
        self.dane[wsp.0][wsp.1]
    }
    
    pub fn wykonaj_ruch(&mut self, wsp: WspolrzednePola, z: ZawartoscPola) {
        self.dane[wsp.0][wsp.1] = z;
    }
    
    pub fn ruch_poprawny(&self, wsp: WspolrzednePola) -> bool {
        self.dane[wsp.0][wsp.1] == ZawartoscPola::Puste
    }
    
    pub fn wynik(&self) -> Wynik {
        if self.zawartosc_pola((0, 0)) == self.zawartosc_pola((1, 1)) && self.zawartosc_pola((1, 1)) == self.zawartosc_pola((2, 2)) &&
        self.zawartosc_pola((0, 0)) != ZawartoscPola::Puste {
            return Wynik::wygrana(self.zawartosc_pola((0, 0)))
        }
        //...
        Wynik::GraNiezakonczona
    }
    
}

#[cfg(test)]
mod tests {
    use super::Plansza;
    use crate::zawartosc_pola::ZawartoscPola;
    use crate::wynik::Wynik;

    #[test]
    fn test1() {
        let p = Plansza::new();
        assert_eq!(p.zawartosc_pola((0, 0)), ZawartoscPola::Puste);
    }
    
    #[test]
    fn test2() {
        let mut p = Plansza::new();
        p.wykonaj_ruch((1, 2), ZawartoscPola::Kolko);
        assert_eq!(p.zawartosc_pola((1, 2)), ZawartoscPola::Kolko);
        assert_eq!(p.zawartosc_pola((2, 1)), ZawartoscPola::Puste);
    }
    
    #[test]
    fn test3() {
        let mut p = Plansza::new();
        p.wykonaj_ruch((1, 2), ZawartoscPola::Kolko);
        assert_eq!(p.ruch_poprawny((1, 2)), false);
    }
    
    #[test]
    fn test4() {
        let mut p = Plansza::new();
        assert_eq!(p.wynik(), Wynik::GraNiezakonczona);
        p.wykonaj_ruch((0, 0), ZawartoscPola::Kolko);
        assert_eq!(p.wynik(), Wynik::GraNiezakonczona);
        p.wykonaj_ruch((1, 1), ZawartoscPola::Kolko);
        assert_eq!(p.wynik(), Wynik::GraNiezakonczona);
        p.wykonaj_ruch((2, 2), ZawartoscPola::Kolko);
        assert_eq!(p.wynik(), Wynik::Kolko);
    }
}
