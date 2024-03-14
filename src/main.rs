extern crate sdl2;
mod tdutil;
use crate::tdutil::*;

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect};
use std::time::Duration;

fn main() {

    let mut _frameCount: f32 = 0.0;

    let sdl_context = sdl2::init().unwrap();
    let video_sys = sdl_context.video().unwrap();

    let window = video_sys.window("SDL Window", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut geo = Geometry::new();
    
    let mut tris = Vec::<Tri>::new();
    let mut verts = Vec::<Vec3>::new();

    {
        let s = 100.0;

        for i in 0..8 {
            verts.push(Vec3::new(
                if (i & 4) != 0 {s} else {-s},
                if (i & 2) != 0 {s} else {-s},
                if (i & 1) != 0{s} else {-s}
            )
            )
        }

        for i in 0..3 {
            let v1 = 1 << i;
            let v2 = if v1 == 4 {1} else {v1 << 1};
            tris.push(Tri::new(*verts.get(0).unwrap(), *verts.get(v1).unwrap(), *verts.get(v2).unwrap()));
            tris.push(Tri::new(*verts.get(v1 + v2).unwrap(), *verts.get(v2).unwrap(), *verts.get(v1).unwrap()));
            tris.push(Tri::new(*verts.get(7).unwrap(), *verts.get(7 - v2).unwrap(), *verts.get(7 - v1).unwrap()));
            tris.push(Tri::new(*verts.get(7 - (v1 + v2)).unwrap(), *verts.get(7 - v1).unwrap(), *verts.get(7 - v2).unwrap()));
        }
    }

    geo.add_tris(tris);

    geo.transform.translate(Vec3::new(0.,0.,1000.));

    let mut geo = vec![geo];

    let mut camera = Camera::new( rect::Point::new(800, 600), 70.0);

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

        geo.get_mut(0).unwrap().transform.set_pos(Vec3::new(0.,(_frameCount / 10.).sin() * 100., 1000.));


        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        geo.get_mut(0).unwrap().transform.rot_z(0.02);
        geo.get_mut(0).unwrap().transform.rot_y(0.01);
        geo.get_mut(0).unwrap().transform.rot_x(0.005);


        camera.render(&mut canvas, &geo);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        _frameCount+=1.0;
    }

    // println!("{:?}", geo.get_mut(0).unwrap().transformation.zr);
}