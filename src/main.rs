use anyhow::Result;
use freyja::mount::mount;

mod freyja;

fn main() -> Result<()> {
  mount();
  Ok(())
}
