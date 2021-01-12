use crate::commands;
use crate::domain;

use std::fs;
use std::io;
use std::sync::mpsc::channel;
use std::thread;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::Font;
use tetra::graphics::text::Text;
use tetra::graphics::{self, Camera, Color, DrawParams};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, Event, State};

const MOVEMENT_SPEED: f32 = 8.0;
const ZOOM_SPEED: f32 = 0.1;
const BACKGROUND_COLOR: Color = Color::rgb(0.769, 0.812, 0.631);

pub struct GameState {
    msg_chan: std::sync::mpsc::Receiver<String>,
    scaler: ScreenScaler,
    camera: Camera,
    text: Text,
    battlemap: domain::Battlemap,
    token: Option<domain::Token>, // HashMap<String, domain::Token>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let args: Vec<String> = ::std::env::args().collect();
        println!("{:?}", args);
        let dm_mode = args.contains(&String::from("--dm"));
        let file_name = args
            .iter()
            .nth(1)
            .expect("first arg need to be a game file name")
            .to_owned();

        // handle command line input in a separate thread
        // and communicate with main thread via channel
        let (tx, rx) = channel();

        // cli thread
        thread::spawn(move || {
            // read file and produce messages
            // tried to have this in main thread but rust didn't like it
            let file_content = fs::read_to_string(file_name);
            println!("file_content> {:?}", file_content);

            // tread each line as if it where typed by a person
            for l in file_content.unwrap().lines() {
                tx.send(l.trim().to_owned())
                    .expect("Failed to send command to channel.");
            }

            let stdin = io::stdin();
            println!("{}", commands::HELP);

            // continuesly listen for new messages
            loop {
                let mut line_input = String::new();
                match stdin.read_line(&mut line_input) {
                    Ok(_bytes) => tx
                        .send(line_input.trim().to_owned())
                        .expect("Unable to send on channel"),
                    Err(e) => eprintln!("Error reading input: {:?}", e),
                }
            }
        });

        let text = Text::new(
            if dm_mode { "DM Mode" } else { "Player Mode" },
            Font::vector(ctx, "./assets/SourceCodePro-Black.ttf", 32.0)?,
        );

        Ok(GameState {
            msg_chan: rx,
            text,
            scaler: ScreenScaler::with_window_size(ctx, 2048, 1920, ScalingMode::CropPixelPerfect)?,
            camera: Camera::new(2048.0, 1920.0),
            battlemap: domain::Battlemap::new(ctx, "./assets/bg_placeholder.jpg".into(), 12, 20),
            token: None,
        })
    }
}

pub fn run(ctx: &mut Context, game_state: &mut GameState, cmd: &commands::Command) {
    use commands::Command::*;
    match cmd {
        Quit => std::process::exit(0),
        PrintHelp(l) => println!("Unknows command: {}\n{}", l, commands::HELP),
        Role(roller) => match roller.roll() {
            Ok(result) => println!("-> {}", result),
            Err(_) => {
                eprintln!("Can't roll this: {:?}", roller)
            }
        },
        UpdateBattlemap(ref b_map_opts) => {
            let bm = &game_state.battlemap;
            game_state.battlemap = domain::Battlemap::new(
                ctx,
                match b_map_opts.url.to_owned() {
                    Some(image_path) => image_path,
                    None => bm.image_path.to_owned(),
                },
                b_map_opts.rows.unwrap_or(bm.rows),
                b_map_opts.columns.unwrap_or(bm.columns),
            );
        }
        UpdateToken(ref token_opts) => {
            // println!("{:?}", token_opts);

            let token_opts = token_opts.clone();
            let token_ref = game_state.token.as_ref();
            let new_token = domain::Token::new(
                ctx,
                token_opts.token_id,
                // token_opts.image.unwrap_or_else(|| token_ref.unwrap().image.to_owned()),
                // token_opts.name.unwrap_or_else(|| token_ref.unwrap().name.to_owned()),
                token_opts.image.or(token_ref.map(|t| t.image.to_owned())).unwrap_or("./assets/unnamed.png".into()),
                token_opts.name.or(token_ref.map(|t| t.name.to_owned())).unwrap_or("Unnamed".into()),
                token_opts.size.or(token_ref.map(|t| t.size.to_owned())).unwrap_or("small".into()),
                token_opts.max_health.or(token_ref.map(|t| t.max_health)).unwrap_or(10),
                token_opts.pos.or(token_ref.map(|t| t.pos.to_owned())).unwrap_or("A1".into()),
                token_opts.initiative.or(token_ref.map(|t| t.initiative)).unwrap_or(1),
            );

            // let new_token = match &game_state.token {
            // None => domain::Token::new(
            //     ctx,
            //     // id, image, name, size, max-health, pos, initiate
            //     token_opts.token_id.to_owned(),
            //     token_opts.image.to_owned().unwrap(),
            //     token_opts.name.to_owned().unwrap(),
            //     token_opts.size.to_owned().unwrap(),
            //     token_opts.max_health.unwrap(),
            //     token_opts.pos.to_owned().unwrap(),
            //     token_opts.initiative.unwrap(),
            // ),
            // Some(ref t) => domain::Token::new(
            //     ctx,
            //     // id, image, name, size, max-health, pos, initiate
            //     token_opts.token_id.to_owned(),
            //     token_opts.image.to_owned().unwrap_or(t.image.to_owned()),
            //     token_opts.name.to_owned().unwrap_or(t.name.to_owned()),
            //     token_opts.size.to_owned().unwrap_or(t.size.to_owned()),
            //     token_opts.max_health.unwrap_or(t.max_health),
            //     token_opts.pos.to_owned().unwrap_or(t.pos.to_owned()),
            //     token_opts.initiative.unwrap_or(t.initiative),
            // )
            // };


            // lookup or create token by token_opts.token_id
            // update fields
            game_state.token = Some(new_token);

            println!("Token State: {:?}", game_state.token);
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.msg_chan.try_recv() {
            Ok(msg) => {
                println!("msg> {}", msg); // debug
                let cmds = commands::parse(msg);
                for cmd in cmds {
                    println!("Command: {:?}", cmd); // debug
                    match cmd {
                        Ok(c) => run(ctx, self, &c),
                        Err(e) => println!("Err: {}", e),
                    }
                }
            }
            Err(_) => {}
        }

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
        graphics::clear(ctx, BACKGROUND_COLOR);

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
