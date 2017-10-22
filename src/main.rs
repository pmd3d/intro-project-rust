#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate c_vec;
extern crate rocket;
extern crate rocket_contrib;
extern crate sdl2;
extern crate url;

mod url_open;

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
use std::time::Duration;
//use std::fs::File;
//use std::io::Read;
use url::Url;
use url_open::UrlOpen;
use std::thread;
use rocket_contrib::Template;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;

static APP_NAME: &'static str = "Intro Project Rust";

const PI: f64 = 3.141592;

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
    const BORDERWIDTH: i16 = 10;
}

struct FieldState {
    foreground : Color,
    erase_color: Color,
}

impl FieldConstants for FieldState {}

trait PaddleConstants {
    const HEIGHT: i16 = 100;
    const WIDTH: i16 = 10;
    const YMIN: i16 = FieldState::YMIN + 1;
    const YMAX: i16 = FieldState::YMAX - PaddleState::HEIGHT - 1;
    const LEFT: i16 = FieldState::XMIN + 4 * PaddleState::WIDTH; 
    const RIGHT: i16 = FieldState::XMAX - 5 * PaddleState::WIDTH;
    const DELTA: i16 = 10;
}

struct PaddleState {
    y: i16,
    dy: i16
}

impl PaddleConstants for PaddleState {}

trait BallConstants {
    const RADIUS: f64 = 10.0;
    const XMIN: i16 = FieldState::XMIN + 10;
    const XMAX: i16 = FieldState::XMAX - 10;
    const YMAX: i16 = FieldState::YMAX - 10;
    const YMIN: i16 = FieldState::YMIN + 10;
    const INITIALDX: f64 = 1.0;
    const INITIALDY: f64 = 1.0;
    const ACCELERATION: f64 = 2.0;
}

struct BallState {
    x: i16,
    y: i16,
    dx: f64, 
    dy: f64
}

impl BallConstants for BallState {}

struct GameState {
    paddle: [PaddleState; 2],
    ball: BallState 
}

fn draw_ball(canvas: &Canvas<sdl2::video::Window>, ball: &BallState, color: &pixels::Color) {
    let cx = ball.x;
    let cy = ball.y;
    let mut sector = 0.0;
    // draw ball as set of triangles...
    // TODO: make this a bitmap and blt it later...
    // maybe have a spin animation?
    for _ in 0..36 {
        let angle1 : f64 = sector;
        sector += 180.0 / 18.0 * PI / 180.0;
        let angle2 : f64 = sector;
        let x1 = cx + (BallState::RADIUS * angle1.cos()) as i16;
        let y1 = cy + (BallState::RADIUS * angle1.sin()) as i16;
        let x2 = cx + (BallState::RADIUS * angle2.cos()) as i16;
        let y2 = cy + (BallState::RADIUS * angle2.sin()) as i16;
        canvas.filled_trigon(x1, y1, cx, cy, x2, y2, *color).unwrap();
    }
}

fn draw_paddle(canvas: &Canvas<sdl2::video::Window>, x:i16, paddle: &PaddleState, color: &pixels::Color) {
    canvas.box_(x, paddle.y, x+PaddleState::WIDTH, paddle.y+PaddleState::HEIGHT, *color).unwrap();
}

fn draw_field(canvas: &Canvas<sdl2::video::Window>, field: &FieldState) {
    let lx = FieldState::XMIN - FieldState::BORDERWIDTH;
    let rx = FieldState::XMAX + FieldState::BORDERWIDTH;
    let uy = FieldState::YMIN - FieldState::BORDERWIDTH;
    let by = FieldState::YMAX + FieldState::BORDERWIDTH;

    canvas.box_(lx, uy, rx, FieldState::YMIN, field.foreground).unwrap();
    canvas.box_(FieldState::XMAX, uy, rx, by, field.foreground).unwrap();
    canvas.box_(lx, FieldState::YMAX, rx, by, field.foreground).unwrap();
    canvas.box_(lx, uy, FieldState::XMIN, by, field.foreground).unwrap();
}

fn update_game(state: &GameState, new_paddle_dy : i16) -> GameState {
    let new_ball_x = state.ball.x + state.ball.dx as i16;
    let new_ball_y = state.ball.y + state.ball.dy as i16;

    let new_paddle_y = state.paddle[0].y + state.paddle[0].dy;
    // todo: fix so LEFT and RIGHT are enumerable...
    let collision: [bool; 2] = [
        new_ball_x >= PaddleState::LEFT && new_ball_x <= PaddleState::LEFT + PaddleState::WIDTH &&
                          new_ball_y >= state.paddle[0].y && new_ball_y <= state.paddle[0].y + PaddleState::HEIGHT,
        new_ball_x >= PaddleState::RIGHT && new_ball_x <= PaddleState::RIGHT + PaddleState::WIDTH &&
                          new_ball_y >= state.paddle[1].y && new_ball_y <= state.paddle[1].y + PaddleState::HEIGHT];

    GameState {
        paddle: [
            PaddleState {
                y: if new_paddle_y >= PaddleState::YMIN && new_paddle_y <= PaddleState::YMAX {
                       new_paddle_y
                   } else {
                       state.paddle[0].y
                   },
                dy: if (state.paddle[0].dy == 0 || state.paddle[0].dy == new_paddle_dy) &&
                       new_paddle_y >= PaddleState::YMIN && new_paddle_y <= PaddleState::YMAX {
                       new_paddle_dy
                    } else {
                       0
                    }
            }, 
            PaddleState {
                dy: state.paddle[1].dy,
                y: state.paddle[1].y
            }],
        ball: BallState {
            x : if new_ball_x >= BallState::XMIN && new_ball_x <= BallState::XMAX {
                    new_ball_x
                } else {
                    state.ball.x
                },
            y : if new_ball_y >= BallState::YMIN && new_ball_y <= BallState::YMAX {
                    new_ball_y
                } else {
                    state.ball.y
                },
            dx: if collision[0] || collision[1] {
                    -state.ball.dx * BallState::ACCELERATION
                }
                else if new_ball_x == BallState::XMIN || new_ball_x == BallState::XMAX {
                    -state.ball.dx
                } else {
                     state.ball.dx
                },
            dy: if collision[0] || collision[1] { 
                     -state.ball.dy * BallState::ACCELERATION
                }
                else if new_ball_y == BallState::YMIN || new_ball_y == BallState::YMAX {
                     -state.ball.dy
                } else {
                     state.ball.dy
                }
            // todo: ball hits side of paddle...
            // todo: bug, log the dy. ball gets stuck on bottom.
        }
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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = GameState {
        paddle: [
            PaddleState {
                y: FieldState::YMAX/2,
                dy: 0 },
            PaddleState {
                y: FieldState::YMAX/2,
                dy: 0 }
        ],
        ball:  BallState {
            x: FieldState::XMAX/2,
            y: FieldState::YMAX/2,
            dx: BallState::INITIALDX,
            dy: BallState::INITIALDY
        }
    };

    let mut new_paddle_dy = 0;

    let field = FieldState {
        foreground: pixels::Color::RGB(255, 255, 255),
        erase_color: pixels::Color::RGB(0, 0, 0)
    };

// todo: maybe send over the entire event with timestamp?
// todo: would be nice if could control with just two keys

    draw_field(&canvas, &field);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => new_paddle_dy = PaddleState::DELTA,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => new_paddle_dy = -PaddleState::DELTA,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => new_paddle_dy = 0,
                _ => {}
            }
        }
        // TODO: adjust this for frame rate...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        let _counter = timer.ticks();

        draw_ball(&canvas, &game_state.ball, &field.erase_color);
        draw_paddle(&canvas, PaddleState::LEFT, &game_state.paddle[0], &field.erase_color);
        draw_paddle(&canvas, PaddleState::RIGHT, &game_state.paddle[1], &field.erase_color);

        game_state = update_game(&game_state, new_paddle_dy);

        draw_ball(&canvas, &game_state.ball, &field.foreground);
        draw_paddle(&canvas, PaddleState::LEFT, &game_state.paddle[0], &field.foreground);
        draw_paddle(&canvas, PaddleState::RIGHT, &game_state.paddle[1], &field.foreground);

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

    // TODO: this needs to change in case port is taken...
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
