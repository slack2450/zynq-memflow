use cloneable_file::CloneableFile;
use memflow::{prelude::{v1::{Result}}};
use memflow::mem::MemoryMap;
use memflow::connector::FileIoMemory;
use memflow_win32::win32::Win32Kernel;

use simplelog::{LevelFilter, TermLogger, Config, TerminalMode };


fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    
    let file = CloneableFile::open("/dev/ax_dma_0").unwrap();

    let mut map = MemoryMap::new();

    map.push_remap(0x0.into(), !0, 0x0.into());
    let connector = FileIoMemory::try_with_reader(file, map)?;

    let kernel = Win32Kernel::builder(connector).no_symbol_store().build().unwrap();

    println!("{:?}", kernel);

    return Ok(());
}
