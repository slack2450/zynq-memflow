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

    let kernel = Win32Kernel::builder(connector).build().unwrap();

    let mut process = kernel.into_process_by_name("flag-bin.exe").unwrap();

    let module = process.module_by_name("flag-bin.exe").unwrap();

    println!("module: {:#?}", module);

    let target_string = b"There is nothing here!!!!";
    let replace_string = b"Hello world from memflow!";

    let base = module.base;
    for i in 0..module.size {
        let mut buf = vec![0; target_string.len()];
        process.virt_mem.read_raw_into(base + i, &mut buf).data_part()?;
        if target_string == buf.as_slice() {
            println!("found string at {:#x}", base + i);
            process.virt_mem.write_raw(base + i, replace_string)?;
            break;
        }
    }

    return Ok(());
}
