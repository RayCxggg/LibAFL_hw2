use std::path::PathBuf;
#[cfg(windows)]
use std::ptr::write_volatile;

#[cfg(feature = "tui")]
use libafl::monitors::tui::TuiMonitor;
#[cfg(not(feature = "tui"))]
use libafl::monitors::SimpleMonitor;
use libafl::{
    bolts::{current_nanos, rands::StdRand, tuples::tuple_list, AsSlice},
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::SimpleEventManager,
    executors::{inprocess::InProcessExecutor, ExitKind},
    feedbacks::{CrashFeedback, MaxMapFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    generators::RandBytesGenerator,
    inputs::{BytesInput, HasTargetBytes},
    mutators::scheduled::{havoc_mutations, StdScheduledMutator},
    observers::StdMapObserver,
    schedulers::QueueScheduler,
    stages::mutational::StdMutationalStage,
    state::StdState,
};

pub fn main() {
    let mut generator = RandBytesGenerator::new()
    let mut feedback = CrashFeedback::new();
    let scheduler = QueueScheduler::new();
}