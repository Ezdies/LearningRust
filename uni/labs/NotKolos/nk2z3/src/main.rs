struct Error {
    error_type: ErrorType,
}

enum ErrorType {
    NotAnError,
    BadFormat,
    FileNotExists,
    FileTooBig,
}

impl Error {
    fn pokaz_komunikat(error_type: ErrorType) {
        match error_type {
            ErrorType::NotAnError => {
                println!("Nie ma errora");
            }
            ErrorType::BadFormat => {
                println!("Zły format");
            }
            ErrorType::FileNotExists => {
                println!("Plik nie istnieje");
            }
            ErrorType::FileTooBig => {
                println!("Plik za duży");
            }
        }
    }
}

fn main() {
    Error::pokaz_komunikat(ErrorType::BadFormat);
}
