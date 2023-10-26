use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"], &["protos"])?;

    Command::new("cargo").args(&["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto");
    Ok(())
}
