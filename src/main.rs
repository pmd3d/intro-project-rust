// todo: read in index.html
// todo: rustfmt
// todo: call start game from start link
extern crate sdl2;
extern crate iron;
extern crate router;
extern crate url;

mod url_open;

//use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::path::Path;
use url::Url;
use url_open::UrlOpen;


static APP_NAME: &'static str =  "Intro Project Rust";

pub fn main() {
    let mut router = Router::new();

    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    // TODO: this need to change in case port is taken...
    // more error handling too.
    let _server = Iron::new(router).http("127.0.0.1:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");

        if (query == "start") {
        }
        else {
           query = 
        }
        Ok(Response::with((status::Ok, *query)))
    }

    Url::parse("http://127.0.0.1:3000/").unwrap().open();

    fn start_game() {
        let sdl_context = sdl2::init().unwrap();
    
        let mut timer = sdl_context.timer().unwrap();
    
        let video_subsystem = sdl_context.video().unwrap();

       let window = video_subsystem.window(APP_NAME, 800, 600)
           .fullscreen()
           .opengl()
           .build()
           .unwrap();

       let mut canvas = window.into_canvas().build().unwrap();

       let texture_creator = canvas.texture_creator();

       let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("./assets/animate.bmp")).unwrap();

       let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

       let source_rect = Rect::new(0, 0, 128, 82);
       let mut dest_rect = Rect::new(0, 0, 128, 82);

       let center = Point::new(320,240);
       dest_rect.center_on(center);

       let mut event_pump = sdl_context.event_pump().unwrap();

       'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                   Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                       break 'running
                   },
                   _ => {}
               }
           }
           ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

           let counter = timer.ticks();

           dest_rect.set_x(((counter / 100) % 100) as i32);
  
           canvas.copy_ex(&texture, Some(source_rect), Some(dest_rect), 10.0, None, true, false).unwrap();
           canvas.present();
       }
   }
}
