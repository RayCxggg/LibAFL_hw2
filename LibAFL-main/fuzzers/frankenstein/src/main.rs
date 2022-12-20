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
    QemuHooks,
    QemuInstrumentationFilter,
    Regs,
};

#[cfg(target_os = "linux")]
pub fn main() {
    let corpus_dirs = [PathBuf::from("./corpus")];
    let objective_dir = PathBuf::from("./crashes");
}
