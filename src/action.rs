
use ui;

pub enum Action {
    InsertLine(String),
    SwitchMode(ui::Mode),
    Quit
}

