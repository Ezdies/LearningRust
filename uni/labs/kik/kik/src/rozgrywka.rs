use crate::gra::gra;
use crate::gracz_czlowiek::GraczCzlowiek;
use crate::gracz_komputer_losowy::GraczKomputerLosowy;
use crate::zawartosc_pola::ZawartoscPola;
use crate::gracz::Gracz;

fn rozegraj() {
    // .... pytania o graczy
    gra(ZawartoscPola::Kolko,
        &mut GraczCzlowiek::new(ZawartoscPola::Kolko),
        &mut GraczKomputerLosowy::new(ZawartoscPola::Krzyzyk));
}
