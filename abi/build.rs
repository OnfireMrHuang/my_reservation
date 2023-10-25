use std::io::Result;
use std::{fs, process::Command};

fn main() -> Result<()> {
    tonic_build::configure()
        .out_dir("src/pb")
        .build_client(false)
        .compile(&["protos/reservation.proto"], &["protos"])?;
    // fs::remove_file("src/pb/google.protobuf.rs")?;

    Command::new("cargo").args(&["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto");
    Ok(())
}
