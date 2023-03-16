use std::fs::File;

use memflow::prelude::v1::*;
use memflow::{prelude::{v1::{Result}, CloneFile}};
use memflow::mem::MemoryMap;
use memflow::connector::FileIoMemory;
use memflow_win32::win32::{Win32Kernel, Win32VirtualTranslate, Win32Process};

use serde::Serialize;
use simplelog::{LevelFilter, TermLogger, Config, TerminalMode };

use url::Url;

use scanflow::value_scanner::ValueScanner;

#[derive(Serialize)]
pub struct WebsocketResponse<T> {
    pub action: String,
    pub success: bool,
    pub data: Option<Vec<T>>,
    pub error: Option<String>
}

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();

    let file = CloneFile::from(File::open("/dev/ax_dma_0").unwrap());

    let mut map = MemoryMap::new();

    map.push_remap(0x0.into(), !0, 0x0.into());
    let connector = FileIoMemory::try_with_reader(file, map)?;

    let mut kernel = Win32Kernel::builder(connector).build().unwrap();

    let url = Url::parse("ws://192.168.0.13:9001/device/device-1").unwrap();

    let (mut socket, _response) = tungstenite::connect(url).unwrap();

    let mut scanner = ValueScanner::default();

    let mut process_option: Option<Win32Process<FileIoMemory<CloneFile>, DirectTranslate, Win32VirtualTranslate>> = None;

    loop {
        if !socket.can_read() {
            continue;
        }

        let msg = socket.read_message().unwrap();

        if msg.is_text() {
            let msg = msg.to_text().unwrap();
            let msg: serde_json::Value = serde_json::from_str(msg).unwrap();
            if msg["action"].is_string() {
                match msg["action"].as_str().unwrap() {
                    "process-info-list" => {
                        let response: WebsocketResponse<ProcessInfo> = WebsocketResponse {
                            action: "process-info-list".to_string(),
                            success: true,
                            data: Some(kernel.process_info_list().unwrap()),
                            error: None
                        };
                        let info = serde_json::to_string(&response).unwrap();
                        socket.write_message(tungstenite::Message::Text(info)).unwrap();
                    },
                    "attach-process" => {
                        process_option = Some(kernel.clone().into_process_by_name(msg["name"].as_str().unwrap()).unwrap());
                        let response: WebsocketResponse<bool> = WebsocketResponse {
                            action: "attach-process".to_string(),
                            success: true,
                            data: None,
                            error: None
                        };
                        let info = serde_json::to_string(&response).unwrap();
                        socket.write_message(tungstenite::Message::Text(info)).unwrap();
                    },
                    "module-list" => {
                        if let Some(ref mut process) = process_option {
                            let respose: WebsocketResponse<ModuleInfo> = WebsocketResponse {
                                action: "module-info".to_string(),
                                success: true,
                                data: Some(process.module_list().unwrap()),
                                error: None
                            };
                            let info = serde_json::to_string(&respose).unwrap();
                            socket.write_message(tungstenite::Message::Text(info)).unwrap();
                        } else {
                            let respose: WebsocketResponse<ModuleInfo> = WebsocketResponse {
                                action: "module-info".to_string(),
                                success: false,
                                data: None,
                                error: Some("no process attached".to_string())
                            };
                            let info = serde_json::to_string(&respose).unwrap();
                            socket.write_message(tungstenite::Message::Text(info)).unwrap();
                        }
                    },
                    "reset" => {
                        scanner.reset();
                        process_option = None;
                    },
                    "search" => {
                        if let Some(ref mut process) = process_option {
                            if let Some(data) = parse_input(msg["value"].as_str().unwrap(), msg["type"].as_str().unwrap()) {
                                println!("searching for {:#?} of type {:#?}", msg["value"], msg["type"]);
                                scanner.scan_for(process, &data).unwrap();
                                let response: WebsocketResponse<Address> = WebsocketResponse {
                                    action: "search-result".to_string(),
                                    success: true,
                                    data: Some(scanner.matches().to_vec()),
                                    error: None
                                };
                                let info = serde_json::to_string(&response).unwrap();
                                socket.write_message(tungstenite::Message::Text(info)).unwrap();
                            } else {
                                println!("unknown type: {:#?}", msg["type"].as_str().unwrap());
                            }
                        }
                    },
                    _ => {
                        println!("unknown action: {:#?}", msg["action"].as_str().unwrap());
                    }
                }
            }
        }

        if msg.is_close() {
            break;
        }
    }

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

// Stolen shamelessly from scanflow

type PrintFn = fn(&[u8]) -> Option<String>;
type ParseFn = fn(&str) -> Option<Box<[u8]>>;
pub struct Type(&'static str, Option<usize>, PrintFn, ParseFn);

const TYPES: &[Type] = &[
    Type(
        "str",
        None,
        |buf| Some(String::from_utf8_lossy(buf).to_string()),
        |value| Some(Box::from(value.as_bytes())),
    ),
    Type(
        "str_utf16",
        None,
        |buf| {
            let mut vec = vec![];
            for w in buf.chunks_exact(2) {
                let s = u16::from_ne_bytes(w.try_into().unwrap());
                vec.push(s);
            }
            Some(format!("{}", String::from_utf16_lossy(&vec)))
        },
        |value| {
            let mut out = vec![];
            for v in value.encode_utf16() {
                out.extend(v.to_ne_bytes().iter().copied());
            }
            Some(out.into_boxed_slice())
        },
    ),
    Type(
        "i128",
        Some(16),
        |buf| Some(format!("{}", i128::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<i128>().ok()?.to_ne_bytes())),
    ),
    Type(
        "i64",
        Some(8),
        |buf| Some(format!("{}", i64::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<i64>().ok()?.to_ne_bytes())),
    ),
    Type(
        "i32",
        Some(4),
        |buf| Some(format!("{}", i32::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<i32>().ok()?.to_ne_bytes())),
    ),
    Type(
        "i16",
        Some(2),
        |buf| Some(format!("{}", i16::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<i16>().ok()?.to_ne_bytes())),
    ),
    Type(
        "i8",
        Some(1),
        |buf| Some(format!("{}", i8::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<i8>().ok()?.to_ne_bytes())),
    ),
    Type(
        "u128",
        Some(16),
        |buf| Some(format!("{}", u128::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<u128>().ok()?.to_ne_bytes())),
    ),
    Type(
        "u64",
        Some(8),
        |buf| Some(format!("{}", u64::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<u64>().ok()?.to_ne_bytes())),
    ),
    Type(
        "u32",
        Some(4),
        |buf| Some(format!("{}", u32::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<u32>().ok()?.to_ne_bytes())),
    ),
    Type(
        "u16",
        Some(2),
        |buf| Some(format!("{}", u16::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<u16>().ok()?.to_ne_bytes())),
    ),
    Type(
        "u8",
        Some(1),
        |buf| Some(format!("{}", u8::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<u8>().ok()?.to_ne_bytes())),
    ),
    Type(
        "f64",
        Some(4),
        |buf| Some(format!("{}", f64::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<f64>().ok()?.to_ne_bytes())),
    ),
    Type(
        "f32",
        Some(4),
        |buf| Some(format!("{}", f32::from_ne_bytes(buf.try_into().ok()?))),
        |value| Some(Box::from(value.parse::<f32>().ok()?.to_ne_bytes())),
    ),
];

pub fn parse_input(input: &str, typename: &str) -> Option<Box<[u8]>> {

    let b =  TYPES
        .iter()
        .filter(|Type(name, _, _, _)| name == &typename)
        .next()?
        .3(input)?;

    Some(b)
}