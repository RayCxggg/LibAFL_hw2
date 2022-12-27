use libafl::monitors::SimpleMonitor;
use libafl::{
    bolts::{current_nanos, rands::StdRand, tuples::tuple_list, AsSlice},
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::SimpleEventManager,
    executors::{inprocess::InProcessExecutor, ExitKind},
    feedbacks::{CrashFeedback, MaxMapFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    generators::RandPrintablesGenerator,
    inputs::{BytesInput, HasTargetBytes},
    mutators::scheduled::{havoc_mutations, StdScheduledMutator},
    observers::StdMapObserver,
    schedulers::QueueScheduler,
    stages::mutational::StdMutationalStage,
    state::StdState,
};

use core::time::Duration;
use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
use std::{env, path::PathBuf, process};

#[cfg(target_os = "linux")]
pub fn main() {
    let corpus_dirs = [PathBuf::from("./corpus")];
    let objective_dir = PathBuf::from("./crashes");

    let mut harness = |input: &BytesInput| {
        let target = input.target_bytes();
        let buf = target.as_slice();
        // Command::new("qemu-arm")
        //     .arg("../../../frankenstein-dev/projects/CYW20735B1/gen/acl_fuzz.exe")
        //     .spawn()
        //     .unwrap();
        println!("Accepted input testcase: {:?}", buf);
        ExitKind::Ok
    };

    // The Monitor trait defines how the fuzzer stats are displayed to the user
    let mon = SimpleMonitor::new(|s| println!("{}", s));

    // The event manager handle the various events generated during the fuzzing loop
    // such as the notification of the addition of a new item to the corpus
    let mut mgr = SimpleEventManager::new(mon);

    // Feedback to rate the interestingness of an input
    // let mut feedback = MaxMapFeedback::new(&observer);

    // A feedback to choose if an input is a solution or not
    let mut objective = CrashFeedback::new();

    // A queue policy to get testcasess from the corpus
    let scheduler = QueueScheduler::new();

    // create a State from scratch
    let mut state = StdState::new(
        // RNG
        StdRand::with_seed(current_nanos()),
        // Corpus that will be evolved, we keep it in memory for performance
        InMemoryCorpus::new(),
        // Corpus in which we store solutions (crashes in this example),
        // on disk so the user can get them after stopping the fuzzer
        OnDiskCorpus::new(objective_dir).unwrap(),
        &mut (),
        &mut objective,
    )
    .unwrap();

    // A fuzzer with feedbacks and a corpus scheduler
    let mut fuzzer = StdFuzzer::new(scheduler, (), objective);

    // Create the executor for an in-process function
    let mut executor = InProcessExecutor::new(&mut harness, (), &mut fuzzer, &mut state, &mut mgr)
        .expect("Failed to create the Executor");

    // Generator of printable bytearrays of max size 32
    // let mut generator = RandPrintablesGenerator::new(32);

    // Generate 8 initial inputs
    state
        .load_initial_inputs(&mut fuzzer, &mut executor, &mut mgr, corpus_dirs)
        .unwrap_or_else(|_| panic!("Failed to load initial corpus at {:?}", &corpus_dirs));

    // Setup a mutational stage with a basic bytes mutator
    let mutator = StdScheduledMutator::new(havoc_mutations());
    let mut stages = tuple_list!(StdMutationalStage::new(mutator));

    // fuzz_loop will request a testcase for each iteration to the fuzzer using the
    // scheduler and then it will invoke the stage.
    fuzzer
        .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
        .expect("Error in the fuzzing loop");
}
