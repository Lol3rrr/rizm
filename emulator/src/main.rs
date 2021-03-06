use std::{
    collections::HashSet,
    io::{stdin, stdout, Write},
};

use emulator::{
    target::{self, CLIDebugger, EmptyDebugger},
    Emulator,
};
use g3a;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct RizmEmulate {
    #[structopt(short = "i")]
    input: String,
}

#[tokio::main]
async fn main() {
    let cmd = RizmEmulate::from_args();

    let raw_file = std::fs::read(cmd.input).unwrap();
    let file = match g3a::File::parse(&raw_file) {
        Ok(f) => f,
        Err(e) => panic!("Error parsing File: {:?}", e),
    };

    let mut breakpoints: HashSet<u32> = HashSet::new();

    let mut cli_input = target::CLIInput::new();
    let mut mock_display = emulator::MockDisplay::new();

    let mut em = Emulator::new(file, cli_input, mock_display);
    loop {
        let mut cli_in = String::new();
        stdout().write(&[b'>']).expect("Writing to Stdout");
        stdout().flush().expect("Flushing StdOut");
        stdin()
            .read_line(&mut cli_in)
            .expect("Could not get string");
        cli_in.remove(cli_in.len() - 1);

        let mut em_cmd = cli_in.split(" ");
        match em_cmd.next() {
            Some("run") => loop {
                if breakpoints.get(&em.pc()).is_some() {
                    println!("Reached Breakpoint");
                    break;
                }
                if let Err(e) = em.emulate_single().await {
                    println!("Error: {:?}", e);
                    break;
                }
            },
            Some("b") => {
                match em_cmd.next() {
                    Some(raw_br) => {
                        let br = if raw_br.starts_with("0x") {
                            let tmp = raw_br.strip_prefix("0x").unwrap();
                            u32::from_str_radix(tmp, 16).unwrap()
                        } else {
                            0
                        };

                        breakpoints.insert(br);
                        println!("Breakpoint: x{:X}", br);
                    }
                    None => println!("Expected-Breakpoint"),
                };
            }
            Some("step") => {
                if let Err(e) = em.emulate_single().await {
                    println!("Error: {:?}", e);
                }
            }
            Some("info") => {
                match em_cmd.next() {
                    Some("reg") => em.print_registers(),
                    Some("instr") => {
                        let current_instr = em.get_instr(0);
                        let next_instr = em.get_instr(2);
                        println!("Current Instruction: {:?}", current_instr,);
                        println!("Next Instruction: {:?}", next_instr);
                    }
                    Some("code") => em.print_code(None, None),
                    Some("stack") => em.print_stack(),
                    Some("memory") => {
                        // TODO
                        println!("Printing Memory");
                    }
                    _ => println!("Unknown"),
                };
            }
            Some("verbose") => {
                match em_cmd.next() {
                    Some("true") => em.set_debug(Box::new(CLIDebugger::new())),
                    Some("false") => em.set_debug(Box::new(EmptyDebugger::new())),
                    _ => println!("Unknown input"),
                };
            }
            Some("help") => {
                // TODO
                println!("Help:");
            }
            _ => {
                println!("Unknown");
            }
        };
    }
}
