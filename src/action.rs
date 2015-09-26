
use command;

pub enum Action {
    Command(command::Command),
    Insert(String),
    InsertEnd,
}


