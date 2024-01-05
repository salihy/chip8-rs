use std::{env, fs};
use chip8_rs::computer::Computer;
use chip8_rs::display::Display;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::PressEvent;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    
}

impl App {
    fn render(&mut self, args: &RenderArgs, dsp: Display) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 18.0);
        self.gl.draw(args.viewport(), |c, gl| {
        
            // Clear the screen.
            clear(GREEN, gl);

            let ld = dsp.clone();
            for j in 0..32 { //0..32 
                for i in 0..64 { //0..64 
                    
                    if ld.get(i, j) {
                        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
                        let (x, y) = ((i * 20 + 60) as f64, (j * 20 + 60) as f64);

                        //println!("{} - {}", x, y);

                        let transform = c
                            .transform
                            .trans(x, y)
                            .trans(-25.0, -25.0);
            
                        // Draw a box rotating around the middle of the screen.
                        rectangle(RED, square, transform, gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let file = fs::read(args.get(1).unwrap()).expect("file not found!");
    println!("{:?}", args);

    let mut comp = Computer::new(file);
   

    // fs::write(args.get(2).unwrap(), c.dump()).expect("could not written!");

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {

            let disp = comp.display();
            app.render(&args, disp);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);

            comp.tick();
        }

        if let Some(args) = e.press_args() {
            println!("{:?}", args);
        }
    }
}