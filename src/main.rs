use ggez::event::{self, EventHandler};
use ggez::graphics::*;
use ggez::{graphics, Context, ContextBuilder, GameResult};

fn main() {
    println!("Hello, world!");

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("foil", "🔥 Big Foil")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut game = GameState::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct GameState {
    // Your state here...
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        // Load/create resources such as images here.
        GameState {
            // ...
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            mint::Point2 { x: 200.0, y: 300.0 },
            100.0,
            0.1,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

        // Draw code here...
        graphics::present(ctx)
    }
}
