use sysinfo::System;

/// Result of checking for blocking applications
pub enum SuppressionResult {
    /// No blocking apps detected, proceed with playback
    Clear,

    /// A blocking app is running
    Blocked { app_name: String },
}

/// Handles process enumeration and matching
pub struct ProcessDetector {
    system: System,
}

impl ProcessDetector {
    /// Create a new detector, initializing the process list
    pub fn new() -> Self {
        Self {
            system: System::new(),
        }
    }

    /// Check if any blocklisted process is running.
    /// Returns the first matching process name, if any.
    pub fn check_blocklist(&mut self, blocklist: &[String]) -> SuppressionResult {
        self.system.refresh_processes();

        for process in self.system.processes().values() {
            let name = process.name().to_lowercase();

            for blocked in blocklist {
                if name.contains(&blocked.to_lowercase()) {
                    return SuppressionResult::Blocked {
                        app_name: process.name().to_string(),
                    };
                }
            }
        }

        SuppressionResult::Clear
    }
}
