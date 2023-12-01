use std::fs::{self, File, OpenOptions};
use std::ffi::OsString;
use std::path::Path;
use std::io::{stdout, Read};
use crossterm::ExecutableCommand;
use crossterm::{
    execute, cursor,
    terminal::{self, Clear, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};

pub struct Editor {
    file: Option<File>,
    width: u16,
    height: u16,
    right_padding: u16,
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

enum Command {
    Insert(char),
    Move(Move),
    Exit,
}

impl Editor {
    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        Editor {
            file: None,
            width: width,
            height: height,
            right_padding: 8
        }
    }

    pub fn open_file(&mut self, path: &Path) {
        let exists = fs::metadata(path).is_ok();
        
        let mut file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .create(!exists)
            .open(path)
            .expect("Could not open file");

        let filename: OsString = path.file_name().unwrap().to_owned();
        
        if !exists {
            println!("Created new file called {}", filename.to_str().unwrap());
        }

        self.file = Some(file);
    }

    pub fn run(&mut self) {
        terminal::enable_raw_mode().expect("Couldn't enable raw mode");
        
        let mut contents: String = self.get_contents();
        let mut pos: (u16, u32) = (0, 0);
        let mut first_line: u32 = 0;
        let mut last_line: u32 = contents.lines().count() as u32;
        
        loop {
            self.draw(&mut contents, &mut pos, &first_line);
            if let Some(keypress) = self.get_keypress() {
                let command_option = self.process_keypress(&keypress);
                
                if let Some(command) = command_option {
                    match command {
                        Command::Insert(c) => {
                            // TODO
                        }

                        Command::Move(direction) => {
                            match direction {
                                Move::Up => {pos.1 = 0.max(pos.1-1)}
                                Move::Down => {pos.1 = last_line.min(pos.1+1)},
                                Move::Right => {pos.0 = self.width.min(pos.0+1)},
                                Move::Left => {pos.0 = (0 as u16).max(pos.0-1)},
                                // Move::Left => {pos.0 = 0.max(pos.0-1)},
                            }
                        }

                    Command::Exit => break,
                    }
                }
            }
        }
    }

    fn get_keypress(&self) -> Option<KeyEvent> {
        if let Event::Key(event) = event::read().unwrap() {
            Some(event)
        } else {
            None
        }
    }

    fn process_keypress(&self, keypress: &KeyEvent) -> Option<Command> {
        match keypress {
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                ..
            } => return self.process_unmodified(keypress),

            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return self.process_ctrl(keypress),

            _ => return None,
        }
    }

    fn process_unmodified(&self, keypress: &KeyEvent) -> Option<Command> {
        match keypress {
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => return Some(Command::Insert(*c)),

            KeyEvent {code: KeyCode::Up, ..} => return Some(Command::Move(Move::Up)),
            KeyEvent {code: KeyCode::Down, ..} => return Some(Command::Move(Move::Down)),
            KeyEvent {code: KeyCode::Left, ..} => return Some(Command::Move(Move::Left)),
            KeyEvent {code: KeyCode::Right, ..} => return Some(Command::Move(Move::Right)),

            _ => return None,
        }
    }

    fn process_ctrl(&self, keypress: &KeyEvent) -> Option<Command> {
        match keypress {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => Some(Command::Exit),

            _ => return None,
        }
    }

    fn get_contents(&mut self) -> String {
        let mut contents: String = String::new();

        if let Some(file) = self.file.as_mut() {
            file.read_to_string(&mut contents).unwrap();

        } else {
            panic!("No file was loaded");
        }

        return contents;
    }

    fn draw(&self, content: &mut str, pos: &mut (u16, u32), first_line: &u32) {
        let (x, y) = pos;
        let screen_x: u16 = (*x as u16) + self.right_padding;
        let screen_y: u16 = (*y + first_line) as u16;

        self.clear_screen().unwrap();
        for (i,l) in content
            .lines()
            .skip(*first_line as usize)
            .take(self.height as usize)
            .enumerate() {

            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("{}\t", i)),
                ResetColor,
                Print(format!("{}", l)),
            ).unwrap();

            if i < (self.height - 1).into() {
                stdout().execute(Print("\n\r")).unwrap();
            }
        }
        execute!(
            stdout(),
            cursor::MoveTo(screen_x, screen_y)
        ).unwrap();
    }

    fn clear_screen(&self) -> Result<(), std::io::Error> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
        )
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        self.clear_screen().unwrap();
    }
}