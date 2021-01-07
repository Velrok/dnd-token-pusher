use structopt::StructOpt;

mod opts {
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[derive(StructOpt, Debug, PartialEq)]
    pub struct Battlemap {
        #[structopt(long)]
        url: PathBuf,
        #[structopt(long)]
        columns: i32,
        #[structopt(long)]
        rows: i32,
    }

    #[test]
    fn opts_test() {
        assert_eq!(
            Battlemap {
                url: PathBuf::from("./assets/background.jpg"),
                columns: 100,
                rows: 100
            },
            Battlemap::from_iter_safe(
                "battlemap --url=./assets/background.jpg --columns=100 --rows=100"
                    .split_whitespace()
            )
            .unwrap()
        )
    }
}

#[derive(Debug)]
pub enum Command {
    Battlemap(opts::Battlemap),
    PrintHelp(String),
    Quit,
    Role(String),
}

pub const HELP: &str = "Commands:
q | quit | exit -> terminate programm
r 3d6 + 5       -> roll dice and do some math
r 2d20 K1       -> advantage (keep highest one)
r 2d20 k1       -> disadvantage (keep lowest one)
h | help | ?    -> print this help";

pub fn run(cmd: &Command) {
    use Command::*;
    match cmd {
        Quit => std::process::exit(0),
        PrintHelp(l) => println!("Unknows command: {}\n{}", l, HELP),
        Role(l) => {
            let result = caith::Roller::new(l).unwrap().roll().unwrap();
            println!("-> {}", result);
        }
    }
}

pub fn parse(content: String) -> Vec<Command> {
    use Command::*;
    content
        .lines()
        .map(|l| {
            let words: Vec<_> = l.split_whitespace().collect();
            match words[0] {
                "q" => Quit,
                "quit" => Quit,
                "exit" => Quit,
                "r" => Role(l.to_owned().replace("r ", "")),
                "battlemap" => {
                    Battlemap(opts::Battlemap::from_iter_safe(l.split_whitespace()).unwrap())
                }
                _ => PrintHelp(l.to_owned()),
            }
        })
        .collect()
}
