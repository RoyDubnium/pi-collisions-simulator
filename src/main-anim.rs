use ggez::{
    audio::{SoundSource, Source},
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, Mesh, Rect},
    Context, ContextBuilder, GameResult,
};
use eq_float::F64;
use aatree::AATreeMap;
use std::time::{Duration, Instant};
mod collisions;

/// Main state of the animation.
struct MainState {
    /// Timeline: keys are times (wrapped in F64) and values are snapshots:
    /// ((p1, v1), (p2, v2))
    timeline: AATreeMap<F64, ((f64, f64), (f64, f64))>,
    start_time: Instant,
    /// Customizable normalized size for object 2.
    p2_size: f64,
    window_width: f32,
    window_height: f32,
}

impl MainState {
    /// Creates a new state and populates some sample snapshots.
    fn new(ctx: &mut Context, timeline: AATreeMap<F64, ((f64, f64), (f64, f64))>) -> GameResult<MainState> {
        let snapshot_sound = Source::new(ctx, "/clack.wav")?;
        let (window_width, window_height) = (800.0, 600.0);
        Ok(MainState {
            timeline,
            start_time: Instant::now(),
            p2_size: 0.15, // Customizable normalized size for object 2.
            window_width,
            window_height,
        })
    }

    /// Uses AATreeMap’s last_key_value_at_or_before to get the latest snapshot at or before the given time.
    fn get_latest_snapshot(&self, current_time: f64) -> Option<(f64, ((f64, f64), (f64, f64)))> {
        self.timeline
            .last_key_value_at_or_before(&F64(current_time))
            .map(|(time, snapshot)| (time.0, *snapshot))
    }

    /// Given a snapshot and its timestamp, compute the extrapolated positions at the current time.
    /// It applies a simple linear update: position += velocity * (current_time - snapshot_time)
    fn interpolate_positions(
        snapshot: ((f64, f64), (f64, f64)),
        snapshot_time: f64,
        current_time: f64,
    ) -> (f64, f64) {
        let dt = current_time - snapshot_time;
        // Unpack snapshot: ((p1, v1), (p2, v2))
        let ((p1, v1),(p2, v2)) = snapshot;
        let new_p1 = p1 + v1 * dt;
        let new_p2 = p2 + v2 * dt;
        (new_p1, new_p2)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen.
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Calculate elapsed time in seconds since the animation started.
        let elapsed = self.start_time.elapsed();
        let current_time = elapsed.as_secs_f64()/20.0;

        // Retrieve the latest collision at or before the current time.
        if let Some((snapshot_time, snapshot)) = self.get_latest_snapshot(current_time) {
            let (pos1, pos2) =
                MainState::interpolate_positions(snapshot, snapshot_time, current_time);

            // Convert normalized positions to pixel coordinates.
            let scale_factor = 2.0;
            let square1_size = 0.1 * self.window_width * scale_factor;
            let square2_size = (self.p2_size as f32) * self.window_width * scale_factor;

            let pos1_pixel = [
                (pos1 as f32 * self.window_width * scale_factor),
                (0.8_f32 * self.window_height) - (square1_size),
            ];
            let pos2_pixel = [
                (pos2 as f32 * self.window_width * scale_factor),
                (0.8_f32 * self.window_height) - (square2_size),
            ];

            // Create square meshes for the objects.
            let square1 = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(pos1_pixel[0], pos1_pixel[1], square1_size, square1_size),
                Color::from_rgb(255, 0, 0),
            )?;
            let square2 = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(pos2_pixel[0], pos2_pixel[1], square2_size, square2_size),
                Color::from_rgb(0, 255, 0),
            )?;

            // Draw both squares.
            canvas.draw(&square1, graphics::DrawParam::default());
            canvas.draw(&square2, graphics::DrawParam::default());
        }

        // Present the current frame.
        canvas.finish(ctx)?;

        Ok(())
    }
}
use std::env;
pub fn main() -> GameResult {
    // Build the context and event loop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("animation", "Author")
        .window_setup(WindowSetup::default().title("Animation"))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Create the main state.
    let args: Vec<String> = env::args().collect();
    let mut n : i32 = 3;
    if args.len() > 1
    {
        n = match args[args.len()-1].to_string().parse::<i32>()
        {
            Ok(number) => number,
            Err(_) => 3
        };
    }
    let timeline : AATreeMap<F64,((f64,f64),(f64,f64))> = collisions::getcollisions(n);
    let state = MainState::new(&mut ctx, timeline)?;
    // Run the main event loop.
    event::run(ctx, event_loop, state)
}