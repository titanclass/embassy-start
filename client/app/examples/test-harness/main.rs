extern crate menu;

use menu::*;
use pancurses::{endwin, initscr, noecho, Input};
use std::fmt::Write;

const ROOT_MENU: Menu<Context> = Menu {
    label: "root",
    items: &[&Item {
        item_type: ItemType::Callback {
            function: some_command,
            parameters: &[Parameter::Mandatory {
                parameter_name: "some-param",
                help: Some("Some parameter"),
            }],
        },
        command: "some-command",
        help: Some("Accepts a command with a param."),
    }],
    entry: Some(enter),
    exit: None,
};

struct Window(pancurses::Window);

impl std::fmt::Write for Window {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.0.printw(s);
        Ok(())
    }
}

struct Context {
    window: Window,
}

impl std::fmt::Write for Context {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.window.0.printw(s);
        Ok(())
    }
}

fn main() {
    let window = initscr();
    window.scrollok(true);
    noecho();
    let mut buffer = [0u8; 64];
    let mut r = Runner::new(
        &ROOT_MENU,
        &mut buffer,
        Context {
            window: Window(window),
        },
    );
    loop {
        match r.context.window.0.getch() {
            Some(Input::Character('\n')) => {
                r.input_byte(b'\r');
            }
            Some(Input::Character(c)) => {
                let mut buf = [0; 4];
                for b in c.encode_utf8(&mut buf).bytes() {
                    r.input_byte(b);
                }
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                r.context.window.0.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }
    endwin();
}

fn enter(_menu: &Menu<Context>, context: &mut Context) {
    writeln!(context.window, "Starting").unwrap();
}

fn some_command(_menu: &Menu<Context>, item: &Item<Context>, args: &[&str], context: &mut Context) {
    match argument_finder(item, args, "some-param") {
        Ok(Some(param)) => {
            if let Ok(num_param) = param.parse::<u32>() {
                writeln!(context, "Got some command with param {}", num_param).unwrap();
            } else {
                writeln!(context, "Expecting numbic arg, got {}", param).unwrap();
            }
        }
        _ => {
            writeln!(context, "Expecting arg").unwrap();
        }
    };
}
