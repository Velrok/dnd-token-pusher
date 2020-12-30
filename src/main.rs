use bevy::prelude::*;

struct Person;
struct Name(String);

fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if the timer hasn't finished yet, we return
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    // looks like Query does funky stuff; looks a bit like patter matching agains tuples maybe?
    // From the docs:
    // You can interpret the Query above as: "iterate over every Name component for entities that also have a Person component"
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true))) // the reason we call from_seconds with the true flag is to make the timer repeat itself
            .add_startup_system(add_people.system()) // run exactly once before other systems
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins) // render, input & event loops, ...
        .add_plugin(HelloPlugin)
        .run();
}

// extern crate piston_window;

// use piston_window::*;

// fn main() {
//     let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
//         .exit_on_esc(true)
//         .build()
//         .unwrap();
//     while let Some(event) = window.next() {
//         window.draw_2d(&event, |context, graphics, _device| {
//             clear([1.0; 4], graphics);
//             rectangle(
//                 [1.0, 0.0, 0.0, 1.0], // red
//                 [0.0, 0.0, 100.0, 100.0],
//                 context.transform,
//                 graphics,
//             );
//         });
//     }
// }
