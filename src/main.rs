#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use urllogpass::reading::init;


fn main() -> std::io::Result<()> {
    init()?;
    Ok(())
}