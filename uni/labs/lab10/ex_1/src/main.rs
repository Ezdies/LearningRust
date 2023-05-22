#[derive(Debug)]
struct RandGen {
    seed: usize,
}

impl RandGen {
    fn new(sed: usize) -> RandGen {
        RandGen { seed: sed }
    }
    fn gen_range(&self, min_rand: usize, max_rand: usize) -> usize {
        let mut last_seed = self.seed;
        let x = 75;
        let c = 74;
        let m = 65537;
        last_seed = (x * (last_seed) + c) % m;
        return last_seed % (max_rand - min_rand + 1) + min_rand;
    }
}

#[derive(Debug)]
struct Urna {
    elementy: Vec<Option<char>>,
    index: RandGen,
}

impl Urna {
    fn new(generator: RandGen) -> Urna {
        Urna {
            elementy: vec![],
            index: (generator),
        }
    }

    fn doloz(&self, element: char) {
        self.elementy.push(Some(element));
    }
    fn losuj_bez_us(&self, generator: RandGen) -> Option<char>{
        self.elementy.remove(generator.gen_range(0, self.elementy.len()));

    }
}

fn main() {
    let mut generator = RandGen::new(123);
    let a = generator.gen_range(3, 123406);
    println!("{a}");
}
