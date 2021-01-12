use crate::chess;
use tetra::graphics::text::Font;
use tetra::graphics::text::Text;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::math::Vec2;
use tetra::Context;

const EDGE_WIDTH: i32 = 3;
const GRID_COORD_COL: Color = Color::rgba(0.0, 0.0, 0.0, 0.25);

pub struct Battlemap {
    tile_canvas: graphics::Canvas,
    pub image_path: String,
    pub texture: Texture,
    pub rows: i32,
    pub columns: i32,
}

impl Battlemap {
    pub fn new(ctx: &mut Context, image_path: String, rows: i32, columns: i32) -> Self {
        let texture =
            Texture::new(ctx, image_path.to_owned()).expect(format!("Can't read file.").as_str());
        let tile_canvas = Self::new_tile_canvas(rows, columns, &texture, ctx)
            .expect("Failed to create tile canvas.");
        Battlemap {
            image_path: image_path.to_owned(),
            tile_canvas,
            texture,
            rows,
            columns,
        }
    }

    pub fn render(&mut self, ctx: &mut Context) {
        // Now all drawing operations will be transformed:
        graphics::draw(ctx, &self.texture, DrawParams::new());
        self.render_grid(ctx)
    }

    pub fn grid_size(&self) -> (i32, i32) {
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

        // right edge
        let c = graphics::Canvas::new(ctx, tile_w, tile_h)?;
        let data = vec![128 as u8; (4 * tile_h * EDGE_WIDTH) as usize];
        let (x, y, width, height) = (tile_w - EDGE_WIDTH, 0, EDGE_WIDTH, tile_h);
        c.set_data(ctx, x, y, width, height, &*data)?;

        // bottom edge
        let data = vec![128 as u8; (4 * tile_w * EDGE_WIDTH) as usize];
        let (x, y, width, height) = (0, tile_h - EDGE_WIDTH, tile_w, EDGE_WIDTH);
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
                    DrawParams::default().position(pos).color(GRID_COORD_COL),
                );
                graphics::draw(ctx, &self.tile_canvas, DrawParams::default().position(pos));
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Token {
    pub id: String,
    pub image: String,
    pub name: String,
    pub size: String,
    pub max_health: i32,
    pub pos: String,
    pub initiative: i32,

    pub texture: Texture,
}

impl Token {
    pub fn new(ctx: &mut Context, id: String, image: String, name: String, size: String, max_health: i32, pos: String, initiative: i32) -> Self {
        let texture =
            Texture::new(ctx, image.to_owned()).expect(format!("Can't read file.").as_str());

        Self{
            id, image, name, size, max_health, pos, initiative, 
            texture,
        }
    }

    pub fn render(&self, ctx: &mut Context, bm: &Battlemap) {
        let w = self.texture.width() as f32;
        let h = self.texture.height() as f32;

        let (col,row) = (1, 0); // A2 TODO

        let (tile_w, tile_h) = bm.grid_size();

        let pos = Vec2::new((col * tile_w) as f32, (row * tile_h) as f32);

        let uniscale = (tile_w as f32 / w).min(tile_h as f32 / h);
        let scale = Vec2::new(uniscale, uniscale);
        graphics::draw(ctx, &self.texture, DrawParams::default().position(pos).scale(scale));
    }
}