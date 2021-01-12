use structopt::StructOpt;

mod opts {
    use structopt::StructOpt;

    #[derive(StructOpt, Debug, PartialEq)]
    pub struct Battlemap {
        #[structopt(long)]
        pub url: Option<String>,
        #[structopt(long)]
        pub columns: Option<i32>,
        #[structopt(long)]
        pub rows: Option<i32>,
    }

    #[test]
    fn opts_test() {
        assert_eq!(
            Battlemap {
                url: Some(String::from("./assets/background.jpg")),
                columns: Some(100),
                rows: Some(100)
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
    UpdateBattlemap(opts::Battlemap),
    PrintHelp(String),
    Quit,
    Role(caith::Roller),
}

pub const HELP: &str = "Commands:
q | quit | exit -> terminate programm
r 3d6 + 5       -> roll dice and do some math
r 2d20 K1       -> advantage (keep highest one)
r 2d20 k1       -> disadvantage (keep lowest one)
h | help | ?    -> print this help";

pub fn parse(content: String) -> Vec<Result<Command, String>> {
    use Command::*;
    content
        .lines()
        .map(|l| {
            let words: Vec<_> = l.split_whitespace().collect();
            match words[0] {
                "q" => Ok(Quit),
                "quit" => Ok(Quit),
                "exit" => Ok(Quit),
                "r" => {
                    let roller = caith::Roller::new(&l.to_owned().replace("r ", ""));
                    match roller {
                        Ok(r) => Ok(Role(r)),
                        Err(_) => Err(format!("Can't parse role from {}", l)),
                    }
                }
                "battlemap" => match opts::Battlemap::from_iter_safe(l.split_whitespace()) {
                    Ok(x) => Ok(UpdateBattlemap(x)),
                    Err(_) => Err(format!("Can't parse battlemap command from {}", l)),
                },
                _ => Ok(PrintHelp(l.to_owned())),
            }
        })
        .collect()
}
