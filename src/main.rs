use anyhow::Result;

fn main() -> Result<()> {
    // Entry point - minimal for now
    // Will be wired up in Phase 6
    println!("demoji v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

