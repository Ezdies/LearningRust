use crate::plansza::Plansza;
use crate::zawartosc_pola::ZawartoscPola;
use crate::gracz::Gracz;
use crate::wynik::Wynik;
use crate::interfejs::*;

pub fn gra(pion_startowy: ZawartoscPola,
       gracz1: &mut dyn Gracz,
       gracz2: &mut dyn Gracz) {
    let mut biezacy_pionek = pion_startowy;
    let mut plansza = Plansza::new();
    loop {
        wyswietl_plansze(&plansza);
        let wspolrzedne_ruchu =
            if gracz1.pionek() == biezacy_pionek {
                gracz1.spytaj_o_ruch(&biezacy_pionek)
            } else if gracz2.pionek() == biezacy_pionek {
                gracz2.spytaj_o_ruch(&biezacy_pionek)
            } else {
                panic!("żadenz graczy nie ma bieżącego pionka...!?")
            };
        if plansza.ruch_poprawny(wspolrzedne_ruchu) {
            plansza.wykonaj_ruch(wspolrzedne_ruchu, biezacy_pionek);
            komunikat_o_poprawnym_ruchu(&wspolrzedne_ruchu, &biezacy_pionek);
            biezacy_pionek = biezacy_pionek.zmien_gracza();
        } else {
            komunikat_o_niepoprawnym_ruchu(&wspolrzedne_ruchu, &biezacy_pionek)
        }
        if plansza.wynik() != Wynik::GraNiezakonczona {
            break;
        }
    }
    wyswietl_plansze(&plansza);
    komunikat_o_wyniku(&plansza.wynik());
}
