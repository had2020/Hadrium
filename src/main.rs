use TerimalRtdm::*;

use std::env;

use std::io::{stdout, Write}; //TODO Usage

fn load_confi_file() {} //TODO

fn save_file() {} //TODO

fn open_file() {} //TODO

fn backup_file() {} //TODO

#[derive(PartialEq)]
enum PrimaryMode {
    NormalMode,
    InsertMode,
    VisualMode,
    CommandLineMode,
    ReplaceMode,
    //SelectMode,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut stored_file: Vec<String> = vec![];

    if args.len() > 1 {
        if args[1].contains(".") {
            let mut primary_mode: PrimaryMode = PrimaryMode::NormalMode;
            let mut app = App::new();

            raw_mode(true);
            clear(&mut app);

            app.enable_f_row_and_arrow = true;

            // startup
            app.enable_f_row_and_arrow = true;
            Text::new()
                .foreground(Color::Black)
                .background(Color::Green)
                .style(Style::Bold)
                .show(&mut app, "Normal", pos!(0, 0));

            loop {
                // Normal mode
                if Key::o().pressed(&mut app, KeyType::Esc) {
                    app.enable_f_row_and_arrow = true;
                    primary_mode = PrimaryMode::NormalMode;
                    Text::new()
                        .foreground(Color::Black)
                        .background(Color::Green)
                        .style(Style::Bold)
                        .show(&mut app, "Normal", pos!(0, 0));
                }

                // Insert mode
                if Key::o().pressed(&mut app, KeyType::i) && primary_mode != PrimaryMode::InsertMode
                {
                    app.enable_f_row_and_arrow = true;
                    primary_mode = PrimaryMode::InsertMode;
                    Text::new()
                        .foreground(Color::Black)
                        .background(Color::Red)
                        .style(Style::Bold)
                        .show(&mut app, "Insert", pos!(0, 0));
                }

                // Visual mode
                if Key::o().case_sen(true).pressed(&mut app, KeyType::v)
                    && primary_mode != PrimaryMode::InsertMode
                {
                    app.enable_f_row_and_arrow = false;
                    primary_mode = PrimaryMode::VisualMode;
                    Text::new()
                        .foreground(Color::Black)
                        .background(Color::Blue)
                        .style(Style::Bold)
                        .show(&mut app, "Visual", pos!(0, 0));
                }

                // Command mode
                if Key::o().pressed(&mut app, KeyType::Colon)
                    && primary_mode != PrimaryMode::InsertMode
                {
                    app.enable_f_row_and_arrow = false;
                    primary_mode = PrimaryMode::CommandLineMode;
                    Text::new()
                        .foreground(Color::Black)
                        .background(Color::Yellow)
                        .style(Style::Bold)
                        .show(&mut app, "Command", pos!(0, 0));
                }

                // Replace mode
                if Key::o().pressed(&mut app, KeyType::R) && primary_mode != PrimaryMode::InsertMode
                {
                    app.enable_f_row_and_arrow = false;
                    primary_mode = PrimaryMode::ReplaceMode;
                    Text::new()
                        .foreground(Color::Black)
                        .background(Color::Magenta)
                        .style(Style::Bold)
                        .show(&mut app, "Replace", pos!(0, 0));
                }

                if primary_mode == PrimaryMode::CommandLineMode {
                    if Key::o().pressed(&mut app, KeyType::q) {
                        clear(&mut app);
                        break;
                    }
                }

                if primary_mode == PrimaryMode::NormalMode {
                    // Vim directional
                    if Key::o().no_clear().pressed(&mut app, KeyType::k) {
                        Mov::cur().dir(&mut app, Dir::Up, 1); //Todo block impl of wrap
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::j) {
                        Mov::cur().dir(&mut app, Dir::Down, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::h) {
                        Mov::cur().dir(&mut app, Dir::Left, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::l) {
                        Mov::cur().dir(&mut app, Dir::Right, 1);
                    }

                    // Arrow directional
                    if Key::o().no_clear().pressed(&mut app, KeyType::UpArrow) {
                        Mov::cur().block().dir(&mut app, Dir::Up, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::DownArrow) {
                        Mov::cur().block().dir(&mut app, Dir::Down, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::LeftArrow) {
                        Mov::cur().block().dir(&mut app, Dir::Left, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::RightArrow) {
                        Mov::cur().block().dir(&mut app, Dir::Right, 1);
                    }
                }

                if primary_mode == PrimaryMode::InsertMode {
                    //println!("{:?}", key_pressed(&app));
                    //println!("{:?}", debug_code_pressed(&mut app));

                    // Arrow directional with wrap
                    if Key::o().no_clear().pressed(&mut app, KeyType::UpArrow) {
                        Mov::cur().wrap().dir(&mut app, Dir::Up, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::DownArrow) {
                        Mov::cur().wrap().dir(&mut app, Dir::Down, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::LeftArrow) {
                        Mov::cur().wrap().dir(&mut app, Dir::Left, 1);
                    }
                    if Key::o().no_clear().pressed(&mut app, KeyType::RightArrow) {
                        Mov::cur().wrap().dir(&mut app, Dir::Right, 1);
                    }
                }

                if primary_mode == PrimaryMode::InsertMode {
                    stored_file.push(key_pressed(&app));

                    let mut counter = 1;
                    for line in &stored_file {
                        Text::new().vanish(false).show(
                            &mut app,
                            &stored_file[0],
                            Pos { x: 0, y: counter },
                        );
                        counter += 1;
                    }
                }

                render(&app);
                collect_presses(&mut app);
            }

            raw_mode(false);
        } else {
            println!("Add file extension!");
        }
    } else {
        println!("Enter File path as argument!");
    }
}
