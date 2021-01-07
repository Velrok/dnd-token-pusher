use tetra::ContextBuilder;
mod chess;
mod commands;
mod domain;
mod game;

fn main() -> tetra::Result {
    ContextBuilder::new("Cameras", 640, 480)
        .resizable(true)
        .show_mouse(true)
        .quit_on_escape(true)
        .maximized(true)
        .build()?
        .run(game::GameState::new)
}
