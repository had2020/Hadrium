use std::io::{stdout, Write};

use ::TerimalRtdm::*;

fn load_confi_file() {}

fn save_file() {}

fn open_file() {}

fn backup_file() {}

enum PrimaryMode {
    NormalMode,
    InsertMode,
    VisualMode,
    CommandLineMode,
    ReplaceMode,
    SelectMode,
}

fn main() {
    let mut primary_mode: PrimaryMode = PrimaryMode::NormalMode;

    let mut app = App::new();
    clear(&mut app);

    raw_line("Welcome to Hadrium");

    raw_mode(true);
    clear(&mut app);

    loop {
        collect_presses(&mut app);

        // Normal mode
        if key_press(&app, "Esc") {
            primary_mode = PrimaryMode::NormalMode;
            Text::new()
                .foreground(Color::Green)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Normal", pos!(0, 0));
            Text::new().show(&mut app, "Test1", pos!(0, 1));
            Text::new().show(&mut app, "Test2", pos!(0, 2));
        }

        // Insert mode
        if key_press(&app, "i") {
            primary_mode = PrimaryMode::InsertMode;
            Text::new()
                .foreground(Color::Red)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Insert", pos!(0, 0));
        }

        // Visual mode
        if key_press(&app, "v") {
            primary_mode = PrimaryMode::VisualMode;
            Text::new()
                .foreground(Color::Blue)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Visual", pos!(0, 0));
        }
        if key_press(&app, "V") {
            primary_mode = PrimaryMode::VisualMode;
            Text::new()
                .foreground(Color::Green)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Visual", pos!(0, 0));
        }

        // Command mode
        if key_press(&app, ":") {
            primary_mode = PrimaryMode::CommandLineMode;
            Text::new()
                .foreground(Color::Yellow)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Command", pos!(0, 0));

            //TODO fix this case
            if key_press(&app, "q") {
                clear(&mut app);
                break;
            }
        }

        // Replace mode
        if key_press(&app, "R") {
            primary_mode = PrimaryMode::ReplaceMode;
            Text::new()
                .foreground(Color::Magenta)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Replace", pos!(0, 0));
        }

        /*
        if key_press(&app, "q") {
            clear(&mut app);
            break;
        }
        */

        if key_press(&app, "e") {
            move_cursor(&mut app, pos!(0, 5));
        }

        render(&app);
    }

    raw_mode(false);
}
