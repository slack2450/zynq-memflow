use std::fs::File;

use memflow::prelude::v1::*;
use memflow::{prelude::{v1::{Result}, CloneFile}};
use memflow::mem::MemoryMap;
use memflow::connector::FileIoMemory;
use memflow_win32::win32::Win32Kernel;

use simplelog::{LevelFilter, TermLogger, Config, TerminalMode };

use url::Url;

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();

    let file = CloneFile::from(File::open("/dev/ax_dma_0").unwrap());

    let mut map = MemoryMap::new();

    map.push_remap(0x0.into(), !0, 0x0.into());
    let connector = FileIoMemory::try_with_reader(file, map)?;

    let mut kernel = Win32Kernel::builder(connector).build().unwrap();

    let url = Url::parse("ws://192.168.0.13:9001/device").unwrap();

    let (mut socket, _response) = tungstenite::connect(url).unwrap();

    loop {
        let msg = socket.read_message().unwrap();

        let msg = match msg {
            tungstenite::Message::Text(text) => text,
            _ => panic!("unexpected message"),
        };

        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();

        if parsed["action"] == "process-list" {
            let process_info_list = kernel.process_info_list().unwrap();

            let info = serde_json::to_string(&process_info_list).unwrap();

            socket.write_message(tungstenite::Message::Text(info)).unwrap();
        } else {
            println!("unknown action: {:#?}", parsed["action"]);
        }
    }

    //let message = client.recv_message().unwrap();

    //let json = serde_json::from_str(message.try_into().unwrap()).unwrap();



    //let mut process = kernel.into_process_by_name("csgo.exe").unwrap();

    /*let modules = process.module_list().unwrap();

    for module in modules {
        println!("module: {:#?}", module.name);
    }*/

    //let module = process.module_by_name("client.dll").unwrap();

    //println!("module: {:#?}", module);

    //println!("module base: {:#?}", module.base);

    //let pointer_to_local_player: Address = module.base + 0xDEA964;

    //println!("pointer to local player: {:#?}", pointer_to_local_player);

    //let local_player: Address = process.virt_mem.read_addr(pointer_to_local_player).unwrap();

    //println!("local player: {:#?}", local_player);

    //let local_player_health_addr: Address = local_player + 0x100;

    //println!("local player health addr: {:#?}", local_player_health_addr);

    /*loop {
        let health = process.virt_mem.read::<u32>(local_player_health_addr).unwrap();
        println!("health: {:#?}", health);
        let buf = &mut String::new();
        std::io::stdin().read_line(buf).unwrap();
    }*/

    return Ok(());
}
