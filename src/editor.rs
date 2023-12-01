use std::fs::{self, File, OpenOptions};
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::io::{stdin, stdout, Read};
use crossterm::{
    execute, cursor,
    terminal::{self, Clear, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
    event::{self, Event, KeyCode, KeyEvent},
};

pub struct Editor {
    file: Option<File>,
}
enum Command {
    Insert(char),
    Exit,
}

impl Editor {
    pub fn new() -> Self {
        Editor { file: None }
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
        
        loop {
            self.display(&mut contents);
            if let Some(keypress) = self.get_keypress() {
                let command = self.keypress_to_command(&keypress);
                match command {
                    Command::Insert(c) => {
                        // TODO
                    }

                    Command::Exit => break, 
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

    fn keypress_to_command(&self, keypress: &KeyEvent) -> Command {
        match keypress {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Command::Exit,

            _ => return Command::Insert('j')
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

    fn display(&self, content: &mut str) {
        for (i,l) in content.lines().enumerate() {
            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("{}\t", i)),
                ResetColor,
                Print(format!("{}\n\r", l))
            ).unwrap();
        }
        execute!(
            stdout(),
            cursor::MoveTo(1,0)
        ).unwrap();
    }

    fn clear_screen(&self) -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        self.clear_screen().unwrap();
    }
}