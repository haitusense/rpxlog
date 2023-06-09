use anyhow::Context as _;
use thiserror::Error;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::backtrace::Backtrace;

#[derive(Error, Debug)]
pub enum FFIError {
	#[error("Panic : {0}")]
	Panic(String),

	#[error("failed to conv string to c_string")]
	CString(),

	#[error("failed to conv c_char to str")]
	CStr(),

	#[error("failed to {0}")]
	Method(String),
}

static LAST_PANIC_INFO: Lazy<RwLock<Option<(String, Backtrace)>>> = Lazy::new(|| RwLock::new(None));

pub fn ffi_result<F, T>(f: F) -> anyhow::Result<T> where F: FnOnce() -> anyhow::Result<T> + std::panic::UnwindSafe {

	// set hook
	let default_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
		*LAST_PANIC_INFO.write().unwrap() = Some(( 
			// format!("{:?}", panic_info) -> payload: Any { .. }, message: Some(hoge), location: Location { file: \"hoge.rs\", line: 133, col: 18 }, can_unwind: true }
			// format!("{}", panic_info.to_string() -> same as console out
			//   console    : thread '<unnamed>' panicked at 'panic!!!', src\ffi_c\test.rs:133:18
			//                note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
			//   panic_info : panicked at 'panic!!!', src\ffi_c\test.rs:133:18
			format!("{}", panic_info.to_string()
		), Backtrace::capture(), ));
    default_hook(panic_info);
  }));

	// catch_unwind
	let result = std::panic::catch_unwind(|| { f() });

	match result {
		Ok(n) => n,
		Err(_e) => {

			let last = self::LAST_PANIC_INFO.read().unwrap();
			let last = last.as_ref().unwrap();
			anyhow::bail!(FFIError::Panic(format!("{}", last.0)))

			// [not use LAST_PANIC_INFO]
			// if let Some(s) = e.downcast_ref::<&str>() { bail!("panicked in {} : {}", key, String::from(*s)) }
			// if let Some(s) = e.downcast_ref::<String>() { bail!("panicked in {} : {}", key, String::from(s.as_str())) }
			// bail!("panicked in {} : {}", key, String::from("Unknown error occurred."))
		},
	}
}
