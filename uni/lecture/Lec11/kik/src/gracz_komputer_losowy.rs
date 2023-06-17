use crate::zawartosc_pola::ZawartoscPola;
use crate::plansza::WspolrzednePola;
use crate::gracz::Gracz;

pub struct GraczKomputerLosowy {
    p: ZawartoscPola,
}

impl Gracz for GraczKomputerLosowy {
    fn new(p: ZawartoscPola) -> Self {
        Self {p}
    }

    fn pionek(&self) -> ZawartoscPola {
        self.p
    }
    
    fn spytaj_o_ruch(&self, pionek: &ZawartoscPola) -> WspolrzednePola {
        (1, 1)
    }
}
