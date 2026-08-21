use std::process::{Command, Output};

pub const FEATURE_ID: &str = "MET-003";
pub const AZURE_WORK_ITEM: u32 = 243;
pub const RADON_REFERENCE: &str = "54b88e5878b2724bf4d77f97349588b811abdff2";

pub fn run_rusty_kode_without_arguments() -> Output {
    Command::new(env!("CARGO_BIN_EXE_rusty-kode"))
        .output()
        .expect("MET-003 acceptance binary should be executable")
}

pub fn combined_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

pub fn evidence_context() -> String {
    format!("feature={FEATURE_ID}, azure_work_item={AZURE_WORK_ITEM}, reference={RADON_REFERENCE}")
}
