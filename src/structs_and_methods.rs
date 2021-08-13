struct Point{
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line{

    fn len(&self) -> f64{
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx + dy*dy).sqrt()
    }
}

pub fn call_structs(){

    let p1 = Point {x: 3.5, y: 4.2};
    let p2 = Point {x: 0.0, y: 1.0};

    let line = Line{start: p1, end: p2};

    println!("la linea mide {}", line.len());
}