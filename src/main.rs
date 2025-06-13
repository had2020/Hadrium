use std::io::{stdout, Write};

use ::TerimalRtdm::*;

fn load_confi_file() {}

fn save_file() {}

fn open_file() {}

fn backup_file() {}

#[derive(PartialEq)]
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
        if Key::o().pressed(&mut app, "Esc") {
            primary_mode = PrimaryMode::NormalMode;
            Text::new()
                .foreground(Color::Green)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Normal", pos!(0, 0));
        }

        // Insert mode
        if Key::o().pressed(&mut app, "i") {
            primary_mode = PrimaryMode::InsertMode;
            Text::new()
                .foreground(Color::Red)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Insert", pos!(0, 0));

            // written text TODO
            Text::new()
                .vanish(false)
                .show(&mut app, "Test1", pos!(0, 1));
            Text::new()
                .vanish(false)
                .show(&mut app, "Test2", pos!(0, 2));
        }

        // Visual mode
        if Key::o().case_sen(true).pressed(&mut app, "v") {
            primary_mode = PrimaryMode::VisualMode;
            Text::new()
                .foreground(Color::Blue)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Visual", pos!(0, 0));
        }

        // Command mode
        if Key::o().pressed(&mut app, ":") {
            primary_mode = PrimaryMode::CommandLineMode;
            Text::new()
                .foreground(Color::Yellow)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Command", pos!(0, 0));
        }

        if primary_mode == PrimaryMode::CommandLineMode {
            if Key::o().pressed(&mut app, "q") {
                clear(&mut app);
                break;
            }
        }

        // Replace mode
        if Key::o().pressed(&mut app, "R") {
            primary_mode = PrimaryMode::ReplaceMode;
            Text::new()
                .foreground(Color::Magenta)
                .background(Color::White)
                .style(Style::Bold)
                .show(&mut app, "Replace", pos!(0, 0));
        }

        if Key::o().no_clear().pressed(&mut app, "w") {
            mov_cur_dir(&mut app, Dir::Up, 1);
        }
        if Key::o().no_clear().pressed(&mut app, "s") {
            mov_cur_dir(&mut app, Dir::Down, 1);
        }
        if Key::o().no_clear().pressed(&mut app, "a") {
            mov_cur_dir(&mut app, Dir::Left, 1);
        }
        if Key::o().no_clear().pressed(&mut app, "d") {
            mov_cur_dir(&mut app, Dir::Right, 1);
        }

        render(&app);
    }

    raw_mode(false);
}
