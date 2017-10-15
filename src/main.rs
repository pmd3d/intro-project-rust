#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate sdl2;
extern crate url;
extern crate c_vec;

mod url_open;

use sdl2::pixels;
//use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;
use std::path::Path;
//use std::fs::File;
//use std::io::Read;
use url::Url;
use url_open::UrlOpen;
use std::thread;
use rocket_contrib::Template;
use sdl2::gfx::primitives::DrawRenderer;

static APP_NAME: &'static str = "Intro Project Rust";


#[get("/")]
fn index() -> Template {
    let context = "todo";
    Template::render("index", &context)
}

#[get("/start")]
fn start() -> Template {
    let context = "todo";
    start_game();
    Template::render("index", &context)
}

fn start_game() {
    let sdl_context = sdl2::init().unwrap();

    let mut timer = sdl_context.timer().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(APP_NAME, 800, 600)
        .fullscreen()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("./assets/animate.bmp")).unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .unwrap();

    let source_rect = Rect::new(0, 0, 128, 82);
    let mut dest_rect = Rect::new(0, 0, 128, 82);

    let center = Point::new(320, 240);
    dest_rect.center_on(center);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        let counter = timer.ticks();

        dest_rect.set_x(((counter / 100) % 100) as i32);

        canvas
            .copy_ex(
                &texture,
                Some(source_rect),
                Some(dest_rect),
                10.0,
                None,
                true,
                false,
            )
            .unwrap();
/*
       canvas.trigonRGBA(
            400,0, 700,400, 0,400, 255, 0, 0, 0)
           .unwrap();
        canvas.line(0, 0, 700, 500, color).unwrap();
        let fill = pixels::Color::RGB(0, 0, 100);
        //canvas.filled_trigon(1, 1, 699, 1, 699, 499, fill).unwrap();
*/
        let red = pixels::Color::RGB(255, 0, 0);
        let blue = pixels::Color::RGB(0, 0, 255);
        let cx = 400;
        let cy = 300;
        let radius = 300.0;
        let mut sector = 0.0;
        // centerx, centery, radius, start angle, end angle
        // canvas.arc(cx, cy, radius as i16, 0, 180, red).unwrap();
        for _ in 0..36 {
            let angle1 : f64 = sector;
            sector += 180.0 / 18.0 * 3.141592 / 180.0;
            let angle2 : f64 = sector;
            // trig
            let x1 = cx + (radius * angle1.cos()) as i16;
            let y1 = cy + (radius * angle1.sin()) as i16;
            let x2 = cx + (radius * angle2.cos()) as i16;
            let y2 = cy + (radius * angle2.sin()) as i16;
            canvas.filled_trigon(x1, y1, cx, cy, x2, y2, blue).unwrap();
        }
        canvas.present();
    }
}


pub fn main() {
    thread::spawn(move || {
        rocket::ignite()
            .mount("/", routes![index, start])
            .attach(Template::fairing())
            .launch()
    });

    // TODO: this need to change in case port is taken...
    // more error handling too.
    Url::parse("http://127.0.0.1:8000/").unwrap().open();

    start_game();
}
