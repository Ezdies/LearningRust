#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ZawartoscPola {
    Puste, Kolko, Krzyzyk
}

impl ZawartoscPola {
    pub fn zmien_gracza(&mut self) -> ZawartoscPola {
        match self {
            ZawartoscPola::Kolko => ZawartoscPola::Krzyzyk,
            ZawartoscPola::Krzyzyk => ZawartoscPola::Kolko,
            ZawartoscPola::Puste => panic!("próba zamiany gracza o pustym pionku...!?")
        }
    }
}

#[cfg(test)]
mod tests {
    
}
