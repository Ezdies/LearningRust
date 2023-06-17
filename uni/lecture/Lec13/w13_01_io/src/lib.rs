pub fn arg_wiersza_pol() {
    for a in std::env::args() {
        println!("argument: {:?}", a)
    }
}

use std::io;
pub fn czytaj_z_konsoli() {
    let konsola = io::stdin();
    let mut odp = String::new();
    print!("wpisz coś: ");
    io::stdout().flush().expect("problem z flushowaniem");
    // to wyżej po to, żeby wypchać z bufora niepełny wiersz danych z poprzedniej linii programu
    konsola.read_line(&mut odp).expect("błąd wczytywania");
    println!("wpisano: {:?}", odp);
}

use std::fs::File;
use std::io::Read;
pub fn czytaj_z_pliku() {
    let mut file = File::open("/home/jmbylina/UMCS-22-23/lato/PwJR-22-23/w13/w13_01_io/src/lib.rs")
        .expect("błąd otwierania pliku");
    // let mut file = File::open("/home/jmbylina/UMCS-22-23/lato/PwJR-22-23/w13/w13_01_io/target/debug/libw13_01_io.rlib")
    //     .expect("błąd otwierania pliku");
    let mut dane = String::new();
    file.read_to_string(&mut dane)
        .expect("bład czytania z pliku");
    println!("wczytano z pliku: {:?}", dane);
}

use std::io::BufRead;
use std::io::BufReader;
pub fn czytaj_linie() {
    let file = File::open("/home/jmbylina/UMCS-22-23/lato/PwJR-22-23/w13/w13_01_io/src/lib.rs")
        .expect("błąd otwierania pliku");
    let reader = BufReader::new(file);
    for linia in reader.lines() {
        println!("wczytano linię: {:?}", linia.expect("błąd czytania linii"))
    }
}

use std::io::Write;
pub fn zapisz_do_pliku() {
    let mut file = File::create("/tmp/test_zapis_1.txt").expect("błąd otwarcia pliku");
    writeln!(file, "test1").expect("problem z zapisem");
    writeln!(file, "test2").expect("problem z zapisem");
}

use std::fs::OpenOptions;
pub fn dopisz_do_pliku() {
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(true)
        .open("/tmp/test_zapis_2.txt")
        .expect("błąd otwierania pliku");
    writeln!(file, "test3").expect("problem z zapisem");
    writeln!(file, "test4").expect("problem z zapisem");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg() {
        arg_wiersza_pol();
    }

    #[test]
    fn test_czytanie_z_pliku() {
        czytaj_z_pliku();
    }

    #[test]
    fn test_czytanie_linii_z_pliku() {
        czytaj_linie();
    }

    #[test]
    fn test_zapisu_do_pliku() {
        zapisz_do_pliku();
    }

    #[test]
    fn test_dopisz_do_pliku() {
        dopisz_do_pliku();
    }
}
