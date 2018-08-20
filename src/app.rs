use console;
use canvas::{Canvas, Color, Method, Geometry};

pub struct App {
    canvas: Canvas,
}

impl App {
    pub fn new() -> Self {
        my_print!("App created!");
        App { canvas: Canvas::new() }
    }

    pub fn draw(&mut self) {
        self.canvas.draw(
            &Geometry::Circle { x: 400.0, y: 300.0, r: 100.0 },
            &Method::Fill { color: Color(0.5, 0.0, 1.0, 1.0) }
        );
    }
}
