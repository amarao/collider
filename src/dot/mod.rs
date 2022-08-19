use speedy2d::dimen::Vector2;
#[derive(Debug)]
pub struct TheDot {
    logic: f32,
    draw_radius: u32,
    mass: u32,
    speed: Vector2<f32>,
    position: Vector2<f32>,
}

impl TheDot {
    pub fn new(logic: f32) -> Self {
        TheDot {
            logic,
            draw_radius: 5,
            mass: 0,
            speed: Vector2::new(0.0, 0.0),
            position: Vector2::new(5.0, 5.0),
        }
    }

    pub fn draw_info(&self, scale_factor: f64) -> (Vector2<f32>, f32) {
        (
            self.position,
            (self.draw_radius as f64 * scale_factor) as f32,
        )
    }
    pub fn accelerate_tick (&mut self, accel: Vector2<f32>){
        // self.speed = self.speed + (self.position - accel);
        self.speed = self.speed + Vector2::new(1.0,1.0);
    }
    pub fn speed_tick(&mut self, canvas: Vector2<u32>){
        self.position = self.position + self.speed;
        if self.position.x >= canvas.x as f32{
            self.position.x = canvas.x as f32;
            self.speed.x = - self.speed.x;
        }
        if self.position.y >= canvas.y as f32{
            self.position.y = canvas.y as f32;
            self.speed.y = - self.speed.y;
        }
        if self.position.x <= 0.0{
            self.position.x = 0.0;
            self.speed.x = - self.speed.x;
        }
        if self.position.y <= 0.0{
            self.position.y = 0.0;
            self.speed.y = - self.speed.y;
        }


    }
    pub fn restart(&mut self, canvas_size: Vector2<u32>) {
        self.position = canvas_size.into_f32()/self.logic;
    }
}
