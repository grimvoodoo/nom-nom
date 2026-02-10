//! Unit tests for ThemeToggle helper functions.
//!
//! These tests verify the pure logic used by the ThemeToggle component:
//! - Theme initialization from localStorage values
//! - Theme string conversion for DOM and localStorage
//! - Theme toggle state transitions
//! - Graceful handling of missing/empty localStorage values

use nom_nom::theme::{is_theme_light, theme_to_string, toggle_theme_state};

// Test 1: is_theme_light initializes correctly from localStorage values
#[test]
fn test_is_theme_light_with_light_value() {
    assert!(is_theme_light(Some("light")));
}

#[test]
fn test_is_theme_light_with_dark_value() {
    assert!(!is_theme_light(Some("dark")));
}

#[test]
fn test_is_theme_light_with_other_value() {
    // Any value other than "light" should be treated as dark
    assert!(!is_theme_light(Some("custom")));
    assert!(!is_theme_light(Some("LIGHT"))); // Case sensitive
}

// Test 2 & 3: theme_to_string returns correct value for DOM and localStorage
#[test]
fn test_theme_to_string_light() {
    assert_eq!(theme_to_string(true), "light");
}

#[test]
fn test_theme_to_string_dark() {
    assert_eq!(theme_to_string(false), "dark");
}

// Test 4: toggle_theme_state correctly switches between themes
#[test]
fn test_toggle_from_dark_to_light() {
    let is_light = false;
    let new_state = toggle_theme_state(is_light);
    assert!(new_state);
    assert_eq!(theme_to_string(new_state), "light");
}

#[test]
fn test_toggle_from_light_to_dark() {
    let is_light = true;
    let new_state = toggle_theme_state(is_light);
    assert!(!new_state);
    assert_eq!(theme_to_string(new_state), "dark");
}

#[test]
fn test_multiple_toggles() {
    let mut state = false; // Start dark
    state = toggle_theme_state(state);
    assert!(state); // Now light
    state = toggle_theme_state(state);
    assert!(!state); // Back to dark
    state = toggle_theme_state(state);
    assert!(state); // Light again
}

// Test 5: Handles empty or unavailable localStorage gracefully
#[test]
fn test_is_theme_light_with_none() {
    // When localStorage is unavailable or theme not set
    assert!(!is_theme_light(None));
}

#[test]
fn test_is_theme_light_with_empty_string() {
    // When theme is set but empty
    assert!(!is_theme_light(Some("")));
}

#[test]
fn test_default_theme_is_dark() {
    // Verify default behavior: when no theme is stored, default to dark
    let default_state = is_theme_light(None);
    assert!(!default_state);
    assert_eq!(theme_to_string(default_state), "dark");
}
