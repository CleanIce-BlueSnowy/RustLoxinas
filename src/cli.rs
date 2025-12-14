use sysinfo;

use crate::error;

pub struct CommandArgs {
    pub input_file: String,
}

pub fn get_arg_list() -> Vec<String> {
    std::env::args().collect()
}

pub fn parse_args(arg_list: Vec<String>) -> CommandArgs {
    if arg_list.len() == 1 {
        print_info();
        std::process::exit(0);
    } else {
        let mut input_file = None;
        for arg in arg_list.into_iter().skip(1) {
            match arg {
                arg => {
                    if let None = input_file {
                        input_file = Some(arg);
                    } else {
                        error::program_error(&format!("Unknown argument `{arg}`."));
                    }
                }
            }
        }
        CommandArgs {
            input_file: input_file.unwrap_or_else(|| error::program_error("Expect input file.")),
        }
    }
}

fn print_info() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    println!(
        "Loxinas (implemented by Rust) [System {system_name} {system_version} | Kernel {kernel_version}]",
        system_name = sysinfo::System::name().unwrap_or("Unknown".to_string()),
        system_version = sysinfo::System::os_version().unwrap_or("Unknown".to_string()),
        kernel_version = sysinfo::System::kernel_version().unwrap_or("Unknown".to_string()),
    );
}
