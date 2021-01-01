use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::*;
use tetra::graphics::{self, Camera, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, Event, State};

const MOVEMENT_SPEED: f32 = 4.0;
const ROTATION_SPEED: f32 = 0.1;
const ZOOM_SPEED: f32 = 0.1;

struct GameState {
    texture: Texture,
    scaler: ScreenScaler,
    camera: Camera,
    text: Text,
}
use std::str::FromStr;
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let args: Vec<String> = ::std::env::args().collect();
        let dm_mode = args.contains(&String::from("--dm"));
        let file = &args[1];
        println!("{:?}", args);

        let text = Text::new(
            if dm_mode { "DM Mode" } else { "Player Mode" },
            Font::vector(ctx, "./assets/SourceCodePro-Black.ttf", 32.0)?,
        );

        Ok(GameState {
            text: text,
            texture: Texture::new(ctx, "./assets/background.jpg")?,
            scaler: ScreenScaler::with_window_size(ctx, 2048, 1920, ScalingMode::CropPixelPerfect)?,

            camera: Camera::new(2048.0, 1920.0),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.camera.position.y -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.camera.position.y += MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::A) {
            self.camera.position.x -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::D) {
            self.camera.position.x += MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::R) || input::is_mouse_scrolled_up(ctx) {
            self.camera.zoom += ZOOM_SPEED;
        }

        if input::is_key_down(ctx, Key::F) || input::is_mouse_scrolled_down(ctx) {
            self.camera.zoom -= ZOOM_SPEED;
        }

        self.camera.update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        // To 'look through' the camera, we pass the calculated transform matrix
        // into the renderer:
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        // Now all drawing operations will be transformed:
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new(),
            // .origin(Vec2::new(1.0, 1.0))
            // .scale(Vec2::new(1.0, 1.0)),
        );

        // If you want to go back to drawing without transformations, reset the
        // matrix. This is important here, as we're going to draw more stuff
        // this frame, which we don't want to transform:
        graphics::reset_transform_matrix(ctx);
        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);

        graphics::draw(ctx, &self.scaler, Vec2::zero());
        graphics::draw(
            ctx,
            &self.text,
            DrawParams::default()
                .color(graphics::Color::BLUE)
                .position(Vec2::new(16.0, 16.0)),
        );
        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Cameras", 640, 480)
        .resizable(true)
        .show_mouse(true)
        .quit_on_escape(true)
        .maximized(true)
        .build()?
        .run(GameState::new)
}
