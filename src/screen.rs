use std::io;
use terminfo::{Database, capability as cap};

// li lines Lines
// co columns Columns
// cl clear_screen ClearScreen
// ce clr_eol ClrEol
// cm cursor_address CursorAddress
// ti enter_ca_mode EnterCaMode

pub struct ScreenManager {
    db : Database
}

impl ScreenManager {
    pub fn init_screen() -> ScreenManager {
        let manager = ScreenManager {
            db: Database::from_env().unwrap()
        };
        manager.db.get::<cap::EnterCaMode>().unwrap().expand().to(io::stdout()).unwrap();
        manager
    }

    pub fn string_at_pos(&self, string: &str, row: u32, col: u32) {
        let info = Database::from_env().unwrap();
        info.get::<cap::CursorAddress>().unwrap().expand().x(col).y(row).to(io::stdout()).unwrap();
        print!("{}", string);
    }
}

impl Drop for ScreenManager {
    fn drop(&mut self) {
        self.db.get::<cap::ExitCaMode>().unwrap().expand().to(io::stdout());
    }
}