use std::fs::File;
use memflow::{prelude::v1::{Result}, MemoryMap, FileIOMemory};
use memflow_win32::Kernel;

use simplelog::{LevelFilter, TermLogger, Config, TerminalMode };

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();

    let file = File::open("/dev/ax_dma_0").unwrap();

    let mut map = MemoryMap::new();

    map.push_remap(0x0.into(), !0, 0x0.into());
    let connector = FileIOMemory::try_with_reader(file, map)?;

    let kernel = Kernel::builder(connector).build().unwrap();

    println!("{:?}", kernel);

    return Ok(());
}
