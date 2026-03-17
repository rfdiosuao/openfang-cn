//! Progress bars and spinners for CLI output.
//!
//! Uses raw ANSI escape sequences (no external dependency). Supports:
//! - Percentage progress bar with visual block characters
//! - Spinner with label
//! - OSC 9;4 terminal progress protocol (ConEmu/Windows Terminal/iTerm2)
//! - Delay suppression for fast operations

use std::io::{self, Write};
use std::time::{Duration, Instant};

/// Default progress bar width (in characters).
const DEFAULT_BAR_WIDTH: usize = 30;

/// Minimum elapsed time before showing progress output. Operations that
/// complete faster than this threshold produce no visual noise.
const DELAY_SUPPRESS_MS: u64 = 200;

/// Block characters for the progress bar.
const FILLED: char = '\u{2588}'; // █
const EMPTY: char = '\u{2591}'; // ░

/// Spinner animation frames.
const SPINNER_FRAMES: &[char] = &[
    '\u{280b}', '\u{2819}', '\u{2839}', '\u{2838}', '\u{283c}', '\u{2834}', '\u{2826}', '\u{2827}',
    '\u{2807}', '\u{280f}',
];

// ---------------------------------------------------------------------------
// OSC 9;4 progress protocol
// ---------------------------------------------------------------------------

/// Emit an OSC 9;4 progress sequence (supported by Windows Terminal, ConEmu,
/// iTerm2). `state`: 1 = set progress, 2 = error, 3 = indeterminate, 0 = clear.
fn osc_progress(state: u8, percent: u8) {
    // ESC ] 9 ; 4 ; state ; percent ST
    // ST = ESC \   (string terminator)
    let _ = write!(io::stderr(), "\x1b]9;4;{state};{percent}\x1b\\");
    let _ = io::stderr().flush();
}

/// Clear the OSC 9;4 progress indicator.
fn osc_progress_clear() {
    osc_progress(0, 0);
}

// ---------------------------------------------------------------------------
// ProgressBar
// ---------------------------------------------------------------------------

/// A simple percentage-based progress bar.
///
/// ```text
/// Downloading   [████████████░░░░░░░░░░░░░░░░░░]  40% (4/10)
/// ```
pub struct ProgressBar {
    label: String,
    total: u64,
    current: u64,
    width: usize,
    start: Instant,
    suppress_until: Duration,
    visible: bool,
    use_osc: bool,
}

impl ProgressBar {
    /// Create a new progress bar.
    ///
    /// `label`: text shown before the bar.
    /// `total`: the 100% value.
    pub fn new(label: &str, total: u64) -> Self {
        Self {
            label: label.to_string(),
            total: total.max(1),
            current: 0,
            width: DEFAULT_BAR_WIDTH,
            start: Instant::now(),
            suppress_until: Duration::from_millis(DELAY_SUPPRESS_MS),
            visible: false,
            use_osc: true,
        }
    }

    /// Set the bar width in characters.
    pub fn width(mut self, w: usize) -> Self {
        self.width = w.max(5);
        self
    }

    /// Disable delay suppression (always show immediately).
    pub fn no_delay(mut self) -> Self {
        self.suppress_until = Duration::ZERO;
        self
    }

    /// Disable OSC 9;4 terminal progress protocol.
    pub fn no_osc(mut self) -> Self {
        self.use_osc = false;
        self
    }

    /// Update progress to `n`.
    pub fn set(&mut self, n: u64) {
        self.current = n.min(self.total);
        self.draw();
    }

    /// Increment progress by `delta`.
    pub fn inc(&mut self, delta: u64) {
        self.current = (self.current + delta).min(self.total);
        self.draw();
    }

    /// Mark as finished and clear the line.
    pub fn finish(&mut self) {
        self.current = self.total;
        self.draw();
        if self.visible {
            // Move to next line
            eprintln!();
        }
        if self.use_osc {
            osc_progress_clear();
        }
    }

    /// Mark as finished with a message replacing the bar.
    pub fn finish_with_message(&mut self, msg: &str) {
        self.current = self.total;
        if self.visible {
            eprint!("\r\x1b[2K{msg}");
            eprintln!();
        } else if self.start.elapsed() >= self.suppress_until {
            eprintln!("{msg}");
        }
        if self.use_osc {
            osc_progress_clear();
        }
    }

    fn draw(&mut self) {
        // Delay suppression: don't render if op is still fast
        if self.start.elapsed() < self.suppress_until && self.current < self.total {
            return;
        }

        self.visible = true;

        let pct = (self.current as f64 / self.total as f64 * 100.0) as u8;
        let filled = (self.current as f64 / self.total as f64 * self.width as f64) as usize;
        let empty = self.width.saturating_sub(filled);

        let bar: String = std::iter::repeat_n(FILLED, filled)
            .chain(std::iter::repeat_n(EMPTY, empty))
            .collect();

        eprint!(
            "\r\x1b[2K{:<14} [{}] {:>3}% ({}/{})",
            self.label, bar, pct, self.current, self.total
        );
        let _ = io::stderr().flush();

        if self.use_osc {
            osc_progress(1, pct);
        }
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        if self.use_osc && self.visible {
            osc_progress_clear();
        }
    }
}

// ---------------------------------------------------------------------------
// Spinner
// ---------------------------------------------------------------------------

/// An indeterminate spinner for operations without known total.
///
/// ```text
/// ⠋ Loading models...
/// ```
pub struct Spinner {
    label: String,
    frame: usize,
    start: Instant,
    suppress_until: Duration,
    visible: bool,
    use_osc: bool,
}

impl Spinner {
    /// Create a spinner with the given label.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            frame: 0,
            start: Instant::now(),
            suppress_until: Duration::from_millis(DELAY_SUPPRESS_MS),
            visible: false,
            use_osc: true,
        }
    }

    /// Disable delay suppression.
    pub fn no_delay(mut self) -> Self {
        self.suppress_until = Duration::ZERO;
        self
    }

    /// Disable OSC 9;4 protocol.
    pub fn no_osc(mut self) -> Self {
        self.use_osc = false;
        self
    }

    /// Advance the spinner by one frame and redraw.
    pub fn tick(&mut self) {
        if self.start.elapsed() < self.suppress_until {
            return;
        }

        self.visible = true;
        let ch = SPINNER_FRAMES[self.frame % SPINNER_FRAMES.len()];
        self.frame += 1;

        eprint!("\r\x1b[2K{ch} {}", self.label);
        let _ = io::stderr().flush();

        if self.use_osc {
            osc_progress(3, 0);
        }
    }

    /// Update the label text.
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    /// Stop the spinner and clear the line.
    pub fn finish(&self) {
        if self.visible {
            eprint!("\r\x1b[2K");
            let _ = io::stderr().flush();
        }
        if self.use_osc {
            osc_progress_clear();
        }
    }

    /// Stop the spinner and print a final message.
    pub fn finish_with_message(&self, msg: &str) {
        if self.visible {
            eprint!("\r\x1b[2K");
        }
        eprintln!("{msg}");
        if self.use_osc {
            osc_progress_clear();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.use_osc && self.visible {
            osc_progress_clear();
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_bar_percentage() {
        let mut pb = ProgressBar::new("Test", 10).no_delay().no_osc();
        pb.set(5);
        assert_eq!(pb.current, 5);
        pb.inc(3);
        assert_eq!(pb.current, 8);
        // Cannot exceed total
        pb.inc(100);
        assert_eq!(pb.current, 10);
    }

    #[test]
    fn progress_bar_zero_total_no_panic() {
        // total of 0 should be clamped to 1 to avoid division by zero
        let mut pb = ProgressBar::new("Empty", 0).no_delay().no_osc();
        pb.set(0);
        pb.finish();
        assert_eq!(pb.total, 1);
    }

    #[test]
    fn spinner_frame_advance() {
        let mut sp = Spinner::new("Loading").no_delay().no_osc();
        sp.tick();
        assert_eq!(sp.frame, 1);
        sp.tick();
        assert_eq!(sp.frame, 2);
        sp.finish();
    }

    #[test]
    fn delay_suppression() {
        // With default suppress_until, a freshly-created bar should NOT
        // become visible on the first draw (elapsed < 200ms).
        let mut pb = ProgressBar::new("Quick", 10).no_osc();
        pb.set(1);
        assert!(!pb.visible);
    }
}
