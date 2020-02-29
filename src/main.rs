// #![windows_subsystem = "windows"]
// #![warn(bare_trait_objects)]

extern crate ggez;

mod imgui_wrapper;

// extern crate nalgebra;

// use rand::{thread_rng, Rng};
// use nalgebra::{Point2, Vector2};
use crate::imgui_wrapper::ImGuiWrapper;
use ggez::{
    conf,
    event::{self, EventHandler, KeyCode, KeyMods, MouseButton},
    graphics,
    nalgebra::Point2,
    timer,
    Context,
    ContextBuilder,
    GameResult,
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
    // Create a dummy window so we can get monitor scaling information
    let cb = ggez::ContextBuilder::new("", "");
    let (_ctx, events_loop) = &mut cb.build().unwrap();
    let hidpi_factor = events_loop.get_primary_monitor().get_hidpi_factor() as f32;
    println!("main hidpi_factor = {}", hidpi_factor);


    let window_mode = conf::WindowMode::default().resizable(false).dimensions(750.0, 500.0);
    let window_conf = conf::WindowSetup::default()
        .title("Big Foil")
        .icon("/tile.png");

    let (mut ctx, mut event_loop) = ContextBuilder::new("foil", "🔥 Big Foil")
        .window_mode(window_mode)
        .window_setup(window_conf)
        .add_resource_path(ASSETS_DIR_NAME)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut game = MainState::new(&mut ctx, hidpi_factor).unwrap();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

/// Game state here
struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        // Load/create resources such as images here.
        let image = graphics::Image::new(ctx, "/tile.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);

        let state = MainState { spritebatch: batch,
            pos_x: 0.0,
            imgui_wrapper,
            hidpi_factor
        };

        Ok(state)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        
        if timer::ticks(ctx) % 100 == 0 {
            println!(
                "Average FPS: {} | Delta time frame: {:?}",
                timer::fps(ctx),
                timer::delta(ctx)
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // Render game stuff
        {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2::new(self.pos_x, 300.0),
                100.0,
                0.1,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

            for x in 0..2 {
                for y in (0..100).step_by(10) {
                    let x = x as f32;
                    let y = y as f32;
                    let p = graphics::DrawParam::new().dest(Point2::new(x * 50.0, y * 30.0));
                    self.spritebatch.add(p);
                }
            }
    
            graphics::draw(ctx, &self.spritebatch, graphics::DrawParam::default())?;
            self.spritebatch.clear();
        }

        // Render game UI
        {
            self.imgui_wrapper.render(ctx, self.hidpi_factor);
        }

        // Draw code here...
        graphics::present(ctx)?;
        Ok(())
    }
}
