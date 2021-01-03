use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::*;
use tetra::graphics::{self, Camera, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, Event, State};

mod chess {
    // // TODO handle parsing of multi digit numbers
    // fn to_map_coordinates(chess_coords: &str) -> (i32, i32) {
    //     let row: Vec<&str> = chess_coords
    //         .matches(char::is_numeric)
    //         .collect()
    //         .nth(0)
    //         .parse::<i32>()
    //         .unwrap();
    //     let col: Vec<&str> = chess_coords.matches(char::is_alphabetic).collect();
    //     println!("split: {:?} {:?}", v1, v2);
    //     (1, 1)
    // }

    // #[test]
    // fn test_to_map_coordinates() {
    //     assert_eq!(to_map_coordinates("ZZ1"), (701, 0));
    //     assert_eq!(to_map_coordinates("ZZ11"), (701, 10));
    //     assert_eq!(to_map_coordinates("A1"), (0, 0));
    //     assert_eq!(to_map_coordinates("A5"), (0, 4));
    //     assert_eq!(to_map_coordinates("B2"), (1, 1));
    //     assert_eq!(to_map_coordinates("Z1"), (25, 0));
    //     assert_eq!(to_map_coordinates("AA1"), (26, 0));
    //     assert_eq!(to_map_coordinates("AB1"), (27, 0));
    //     assert_eq!(to_map_coordinates("YZ1"), (675, 0));
    //     assert_eq!(to_map_coordinates("ZA1"), (676, 0));
    //     assert_eq!(to_map_coordinates("ZB1"), (677, 0));
    // }

    pub fn from_map_coordinates(column: i32, row: i32) -> String {
        if column >= (27 * 26) {
            panic!("parameter column out of range [0..{}): {}", 27 * 26, column);
        }
        let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let first_c: usize = (column / 26) as usize;
        let second_c: usize = (column % 26) as usize;

        let s: String = format!(
            "{}{}{}",
            if column >= 26 {
                alphabet[first_c - 1].into()
            } else {
                ' '
            },
            alphabet[second_c],
            row + 1,
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
const MOVEMENT_SPEED: f32 = 8.0;
const ZOOM_SPEED: f32 = 0.1;

struct Battlemap {
    tile_canvas: graphics::Canvas,
    texture: Texture,
    rows: i32,
    columns: i32,
}

impl Battlemap {
    fn new(ctx: &mut Context, image_path: &str, rows: i32, columns: i32) -> Self {
        let texture = Texture::new(ctx, image_path)
            .expect(format!("Can't read file {:?}", image_path).as_str());
        let tile_canvas = Self::new_tile_canvas(rows, columns, &texture, ctx)
            .expect("Failed to create tile canvas.");
        Battlemap {
            tile_canvas,
            texture,
            rows,
            columns,
        }
    }

    fn render(&mut self, ctx: &mut Context) {
        // Now all drawing operations will be transformed:
        graphics::draw(ctx, &self.texture, DrawParams::new());
        self.render_grid(ctx)
    }

    fn grid_size(&self) -> (i32, i32) {
        (
            (self.texture.width() as f64 / self.columns as f64).round() as i32,
            (self.texture.height() as f64 / self.rows as f64).round() as i32,
        )
    }

    fn new_tile_canvas(
        rows: i32,
        columns: i32,
        texture: &Texture,
        ctx: &mut Context,
    ) -> tetra::Result<graphics::Canvas> {
        let tile_w = (texture.width() as f64 / columns as f64).round() as i32;
        let tile_h = (texture.height() as f64 / rows as f64).round() as i32;

        let edge_width = 3;

        // right edge
        let c = graphics::Canvas::new(ctx, tile_w, tile_h)?;
        let data = vec![128 as u8; (4 * tile_h * edge_width) as usize];
        let (x, y, width, height) = (tile_w - edge_width, 0, edge_width, tile_h);
        c.set_data(ctx, x, y, width, height, &*data)?;

        // bottom edge
        let data = vec![128 as u8; (4 * tile_w * edge_width) as usize];
        let (x, y, width, height) = (0, tile_h - edge_width, tile_w, edge_width);
        c.set_data(ctx, x, y, width, height, &*data)?;
        Ok(c)
    }

    fn render_grid(&mut self, ctx: &mut Context) {
        let (tile_w, tile_h) = self.grid_size();
        // TODO cache assests in global game state
        let font = Font::vector(ctx, "./assets/SourceCodePro-Black.ttf", 32.0)
            .expect("Failed to load font.");
        for col in 0..self.columns {
            for row in 0..self.rows {
                let pos = Vec2::new((col * tile_w) as f32, (row * tile_h) as f32);
                let text = Text::new(chess::from_map_coordinates(col, row), font.clone());

                graphics::draw(
                    ctx,
                    &text,
                    DrawParams::default()
                        .position(pos)
                        .color(Color::rgba(0.0, 0.0, 0.0, 0.25)),
                );
                graphics::draw(ctx, &self.tile_canvas, DrawParams::default().position(pos));
            }
        }
    }
}

struct GameState {
    scaler: ScreenScaler,
    camera: Camera,
    text: Text,
    battlemap: Battlemap,
}
// use std::str::FromStr;
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let args: Vec<String> = ::std::env::args().collect();
        let dm_mode = args.contains(&String::from("--dm"));
        let _file = &args
            .iter()
            .nth(1)
            .expect("first arg need to be a game file name");
        println!("{:?}", args);

        let text = Text::new(
            if dm_mode { "DM Mode" } else { "Player Mode" },
            Font::vector(ctx, "./assets/SourceCodePro-Black.ttf", 32.0)?,
        );

        Ok(GameState {
            text,
            scaler: ScreenScaler::with_window_size(ctx, 2048, 1920, ScalingMode::CropPixelPerfect)?,
            camera: Camera::new(2048.0, 1920.0),
            battlemap: Battlemap::new(ctx, "./assets/background.jpg", 10, 20),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) || input::is_key_down(ctx, Key::Up) {
            self.camera.position.y -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
            self.camera.position.y += MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
            self.camera.position.x -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
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

        self.battlemap.render(ctx);

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
