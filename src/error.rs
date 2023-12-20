static mut FAILED: bool = false;
pub fn error(line: usize, msg: String) {
	eprintln!("{line}: {msg}");
	unsafe { FAILED = true; }
}
