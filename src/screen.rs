use std::io::{self, stdout, Write};
use terminfo::{capability as cap, Database};

// li lines Lines
// co columns Columns
// cl clear_screen ClearScreen
// ce clr_eol ClrEol
// cm cursor_address CursorAddress
// ti enter_ca_mode EnterCaMode

pub struct ScreenManager {
    db: Database,
}

impl ScreenManager {
    pub fn init_screen() -> ScreenManager {
        let manager = ScreenManager {
            db: Database::from_env().unwrap(),
        };
        manager
    }

    pub fn string_at_pos(&self, string: &str, row: u32, col: u32) {
        self.db
            .get::<cap::CursorAddress>()
            .unwrap()
            .expand()
            .x(col)
            .y(row)
            .to(io::stdout())
            .unwrap();
        print!("{}", string);
        stdout().flush().unwrap();
    }

    pub fn clear_screen(&self) {
        self.db
            .get::<cap::ClearScreen>()
            .unwrap()
            .expand()
            .to(io::stdout())
            .unwrap();
    }

    pub fn get_num_cols(&self) -> i32 {
        self.db.get::<cap::Columns>().unwrap().0
    }

    pub fn get_num_rows(&self) -> i32 {
        self.db.get::<cap::Lines>().unwrap().0
    }
}
