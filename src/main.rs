mod dot;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::{Graphics2D, Window};
use std::rc::Rc;
use std::time::Instant;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::MouseButton;

struct Game {
    dot1: dot::TheDot,
    dot2: dot::TheDot,
    canvas_size: Vector2<u32>,
    scale_factor: f64,
    mouse_position: Option<Vector2<f32>>,
    accel1_position: Option<Vector2<f32>>,
    accel2_position: Option<Vector2<f32>>,
}

impl Game{
    pub fn new() -> Self{
        Game {
            dot1:dot::TheDot::new(1.75),
            dot2:dot::TheDot::new(2.25),
            canvas_size: Vector2::new(0, 0),
            scale_factor: 1.0,
            mouse_position: None,
            accel1_position: None,
            accel2_position: None
        }
    }
}

fn main() {
    // let window = Window::new_centered("Speedy2D: Hello World", (640, 240)).unwrap();
    let window = Window::new_fullscreen_borderless("FPS draw").unwrap();
    window.run_loop(Game::new())
}

impl Game{
    pub fn draw_dots(&self, graphics: &mut Graphics2D){
        let (position, size) = self.dot1.draw_info(self.scale_factor);
        graphics.draw_circle(position, size, Color::from_rgb(0.8, 0.9, 1.0));
        let (position, size) = self.dot2.draw_info(self.scale_factor);
        graphics.draw_circle(position, size, Color::from_rgb(1.0, 0.4, 0.8));
    }
    pub fn tick(&mut self){
        if let Some(accel1) = self.accel1_position{
            self.dot1.accelerate_tick(accel1);
        }
        if let Some(accel2) = self.accel2_position{
            self.dot2.accelerate_tick(accel2);
        }
        self.dot1.speed_tick(self.canvas_size);
        self.dot2.speed_tick(self.canvas_size);
    }
}

impl WindowHandler for Game {

    fn on_start(&mut self, _helper: &mut WindowHelper, info: WindowStartupInfo){
        self.canvas_size = *info.viewport_size_pixels();
        self.scale_factor = info.scale_factor();
        self.dot1.restart(self.canvas_size);
        self.dot2.restart(self.canvas_size);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper, size_pixels: Vector2<u32>){
        println!("{:?}", size_pixels);
        self.canvas_size = size_pixels;
        self.dot1.restart(size_pixels);
        self.dot2.restart(size_pixels);
    }
    fn on_scale_factor_changed(
        &mut self,
        helper: &mut WindowHelper,
        scale_factor: f64
    ){
        self.scale_factor = scale_factor;
    }

    fn on_mouse_button_down(
        &mut self,
        helper: &mut WindowHelper,
        button: MouseButton
    ){
        match button{
            MouseButton::Left => self.accel1_position = self.mouse_position,
            MouseButton::Right => self.accel2_position = self.mouse_position,
            _ => {}
        }

    }
    
    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper,
        button: MouseButton
    ){
        match button{
            MouseButton::Left => self.accel1_position = None,
            MouseButton::Right => self.accel2_position = None,
            _ => {}
        }
    }

    fn on_mouse_move(
        &mut self,
        helper: &mut WindowHelper,
        position: Vector2<f32>)
        {
            self.mouse_position = Some(position);
        }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.tick();
        graphics.clear_screen(speedy2d::color::Color::BLACK);
        // let text = prep_label(
        //     &self.font,
        //     &std::format!(
        //         "Frame: {}, FPS: {:.2}, frame draw time: {:.2} µs",
        //         self.counter,
        //         self.fps,
        //         self.draw_time
        //     ),
        // );
        // graphics.draw_text((0.0, 0.0), speedy2d::color::Color::RED, &text);
        self.draw_dots(graphics);
        helper.request_redraw();
    }
}
