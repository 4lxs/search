mod block;
mod launcher;

use std::io;

use anyhow::{anyhow, Context, Result};
use block::{Block, BlockLine};

use crate::{
    block::all_blocks,
    launcher::{rofi::RofiLauncher, Entry, Launcher},
};

struct BlockData {
    name: String,
    block: Box<dyn Block>,
    lines: Vec<BlockLine>,
}

fn main() -> Result<()> {
    let mut blocks = all_blocks();
    #[allow(unused_mut)] // doesn't compile without
    let mut blocks = blocks
        .drain()
        .map(|(name, mut block)| {
            BlockData {
                name,
                // populate with default search results
                lines: block.get_lines("".into()),
                block,
            }
        })
        .collect::<Vec<BlockData>>();

    let mut initial_entries = Vec::new();
    let mut entries = Vec::new();

    for (bix, block) in blocks.iter().enumerate() {
        for line in &block.lines {
            initial_entries.push(Entry {
                id: entries.len(),
                text: line.text.clone(),
                icon: line.icon.clone(),
            });
            entries.push((bix, line));
        }
    }

    let mut launcher = RofiLauncher::new();
    launcher.update(&mut initial_entries)?;

    let entry_id = loop {
        match launcher.wait()? {
            launcher::Event::InputChange(_input) => {}
            launcher::Event::SelectEntry(entry) => {
                break entry;
            }
        }
    };

    let (bix, line) = match entries.get(entry_id) {
        Some(v) => v,
        None => return Err(anyhow!("invalid eix")),
    };

    let block = match blocks.get(*bix) {
        Some(b) => b,
        None => return Err(anyhow!("failed to get block {bix} of {}", blocks.len())),
    };

    block.block.run(line)?;

    return Ok(());
}
