#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use lpt::init;


fn main() -> std::io::Result<()> {
    init()?;
    Ok(())
}