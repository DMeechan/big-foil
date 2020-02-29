#![windows_subsystem = "windows"]
#![warn(bare_trait_objects)]

#[cfg(not(target_arch = "wasm32"))]
extern crate ggez;
#[cfg(target_arch = "wasm32")]
extern crate good_web_game as ggez;

extern crate nalgebra;

use std::path::Path;
// use rand::{thread_rng, Rng};
// use nalgebra::{Point2, Vector2};
use ggez::{
    // conf,
    timer,
    event::{self, EventHandler},//, KeyCode, KeyMods},
    graphics,
    Context, ContextBuilder, GameResult,
    mint::{Point2}
};

// /// Create a unit vector representing the
// /// given angle (in radians)
// fn vec_from_angle(angle: f32) -> Vector2 {
//     let vx = angle.sin();
//     let vy = angle.cos();
//     Vector2::new(vx, vy)
// }

// // Actors: anything in the game world
// #[derive(Debug)]
// enum ActorType {
//     Continent
// }

// #[derive(Debug)]
// struct Actor {
//     tag: ActorType,
//     pos: Point2,
//     facing: f32,
//     velocity: Vector2,
//     ang_vel: f32,
// }

// // Constants
// const MAX_ROCK_VELOCITY: f32 = 50.0;
const ASSETS_DIR_NAME: &str = "assets";

// // Constructors for different game objects
// fn create_continent() -> Actor {
//     Actor {
//         tag: ActorType::Continent,
//         pos: Point2::origin(),
//         facing: 0.0,
//         velocity: na::zero(),
//         ang_vel: 0.0,
//     }
// }

// Create the continents
// fn create_continents(num: i32, ) -> Vec<Actor> {
//     let new_continent = |_| {
//         let mut continent = create_continent();
//         continent.pos = thread_rng().gen_range(0.0, 500.0);
//         continent
//     }

//     (0..num).map(new_continent).collect();
// }

// Creating a gamestate depends on having an SDL context to load resources.
// Creating a context depends on loading a config file.
// Loading a config file depends on having FS (or we can just fake our way around it
// by creating an FS and then throwing it away; the costs are not huge.)
fn main() {
    println!("Hello, world!");

    // let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR")

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("foil", "🔥 Big Foil")
        .add_resource_path(ASSETS_DIR_NAME)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut game = MainState::new(&mut ctx).unwrap();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

/// Game state here
struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState>{
        // Load/create resources such as images here.
        let image = graphics::Image::new(ctx, "/tile.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let state = MainState {
            spritebatch: batch
        };

        Ok(state)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Average FPS: {} | Delta time frame: {:?}", timer::fps(ctx), timer::delta(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 { x: 200.0, y: 300.0 },
            100.0,
            0.1,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

        let image = graphics::Image::new(ctx, "/tile.png").unwrap();
        graphics::draw(ctx, &image, graphics::DrawParam::default())?;

        // let font = graphics::Font::new(ctx, "blah.ttf")?;

        // Draw code here...
        graphics::present(ctx)
    }
}
