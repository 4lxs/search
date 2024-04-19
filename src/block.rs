use std::collections::HashMap;

use self::desktop_entry::DesktopEntryBlock;

mod desktop_entry;

enum BlockLineData {
    DesktopEntryData(desktop_entry::LineData),
}

pub struct BlockLine {
    pub text: String,
    pub icon: Option<String>,
    data: BlockLineData,
}

pub trait Block {
    fn get_lines(&mut self, input: String) -> Vec<BlockLine>;
    fn run(&self, entry: &BlockLine) -> anyhow::Result<()>;
}

pub enum BlockKind {
    DesktopEntryBlock(DesktopEntryBlock),
}

pub fn all_blocks() -> HashMap<String, Box<dyn Block>> {
    HashMap::from([(
        "desktop_entry".into(),
        Box::new(DesktopEntryBlock::default()) as Box<dyn Block>,
    )])
}
