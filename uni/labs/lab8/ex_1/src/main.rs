#[derive(Debug)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    fn from_3u8(r: u8, g: u8, b: u8) -> RGB {
        RGB { r: r, g: g, b: b }
    }

    fn from_3f32(r: f32, g: f32, b: f32) -> Option<RGB> {
        if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
            return None;
        }

        let new_r = (255.0 * r) as u8;
        let new_g = (255.0 * g) as u8;
        let new_b = (255.0 * b) as u8;

        Some(RGB {
            r: new_r,
            g: new_g,
            b: new_b,
        })
    }
    fn red() -> RGB {
        RGB {
            r: 255,
            g: 255,
            b: 0,
        }
    }
    fn brightness_as_8(&self) -> u8 {
        (self.r + self.g + self.b) / 3
    }

    fn gray(&self, k: f32) -> Option<RGB>{
        if k > 1.0 || k < 0.0{
            return  None;
        }
        Some(RGB{r :self.r as f32 * k, g: self.g as f32 * k, b: self.b as as f32 * k})
    }
}

fn main() {
    let p1 = RGB {
        r: 0,
        g: 250,
        b: 255,
    };

    let _p2 = RGB::from_3u8(20, 30, 40);

    println!("{:?}", p1);
}
