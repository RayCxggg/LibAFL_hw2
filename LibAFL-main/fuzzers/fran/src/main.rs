#[cfg(target_os = "linux")]
mod fuzzer;

#[cfg(target_os = "linux")]
pub fn main() {
    fuzzer::fuzz();
}
