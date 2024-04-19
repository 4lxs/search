pub mod rofi;

/// the text that the user entered so far
pub struct Input {
    input: String,
}

pub type EntryId = usize;

pub struct Entry {
    pub id: EntryId,
    pub text: String,
    pub icon: Option<String>,
}

pub enum Event {
    InputChange(Input),
    SelectEntry(EntryId),
}

pub trait Launcher {
    fn wait(&self) -> anyhow::Result<Event>;
    fn update(&mut self, entries: &mut Vec<Entry>) -> anyhow::Result<()>;
}
