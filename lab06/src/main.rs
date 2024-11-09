use rusqlite::{params, Connection, Result};
use std::{error::Error, fs};

#[derive(Debug)]
enum CustomErr {
    UnknownCommand,
    FileNotFound,
    EmptyLine,
    NoArguments,
    TooManyArguments,
    HelloNotFound,
}

trait Commands {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]) -> Result<(), CustomErr>;
}

#[derive(Default)]
struct PingCommand {}
impl Commands for PingCommand {
    fn get_name(&self) -> &'static str {
        "ping"
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), CustomErr> {
        if !args.is_empty() {
            return Err(CustomErr::TooManyArguments);
        }
        println!("Pong!");
        Ok(())
    }
}
struct CountCommand {}

impl Commands for CountCommand {
    fn get_name(&self) -> &'static str {
        "count"
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), CustomErr> {
        if args.is_empty() {
            return Err(CustomErr::NoArguments);
        }
        println!("counted {} args", args.len());
        Ok(())
    }
}
struct TimesCommand {
    count: i32,
}

impl Commands for TimesCommand {
    fn get_name(&self) -> &'static str {
        "times"
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), CustomErr> {
        if !args.is_empty() {
            return Err(CustomErr::TooManyArguments);
        }
        self.count += 1;
        println!("{}", self.count);
        Ok(())
    }
}
struct ContainsHelloCommand {}

impl Commands for ContainsHelloCommand {
    fn get_name(&self) -> &'static str {
        "contains_hello"
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), CustomErr> {
        let mut contains: bool = false;
        let mut position = 1;
        for arg in args.iter() {
            if arg.to_lowercase() == "hello" {
                contains = true;
                break;
            }
            position += 1;
        }
        if contains {
            println!("The word \"hello\" is the {} argument", position);
            Ok(())
        } else {
            Err(CustomErr::HelloNotFound)
        }
    }
}
struct BookmarkCommand {
    db_connection: Connection,
}
impl BookmarkCommand {
    fn new() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open("resources/bookmarks.db")?;
        let command = r"CREATE TABLE IF NOT EXISTS bookmarks (id INTEGER PRIMARY KEY, name TEXT NOT NULL, url TEXT NOT NULL)";
        conn.execute(command, [])?;
        Ok(BookmarkCommand {
            db_connection: conn,
        })
    }

    fn add(&self, name: &str, url: &str) -> Result<(), Box<dyn Error>> {
        let command = r"INSERT INTO bookmarks (name, url) VALUES (?1, ?2)";
        self.db_connection.execute(command, params![name, url])?;
        println!("Bookmark added: {} -> {}", name, url);
        Ok(())
    }

    fn search(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let command = r"SELECT name, url FROM bookmarks WHERE name LIKE ?1";
        let mut stmt = self.db_connection.prepare(command)?;
        let name_pattern = format!("%{}%", name);

        let mut found = false;
        let bookmark_iter = stmt.query_map([name_pattern], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for bookmark in bookmark_iter {
            let (name, url) = bookmark?;
            println!("{} -> {}", name, url);
            found = true;
        }

        if !found {
            println!("-> No bookmarks found matching '{}'", name);
        }

        Ok(())
    }

    fn clear(&self) -> Result<(), Box<dyn Error>> {
        self.db_connection.execute("DELETE FROM bookmarks", ())?;
        Ok(())
    }
}
struct Terminal {
    commands: Vec<Box<dyn Commands>>,
    bookmark_cmd: BookmarkCommand,
}

impl Terminal {
    fn new() -> Result<Self, Box<dyn Error>> {
        let bookmark_cmd = BookmarkCommand::new()?;
        Ok(Terminal {
            commands: Vec::new(),
            bookmark_cmd,
        })
    }
    fn register(&mut self, command: Box<dyn Commands>) -> Result<(), Box<dyn Error>> {
        self.commands.push(command);
        Ok(())
    }
    fn run(&mut self) {
        let mut line_number = 1;
        match fs::read_to_string("resources/commands.txt") {
            Ok(file_content) => {
                for command_line in file_content.lines() {
                    let command: Vec<&str> = command_line.split_whitespace().collect();
                    if command.is_empty() {
                        println!(
                            "-> Be aware at line {}\n   Possible reason: {:?}",
                            line_number,
                            CustomErr::EmptyLine
                        );
                        line_number += 1;
                        continue;
                    }

                    match command[0] {
                        "bk" => match command[1] {
                            "add" if command.len() == 4 => {
                                match self.bookmark_cmd.add(command[2], command[3]) {
                                    Ok(_) => println!("-> Bookmark added successfully."),
                                    Err(e) => println!("-> Error adding bookmark: {}", e),
                                }
                            }
                            "search" if command.len() == 3 => {
                                match self.bookmark_cmd.search(command[2]) {
                                    Ok(_) => println!("-> Search completed."),
                                    Err(e) => println!("-> Error searching bookmarks: {}", e),
                                }
                            }
                            "clear" if command.len() == 2 => match self.bookmark_cmd.clear() {
                                Ok(_) => println!("-> All bookmarks have been deleted!"),
                                Err(e) => println!("-> Error deleting bookmarks: {}!", e),
                            },
                            _ => {
                                println!("Invalid bk command format. Use 'bk add <name> <url>' or 'bk search <name>'.");
                            }
                        },
                        _ => {
                            let mut command_found = false;
                            for c in &mut self.commands {
                                if c.get_name() == command[0] {
                                    command_found = true;
                                    match c.exec(&command[1..]) {
                                        Ok(_c) => {
                                            println!("-> Command {} executed succesfully!", command[0])
                                        }
                                        Err(err) => println!(
                                            "-> Error at executing command {}\n   Possible Reason: {:?}",
                                            command[0], err
                                        ),
                                    }
                                } else if c.get_name() == command[0].to_lowercase() {
                                    println! {"-> You might want to correct {} into {}, please look at file(line {})!", command[0], c.get_name(), line_number}
                                    command_found = true;
                                } else if command[0] == "stop" {
                                    return;
                                }
                            }

                            if !command_found {
                                println!("-> Error at executing command:(line {}) {}\n   Possible Reason: {:?}", line_number, command[0], CustomErr::UnknownCommand);
                            }
                        }
                    }
                    line_number += 1;
                }
            }
            Err(_err) => println!(
                "-> Error at opening file \n Possible reason: {:?}",
                CustomErr::FileNotFound
            ),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = Terminal::new()?;

    terminal.register(Box::new(PingCommand {}))?;
    terminal.register(Box::new(CountCommand {}))?;
    terminal.register(Box::new(TimesCommand { count: 0 }))?;
    terminal.register(Box::new(ContainsHelloCommand {}))?;

    terminal.run();
    Ok(())
}
