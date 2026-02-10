# Styling Guide

nom-nom uses **Tailwind CSS** with **DaisyUI** for styling, integrated with **Dioxus** (Rust UI framework).

## Stack

- [Dioxus](https://dioxuslabs.com/) — Rust UI framework (RSX syntax)
- [Tailwind CSS](https://tailwindcss.com/) — Utility-first CSS framework
- [DaisyUI](https://daisyui.com/) — Component library for Tailwind with built-in theming

## Themes

- **Default theme**: Dark mode
- **Available themes**: `dark`, `light`
- Theme preference is stored in `localStorage` and persists across sessions

### How Theming Works

DaisyUI handles theming via the `data-theme` attribute on `<html>`.

In Dioxus, theme switching is handled via web-sys:

```rust path=null start=null
// Toggle theme in Rust/Dioxus
let theme = if is_light { "light" } else { "dark" };
if let Some(html) = document.document_element() {
    html.set_attribute("data-theme", theme);
}
storage.set_item("theme", theme);
```

See `src/components/theme_toggle.rs` for the full implementation.

### Using Theme Colors

Always use DaisyUI's semantic color classes instead of hardcoded colors:

```html
<!-- Good: adapts to theme -->
<div class="bg-base-100 text-base-content">
<button class="btn btn-primary">

<!-- Avoid: doesn't adapt to theme -->
<div class="bg-gray-900 text-white">
<button class="bg-blue-500">
```

Key semantic colors:
- `base-100`, `base-200`, `base-300` — Background colors
- `base-content` — Text color
- `primary`, `secondary`, `accent` — Brand colors
- `info`, `success`, `warning`, `error` — Status colors

## Development

### Install dependencies

```bash
npm install
```

### Build CSS (one-time)

```bash
npm run build:css
```

### Watch for changes (during development)

```bash
npm run watch:css
```

### File Structure

```
src/styles/input.css    # Tailwind directives (source)
assets/css/output.css   # Compiled CSS (generated, gitignored)
src/components/         # Dioxus components using Tailwind classes
src/pages/              # Page components
tailwind.config.js      # Tailwind + DaisyUI configuration
```

## Component Patterns

Use DaisyUI components in RSX syntax:

```rust path=null start=null
// Buttons
rsx! {
    button { class: "btn", "Default" }
    button { class: "btn btn-primary", "Primary" }
    button { class: "btn btn-outline", "Outline" }
}

// Cards
rsx! {
    div { class: "card bg-base-200 shadow-xl",
        div { class: "card-body",
            h2 { class: "card-title", "Title" }
            p { "Content" }
        }
    }
}

// Forms
rsx! {
    input { r#type: "text", class: "input input-bordered w-full" }
    select { class: "select select-bordered" }
}
```

See the [DaisyUI documentation](https://daisyui.com/components/) for the full component library.
