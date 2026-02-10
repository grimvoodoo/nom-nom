//! nom-nom library exports for testing.
//!
//! This module exposes pure helper functions that can be unit tested
//! without requiring the full Dioxus/web_sys environment.

/// Theme-related helper functions for the ThemeToggle component.
pub mod theme {
    /// Determines if the theme is light based on the stored theme string.
    /// Returns `false` (dark theme) if the value is None, empty, or not "light".
    pub fn is_theme_light(stored_theme: Option<&str>) -> bool {
        match stored_theme {
            Some(theme) if !theme.is_empty() => theme == "light",
            _ => false, // Default to dark theme
        }
    }

    /// Converts the boolean light mode flag to a theme string.
    pub fn theme_to_string(is_light: bool) -> &'static str {
        if is_light { "light" } else { "dark" }
    }

    /// Toggles the theme state and returns the new state.
    pub fn toggle_theme_state(current_is_light: bool) -> bool {
        !current_is_light
    }
}
