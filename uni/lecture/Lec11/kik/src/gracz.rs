use crate::zawartosc_pola::ZawartoscPola;
use crate::plansza::WspolrzednePola;

pub trait Gracz {
    fn new(p: ZawartoscPola) -> Self where Self: Sized;
    fn pionek(&self) -> ZawartoscPola;
    fn spytaj_o_ruch(&self, pionek: &ZawartoscPola) -> WspolrzednePola;
}
