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

#[cfg(target_os = "linux")]
pub fn main() {
    let corpus_dirs = [PathBuf::from("./corpus")];
    let objective_dir = PathBuf::from("./crashes");

    // Initialize QEMU
    // env::remove_var("LD_LIBRARY_PATH");
    let args = vec![
        "qemu-arm".to_string(),
        "../../../frankenstein-dev/projects/CYW20735B1/gen/execute.exe".to_string(),
    ];
    let env: Vec<(String, String)> = Vec::new();
    let emu = Emulator::new(&args, &env);

    unsafe {
        emu.run();
    }

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
