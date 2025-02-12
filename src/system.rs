use std::process::Command;
use std::io::Result;
use std::thread;
use std::mem::MaybeUninit;

use num_cpus::get;
use winapi::um::psapi::PROCESS_MEMORY_COUNTERS;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::psapi::GetProcessMemoryInfo;


use crate::config::Config;

pub fn clear_screen() -> Result<()> {
    let (cmd, args) = if cfg!(target_os = "windows") {
        ("cmd", &["/c", "cls"][..])
    } else {
        ("clear", &[][..])
    };

    Command::new(cmd).args(args).status()?;
    Ok(())
}


pub fn get_threads(config: &Config) -> usize {
    if config.autothreads {
        thread::available_parallelism()
            .map(|n| n.get().saturating_sub(1))
            .unwrap_or_else(|_| get().saturating_sub(1))
    } else {
        config.threads as usize
    }
}

pub fn get_peak_memory_usage() -> u64 {
    let mut counters: PROCESS_MEMORY_COUNTERS = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe {
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        );
    }
    counters.PeakWorkingSetSize.try_into().unwrap()
}