#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn przeksztalc(&mut self, przeksz: Przeksztalcenie) {
        match przeksz {
            Przeksztalcenie::Przesuniecie(point) => {
                self.x += point.x;
                self.y += point.y;
            }
            Przeksztalcenie::Odbicie(os) => match os {
                Os::Ox => {
                    self.y *= -1.0;
                }
                Os::Oy => {
                    self.x *= -1.0;
                }
            }
            Przeksztalcenie::Obrot(angle) => {
                self.x = self.x * angle.cos() - self.y * angle.sin();
                self.y = self.x * angle.sin() + self.y * angle.cos();
            }
            Przeksztalcenie::Jednokladnosc(skala)=>{
                self.x *= skala;
                self.y *= skala;
            }
        }
    }
}

enum Os {
    Ox,
    Oy,
}

enum Przeksztalcenie {
    Przesuniecie(Point),
    Odbicie(Os),
    Obrot(f32),
    Jednokladnosc(f32),
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("{:?}", p);
}
