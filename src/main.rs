#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use urllogpass::reading::init;


fn main() -> std::io::Result<()> {
    init()?;
    println!("Закончил");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}