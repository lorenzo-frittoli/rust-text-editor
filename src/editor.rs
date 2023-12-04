use std::fs::{self, File, OpenOptions};
use std::ffi::OsString;
use std::path::Path;
use std::io::{stdout, Read, Write, Seek, SeekFrom};
use crossterm::ExecutableCommand;
use crossterm::{
    execute, cursor,
    terminal::{self, Clear, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};

pub struct Editor {
    file: Option<File>,
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
    NewLine,
    BackSpace,
    Save,
    Exit,
}

impl Editor {
    pub fn new() -> Self {
        let (_, height) = terminal::size().unwrap();
        Editor {
            file: None,
            height: height,
            right_padding: 8
        }
    }

    pub fn open_file(&mut self, path: &Path) {
        let exists = fs::metadata(path).is_ok();
        
        self.file = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(!exists)
            .open(path)
            .expect("Could not open file"));

        let filename: OsString = path.file_name().unwrap().to_owned();
        
        if !exists {
            println!("Created new file called {}", filename.to_str().unwrap());
        }
    }

    pub fn run(&mut self) {
        terminal::enable_raw_mode().expect("Couldn't enable raw mode");
        
        let mut lines: Vec<String> = self.get_contents()
            .lines()
            .map(|s| s.to_owned())
            .collect();

        let mut first_line = 0;
        
        stdout().execute(cursor::MoveTo(self.right_padding,0)).unwrap();

        loop {
            let (x,y) = cursor::position().unwrap();
            let current_line = y as usize + first_line;
            self.draw(&lines, &first_line);
            stdout().execute(cursor::MoveTo(x,y)).unwrap();

            if let Some(keypress) = self.get_keypress() {
                let command_option = self.process_keypress(&keypress);
                
                if let Some(command) = command_option {
                    match command {
                        Command::Insert(c) => {
                            lines[current_line].insert((x - self.right_padding) as usize, c);
                            stdout().execute(cursor::MoveRight(1)).unwrap();
                        }

                        Command::Move(direction) => {
                            let pos = cursor::position().unwrap();
                            let max_h = self.height.min((lines.len() - first_line) as u16) - 1;
                            match direction {
                                Move::Up => {
                                    if pos.1 > 0 {
                                        stdout().execute(cursor::MoveUp(1)).unwrap();
                                        if pos.0 - self.right_padding > lines[current_line-1].len() as u16 {
                                            stdout().execute(cursor::MoveToColumn(lines[current_line-1].len() as u16 + self.right_padding)).unwrap();
                                        }

                                    } else if first_line > 0 {
                                        first_line = first_line - 1;
                                        if pos.0 - self.right_padding > lines[current_line-1].len() as u16 {
                                            stdout().execute(cursor::MoveToColumn(lines[current_line-1].len() as u16 + self.right_padding)).unwrap();
                                        }
                                    }
                                },
                                Move::Down => {
                                    if pos.1 < max_h {
                                        stdout().execute(cursor::MoveDown(1)).unwrap();
                                        if pos.0 - self.right_padding > lines[current_line+1].len() as u16 {
                                            stdout().execute(cursor::MoveToColumn(lines[current_line+1].len() as u16 + self.right_padding)).unwrap();
                                        }

                                    } else if (pos.1 as usize) + first_line < lines.len()-1 {
                                        first_line = first_line + 1;
                                        if pos.0 - self.right_padding > lines[current_line+1].len() as u16 {
                                            stdout().execute(cursor::MoveToColumn(lines[current_line+1].len() as u16 + self.right_padding)).unwrap();
                                        }
                                    }
                                },
                                Move::Right => {
                                    if (pos.0 as usize) < lines[current_line].len() + (self.right_padding as usize) {
                                        stdout().execute(cursor::MoveRight(1)).unwrap();

                                    } else if pos.1 < max_h {
                                        execute!(
                                            stdout(),
                                            cursor::MoveToColumn(self.right_padding),
                                            cursor::MoveDown(1),
                                        ).unwrap();

                                    } else if current_line < lines.len()-1 {
                                        first_line += 1;
                                        stdout().execute(cursor::MoveToColumn(self.right_padding)).unwrap();
                                    }
                                },
                                Move::Left => {
                                    if pos.0 > self.right_padding {
                                        stdout().execute(cursor::MoveLeft(1)).unwrap();

                                    } else if pos.1 > 0 {
                                        execute!(
                                            stdout(),
                                            cursor::MoveToColumn(lines[(pos.1-1) as usize + first_line].len() as u16 + self.right_padding),
                                            cursor::MoveUp(1),
                                        ).unwrap();
                                    
                                    } else if first_line > 0 {
                                        execute!(
                                            stdout(),
                                            cursor::MoveToColumn(lines[pos.1 as usize + first_line - 1].len() as u16 + self.right_padding),
                                        ).unwrap();
                                        first_line -= 1;
                                    }
                                },
                            }
                        }

                        Command::NewLine => {
                            let (str1, str2) = lines[current_line]
                                .split_at((x - self.right_padding) as usize);

                            let s1 = str1.to_owned();
                            let s2 = str2.to_owned();

                            lines[current_line] = s1;
                            lines.insert(current_line+1, s2);
                            stdout().execute(cursor::MoveDown(1)).unwrap();
                            stdout().execute(cursor::MoveToColumn(self.right_padding)).unwrap();
                        },

                        Command::BackSpace => {
                            let pos = cursor::position().unwrap();
                            if x > self.right_padding {
                                lines[current_line].remove((x-self.right_padding-1) as usize);
                                stdout().execute(cursor::MoveLeft(1)).unwrap();
                            } else if current_line > 0 {
                                let l = lines.remove(current_line);
                                if pos.1 > 0 {
                                    execute!(
                                        stdout(),
                                        cursor::MoveToColumn(lines[(pos.1-1) as usize + first_line].len() as u16 + self.right_padding),
                                        cursor::MoveUp(1),
                                    ).unwrap();
                                } else {
                                    execute!(
                                        stdout(),
                                        cursor::MoveToColumn(lines[pos.1 as usize + first_line - 1].len() as u16 + self.right_padding),
                                    ).unwrap();
                                    first_line -= 1;
                                }
                                lines[current_line-1].push_str(&l);
                            }
                        }
                        
                        Command::Save => {
                            if let Some(file) = &mut self.file {
                                file.set_len(0).unwrap();
                                file.seek(SeekFrom::Start(0)).unwrap();
                                for l in &lines {
                                    file.write_all((l.to_owned() + "\n").as_bytes()).unwrap();
                                }
                                file.flush().unwrap();
                            }
                        },

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
            KeyEvent {code: KeyCode::Char(c), ..} => return Some(Command::Insert(*c)),
            KeyEvent {code: KeyCode::Up, ..} => return Some(Command::Move(Move::Up)),
            KeyEvent {code: KeyCode::Down, ..} => return Some(Command::Move(Move::Down)),
            KeyEvent {code: KeyCode::Left, ..} => return Some(Command::Move(Move::Left)),
            KeyEvent {code: KeyCode::Right, ..} => return Some(Command::Move(Move::Right)),
            KeyEvent {code: KeyCode::Backspace, ..} => return Some(Command::BackSpace),
            KeyEvent {code: KeyCode::Enter, ..} => return Some(Command::NewLine),

            _ => return None,
        }
    }

    fn process_ctrl(&self, keypress: &KeyEvent) -> Option<Command> {
        match keypress {
            KeyEvent {code: KeyCode::Char('q'), ..} => Some(Command::Exit),
            KeyEvent {code: KeyCode::Char('s'), ..} => Some(Command::Save),
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

    fn draw(&self, content: &Vec<String>, first_line: &usize) {
        self.clear_screen().unwrap();
        for (i,l) in content
            .iter()
            .skip(*first_line as usize)
            .take(self.height as usize)
            .enumerate() {

            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("{}\t", i+first_line)),
                ResetColor,
                Print(format!("{}", l)),
            ).unwrap();

            if i < (self.height - 1).into() {
                stdout().execute(Print("\n\r")).unwrap();
            }
        }
        // execute!(
        //     stdout(),
        //     cursor::MoveTo(self.right_padding, 0)
        // ).unwrap();
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