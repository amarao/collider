use speedy2d::dimen::Vector2;
#[derive(Debug)]
pub struct TheDot {
    logic: f32,
    draw_radius: u32,
    mass: u32,
    speed: u32,
    position: Vector2<f32>,
}

impl TheDot {
    pub fn new(logic: f32) -> Self {
        TheDot {
            logic,
            draw_radius: 5,
            mass: 0,
            speed: 0,
            position: Vector2::new(5.0, 5.0),
        }
    }

    pub fn draw_info(&self, scale_factor: f64) -> (Vector2<f32>, f32) {
        (
            self.position,
            (self.draw_radius as f64 * scale_factor) as f32,
        )
    }
    pub fn accelerate_tick (&mut self){

    }
    pub fn restart(&mut self, canvas_size: Vector2<u32>) {
        self.position = canvas_size.into_f32()/self.logic;
    }
}
