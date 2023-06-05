use crate::zawartosc_pola::ZawartoscPola;
use crate::plansza::WspolrzednePola;
use crate::gracz::Gracz;

pub struct GraczCzlowiek {
    p: ZawartoscPola,
}

impl Gracz for GraczCzlowiek {
    fn new(p: ZawartoscPola) -> Self {
        Self {p}
    }

    fn pionek(&self) -> ZawartoscPola {
        self.p
    }
    
    fn spytaj_o_ruch(&self, pionek: &ZawartoscPola) -> WspolrzednePola {
        (0, 0)
    }
}
