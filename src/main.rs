extern crate sdl2;
mod tdutil;
use crate::tdutil::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_sys = sdl_context.video().unwrap();

    let window = video_sys.window("SDL Window", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut t1 = Tri::new(Point::new(-100., 100., 2.0),
                        Point::new(-100., -100., 1.0),
                        Point::new(100., -100., 1.0));

    let mut t2 = Tri::new(Point::new(-100., 100., 2.0),
                        Point::new(100., 100., 1.0),
                        Point::new(100., -100., 1.0));
    
    let mut geo = Geometry::new(Point::new(0.0,0.0,0.0));
    geo.add_tri(&t1);
    geo.add_tri(&t2);
    let mut geo = vec![geo];


    let camera = Camera::new(Point::new(0., 0., 0.), rect::Point::new(800, 600), 90);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        camera.render(&mut canvas, &geo);

        // let p = t.project(camera);
        // canvas.draw_line(p[0], p[1]);
        // canvas.draw_line(p[1], p[2]);
        // canvas.draw_line(p[2], p[0]);

        // geo[0].rot_z(0.01);
        // geo[0].rot_y(0.005);


        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Quiting...");
}