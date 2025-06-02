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

    clear();
    let mut app = App::new();

    raw_line("Welcome to Hadrium");

    raw_mode(true);

    loop {
        clear();
        collect_presses(&mut app);

        // Normal mode
        if key_press(&app, "Esc") {
            primary_mode = PrimaryMode::NormalMode;
            line(Position { x: 0, y: 5 }, "Normal", "blue");
        }

        // Insert mode
        if key_press(&app, "i") {
            primary_mode = PrimaryMode::InsertMode;
            line(Position { x: 0, y: 5 }, "Insert", "red");
        }

        // Visual mode
        if key_press(&app, "v") {
            primary_mode = PrimaryMode::VisualMode;
            line(Position { x: 0, y: 5 }, "Visual", "yellow");
        }
        if key_press(&app, "V") {
            primary_mode = PrimaryMode::VisualMode;
            line(Position { x: 0, y: 5 }, "Visual", "yellow");
        }
        //TODO Ctrl+v

        // Command mode
        if key_press(&app, ":") {
            primary_mode = PrimaryMode::CommandLineMode;
            line(Position { x: 0, y: 5 }, "Command", "red");
        }

        // Replace mode
        if key_press(&app, "R") {
            primary_mode = PrimaryMode::ReplaceMode;
            line(Position { x: 0, y: 5 }, "Command", "red");
        }

        if key_press(&app, "q") {
            clear();
            break;
        }
    }

    raw_mode(false);
}
