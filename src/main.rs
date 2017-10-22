#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate c_vec;
extern crate rocket;
extern crate rocket_contrib;
extern crate sdl2;
extern crate url;

mod url_open;
mod math;

/*
 * #![recursion_limit = "1024"]
 *
 * #[macro_use]
 * extern crate error_chain;
 *
 * mod errors {
 *     error_chain!{}
 *     }
 *
 *     use errors::*;
 *     */


use sdl2::pixels;
use sdl2::pixels::Color;
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
use sdl2::render::Canvas;

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

trait FieldConstants {
    const XMIN: i16 = 100;
    const XMAX: i16 = 700;
    const YMIN: i16 = 100;
    const YMAX: i16 = 500;
}

struct FieldState {}

impl FieldConstants for FieldState {}

trait PaddleConstants {
    const HEIGHT: i16 = 100;
    const WIDTH: i16 = 10;
    const YMIN: i16 = FieldState::YMIN;
    const YMAX: i16 = FieldState::YMAX - PaddleState::HEIGHT;
}

struct PaddleState {
    y: i16,
    dy: i16
}

impl PaddleConstants for PaddleState {}

trait BallConstants {
    const HEIGHT: i16 = 30;
    const WIDTH: i16 = 30;
}

struct BallState {
    color: Color,
    erase_color: Color,
    x: i16,
    y: i16,
    dx: i16,
    dy: i16
}

impl BallConstants for BallState {}

struct GameState {
    paddle: PaddleState,
    ball: BallState 
}

fn render() {
}

fn draw_ball(canvas: &Canvas<sdl2::video::Window>, cx: i16, cy: i16, color: &pixels::Color) {
    // TODO: put in game constants struct...
    let radius = 50.0;
    let mut sector = 0.0;
    // draw ball as set of triangles...
    // TODO: make this a bitmap and blt it later...
    // maybe have a spin animation?
    for _ in 0..36 {
        let angle1 : f64 = sector;
        sector += 180.0 / 18.0 * 3.141592 / 180.0;
        let angle2 : f64 = sector;
        let x1 = cx + (radius * angle1.cos()) as i16;
        let y1 = cy + (radius * angle1.sin()) as i16;
        let x2 = cx + (radius * angle2.cos()) as i16;
        let y2 = cy + (radius * angle2.sin()) as i16;
        canvas.filled_trigon(x1, y1, cx, cy, x2, y2, *color).unwrap();
    }
}

fn update_game(state: &mut GameState) {
    let new_ball_x = state.ball.x + state.ball.dx;

    if new_ball_x >= FieldState::XMIN && new_ball_x <= FieldState::XMAX {
        state.ball.x = new_ball_x;
    }
    
    let new_ball_y = state.ball.y + state.ball.dy;
    if new_ball_y >= FieldState::YMIN && new_ball_y <= FieldState::YMAX {
        state.ball.y = new_ball_y;
    }
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

    let mut game_state = GameState {
        paddle: PaddleState {
            y: 0,
            dy: 0,
        },
        ball:  BallState {
            color: pixels::Color::RGB(255, 255, 255),
            erase_color: pixels::Color::RGB(0, 0, 0),
            x: 100,
            y: 100,
            dx: 1,
            dy: 0
        }
    };

    //const let red = pixels::Color::RGB(255, 0, 0);
    //const let blue = pixels::Color::RGB(0, 0, 255);
    //const let black = pixels::Color::RGB(0, 0, 0);

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

        let cx = 400;
        let cy = 300;
        let radius = 300.0;
        let mut sector = 0.0;

        let cx = game_state.ball.x;
        let cy = game_state.ball.y;

        draw_ball(&canvas, game_state.ball.x, game_state.ball.y, &game_state.ball.erase_color);

        update_game(&mut game_state);

        draw_ball(&canvas, game_state.ball.x, game_state.ball.y, &game_state.ball.color);

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
        /*  TODO: do this for ball later...
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
        */
