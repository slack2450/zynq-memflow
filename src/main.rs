use std::fs::File;

use memflow::prelude::v1::*;
use memflow::{prelude::{v1::{Result}, CloneFile}};
use memflow::mem::MemoryMap;
use memflow::connector::FileIoMemory;
use memflow_win32::win32::Win32Kernel;

use simplelog::{LevelFilter, TermLogger, Config, TerminalMode };

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();

    let file = CloneFile::from(File::open("/dev/ax_dma_0").unwrap());

    let mut map = MemoryMap::new();

    map.push_remap(0x0.into(), !0, 0x0.into());
    let connector = FileIoMemory::try_with_reader(file, map)?;

    let mut kernel = Win32Kernel::builder(connector).build().unwrap();

    println!("{:?}", kernel);

    kernel.process_info_list().unwrap().iter().for_each(|p| {
        println!("{:?}", p);
    });

    return Ok(());
}
