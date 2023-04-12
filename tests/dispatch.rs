use std::io;
use tempfile::TempDir;
use tenet::dispatch_external_git;


#[test]
fn test_basic_dispatch() -> io::Result<()> {
	let tmp_dir = TempDir::new()?;
	std::env::set_current_dir(tmp_dir.path())?;
	dispatch_external_git("git", &["init"])?;
	Ok(())
}
