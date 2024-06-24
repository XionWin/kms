//! A couple of functions to enable and disable coloring.

use std::default::Default;
use std::env;
use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;

/// Sets a flag to the console to use a virtual terminal environment.
///
/// This is primarily used for Windows 10 environments which will not correctly colorize
/// the outputs based on ANSI escape codes.
///
/// The returned `Result` is _always_ `Ok(())`, the return type was kept to ensure backwards
/// compatibility.
///
/// # Notes
/// > Only available to `Windows` build targets.
///
/// # Example
/// ```rust
/// use colored::*;
/// control::set_virtual_terminal(false).unwrap();
/// println!("{}", "bright cyan".bright_cyan());    // will print '[96mbright cyan[0m' on windows 10
///
/// control::set_virtual_terminal(true).unwrap();
/// println!("{}", "bright cyan".bright_cyan());    // will print correctly
/// ```
#[allow(clippy::result_unit_err)]
#[cfg(windows)]
pub fn set_virtual_terminal(use_virtual: bool) -> Result<(), ()> {
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        STD_OUTPUT_HANDLE,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut original_mode = 0;
        GetConsoleMode(handle, &mut original_mode);

        let enabled = original_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING
            == ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        match (use_virtual, enabled) {
            // not enabled, should be enabled
            (true, false) => {
                SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING | original_mode)
            }
            // already enabled, should be disabled
            (false, true) => {
                SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING ^ original_mode)
            }
            _ => 0,
        };
    }

    Ok(())
}

/// A flag for whether coloring should occur.
pub struct ShouldColorize {
    clicolor: bool,
    clicolor_force: Option<bool>,
    // XXX we can't use Option<Atomic> because we can't use &mut references to ShouldColorize
    has_manual_override: AtomicBool,
    manual_override: AtomicBool,
}

/// Use this to force colored to ignore the environment and always/never colorize
/// See example/control.rs
pub fn set_override(override_colorize: bool) {
    SHOULD_COLORIZE.set_override(override_colorize)
}

/// Remove the manual override and let the environment decide if it's ok to colorize
/// See example/control.rs
pub fn unset_override() {
    SHOULD_COLORIZE.unset_override()
}

pub(crate) static SHOULD_COLORIZE: Lazy<ShouldColorize> = Lazy::new(|| ShouldColorize::from_env());

impl Default for ShouldColorize {
    fn default() -> ShouldColorize {
        ShouldColorize {
            clicolor: true,
            clicolor_force: None,
            has_manual_override: AtomicBool::new(false),
            manual_override: AtomicBool::new(false),
        }
    }
}

impl ShouldColorize {
    /// Reads environment variables and checks if output is a tty to determine
    /// whether colorization should be used or not.
    /// `CLICOLOR_FORCE` takes highest priority, followed by `NO_COLOR`,
    /// followed by `CLICOLOR` combined with tty check.
    pub fn from_env() -> Self {
        ShouldColorize {
            clicolor: ShouldColorize::normalize_env(env::var("CLICOLOR")).unwrap_or(true)
                && io::stdout().is_terminal(),
            clicolor_force: ShouldColorize::resolve_clicolor_force(
                env::var("NO_COLOR"),
                env::var("CLICOLOR_FORCE"),
            ),
            ..ShouldColorize::default()
        }
    }

    /// Returns if the current coloring is expected.
    pub fn should_colorize(&self) -> bool {
        if self.has_manual_override.load(Ordering::Relaxed) {
            return self.manual_override.load(Ordering::Relaxed);
        }

        if let Some(forced_value) = self.clicolor_force {
            return forced_value;
        }

        self.clicolor
    }

    /// Use this to force colored to ignore the environment and always/never colorize
    pub fn set_override(&self, override_colorize: bool) {
        self.has_manual_override.store(true, Ordering::Relaxed);
        self.manual_override
            .store(override_colorize, Ordering::Relaxed);
    }

    /// Remove the manual override and let the environment decide if it's ok to colorize
    pub fn unset_override(&self) {
        self.has_manual_override.store(false, Ordering::Relaxed);
    }

    /* private */

    fn normalize_env(env_res: Result<String, env::VarError>) -> Option<bool> {
        match env_res {
            Ok(string) => Some(string != "0"),
            Err(_) => None,
        }
    }

    fn resolve_clicolor_force(
        no_color: Result<String, env::VarError>,
        clicolor_force: Result<String, env::VarError>,
    ) -> Option<bool> {
        if ShouldColorize::normalize_env(clicolor_force) == Some(true) {
            Some(true)
        } else if ShouldColorize::normalize_env(no_color).is_some() {
            Some(false)
        } else {
            None
        }
    }
}
