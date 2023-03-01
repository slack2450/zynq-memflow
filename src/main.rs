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

    let mut process = kernel.into_process_by_name("csgo.exe").unwrap();

    let modules = process.module_list().unwrap();

    for module in modules {
        println!("module: {:#?}", module.name);
    }

    let module = process.module_by_name("client_panorama.dll").unwrap();

    println!("module: {:#?}", module);

    let i_health: Address = module.base + 0xDEA964 + 0x100;

    loop {
        let health = process.read_raw(i_health, 4).unwrap();
        println!("health: {:#?}", health);
        std::io::stdin();
    }

    return Ok(());
}

pub fn get_line() -> std::result::Result<String, std::io::Error> {
    let mut output = String::new();
    std::io::stdin().read_line(&mut output).map(|_| output)
}

pub fn parse_input(input: &str) -> Option<(Box<[u8]>, String)> {
    None
}