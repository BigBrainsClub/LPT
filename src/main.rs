#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use urllogpass::init;


fn main() -> std::io::Result<()> {
    init()?;
    Ok(())
}