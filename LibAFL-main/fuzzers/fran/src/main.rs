use core::time::Duration;
use std::{env, path::PathBuf, process};

use libafl::{
    corpus::{Corpus, InMemoryCorpus, OnDiskCorpus},
    feedbacks::{CrashFeedback, MaxMapFeedback},
    generators::RandPrintablesGenerator,
    inputs::{BytesInput, HasTargetBytes},
};

use libafl_qemu::{
    drcov::QemuDrCovHelper,
    //asan::QemuAsanHelper,
    elf::EasyElf,
    emu::Emulator,
    QemuExecutor,
};

const XML_RPC_CALL: u32 = 0x087d8ba0;

#[cfg(target_os = "linux")]
pub fn main() {
    let corpus_dirs = [PathBuf::from("./corpus")];
    let objective_dir = PathBuf::from("./crashes");

    println!("running!");

    // Initialize QEMU
    // env::remove_var("LD_LIBRARY_PATH");
    let args = vec![
        "qemu-arm".to_string(),
        // "../../../frankenstein-dev/projects/CYW20735B1/gen/execute.exe".to_string(),
        "/opt/ccslab2/LibAFL_hw2/frankenstein-dev/projects/CYW20735B1/gen/execute.exe".to_string(),
    ];
    let env: Vec<(String, String)> = Vec::new();
    let emu = Emulator::new(&args, &env);

    emu.set_breakpoint(XML_RPC_CALL);
    unsafe {
        emu.run();
    }
    emu.remove_breakpoint(XML_RPC_CALL);

    println!("QEMU initialized");

    // // Create a QEMU in-process executor
    // let executor = QemuExecutor::new(
    //     &mut hooks,
    //     &mut harness,
    //     tuple_list!(edges_observer, time_observer),
    //     &mut fuzzer,
    //     &mut state,
    //     &mut mgr,
    // )
    // .expect("Failed to create QemuExecutor");
}
