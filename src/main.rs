use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::*;
use tetra::graphics::{self, Camera, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, Event, State};

mod chess {
    fn from_map_coordinates(column: i32, row: i32) -> String {
        if column >= (27 * 26) {
            panic!("parameter column out of range [0..{}): {}", 27 * 26, column);
        }
        let alphabet : Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let first_c : usize = (column / 26) as usize;
        let second_c : usize = (column % 26) as usize;

        let s : String = format!("{}{}{}", 
            if column >= 26 { alphabet[first_c-1].into() } else { ' ' },
            alphabet[second_c],
            row+1,
        );
        s.trim().to_string()
    }

    #[test]
    fn test_chess_coordinates() {
        assert_eq!("A1", from_map_coordinates(0, 0));
        assert_eq!("A5", from_map_coordinates(0, 4));
        assert_eq!("B2", from_map_coordinates(1, 1));
        assert_eq!("Z1", from_map_coordinates(25, 0));
        assert_eq!("AA1", from_map_coordinates(26, 0));
        assert_eq!("AB1", from_map_coordinates(27, 0));
        assert_eq!("YZ1", from_map_coordinates(675, 0));
        assert_eq!("ZA1", from_map_coordinates(676, 0));
        assert_eq!("ZB1", from_map_coordinates(677, 0));
        assert_eq!("ZZ1", from_map_coordinates(701, 0));
        // from_map_coordinates(702, 0); // panic!
    }
}
const MOVEMENT_SPEED: f32 = 4.0;
const ZOOM_SPEED: f32 = 0.1;


struct Battlemap {
    texture: Texture,
    rows: i32,
    columns: i32,
}

impl Battlemap {
    fn render(&mut self, ctx: &mut Context) -> tetra::Result {
        // Now all drawing operations will be transformed:
        graphics::draw(ctx, &self.texture, DrawParams::new());

        let (tile_w, tile_h) = (100, 100);

        let c = graphics::Canvas::new(ctx, tile_w, tile_h)?;
        let data = [128; 4 * 100];
        c.set_data(ctx, 99, 0, 1, 100, &data)?;
        c.set_data(ctx, 0, 99, 100, 1, &data)?;

        for col in (0..self.columns) {
            for row in (0..self.rows) {
                graphics::draw(
                    ctx,
                    &c,
                    DrawParams::default()
                        .position(Vec2::new((col * tile_w) as f32, (row * tile_h) as f32)),
                );
            }
        }

        Ok(graphics::draw(ctx, &c, DrawParams::default()))
    }
}

struct GameState {
    scaler: ScreenScaler,
    camera: Camera,
    text: Text,
    battlemap: Battlemap,
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
            scaler: ScreenScaler::with_window_size(ctx, 2048, 1920, ScalingMode::CropPixelPerfect)?,

            camera: Camera::new(2048.0, 1920.0),

            battlemap: Battlemap {
                texture: Texture::new(ctx, "./assets/background.jpg")?,
                columns: 20,
                rows: 10,
            },
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

        self.battlemap.render(ctx)?;

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
