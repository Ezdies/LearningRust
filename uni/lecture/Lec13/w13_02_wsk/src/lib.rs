#[derive(Clone)]
pub enum Stos {
    Pusty,
    Pudelko { glowa: i64, ogon: Box<Stos> },
}

impl Stos {
    pub fn new() -> Self {
        Stos::Pusty
    }

    pub fn empty(&self) -> bool {
        //         match self {
        //             Stos::Pusty => true,
        //             _ => false,
        //         }
        matches!(self, Stos::Pusty)
    }

    pub fn top(&self) -> Option<i64> {
        match self {
            Stos::Pudelko { glowa: g, ogon: _ } => Some(*g),
            Stos::Pusty => None,
        }
    }

    pub fn push(self, x: i64) -> Self {
        Stos::Pudelko {
            glowa: x,
            ogon: Box::new(self),
        }
    }

    pub fn pop(self) -> Option<Self> {
        match self {
            Stos::Pudelko { glowa: _, ogon: o } => Some(*o),
            Stos::Pusty => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let s1 = Stos::new();
        assert!(s1.empty());
        let s2 = s1.push(12);
        assert_eq!(s2.top(), Some(12));
    }
}

// &
// Box
// Rc
// Arc

// RefCell
// Ref
// RefMut
