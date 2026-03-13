# dioxus-bootstrap — Implementation Plan

Bootstrap 5.3 components for Dioxus — type-safe RSX wrappers powered by Bootstrap CSS.

## Approach

This crate does NOT rewrite Bootstrap's CSS. It wraps it. Each Dioxus component emits
the correct Bootstrap HTML structure with the correct class names. Bootstrap's CSS handles
all styling. Bootstrap's JS is replaced entirely by Dioxus signals for interactive components.

The result: pixel-perfect Bootstrap UI, fully offline, fully Rust, on every Dioxus platform
(web, desktop, mobile).

## Crate Structure

```
dioxus-bootstrap/
├── Cargo.toml                    # Workspace root
├── LICENSE                       # Apache 2.0
├── crates/
│   └── dioxus-bootstrap/         # Library crate (published to crates.io)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # Re-exports all modules
│           ├── assets/           # Static CSS/font assets
│           │   ├── bootstrap.min.css         # Bootstrap 5.3.3
│           │   └── bootstrap-icons.min.css   # Bootstrap Icons 1.11.3
│           ├── head.rs           # BootstrapHead component (loads CSS)
│           ├── grid.rs           # Container, Row, Col
│           ├── button.rs         # Button, ButtonGroup
│           ├── card.rs           # Card, CardHeader, CardBody, CardFooter
│           ├── alert.rs          # Alert
│           ├── badge.rs          # Badge
│           ├── spinner.rs        # Spinner
│           ├── progress.rs       # Progress, ProgressBar
│           ├── table.rs          # Table
│           ├── form.rs           # FormGroup, Input, Select, Textarea, Checkbox, Radio
│           ├── list_group.rs     # ListGroup, ListGroupItem
│           ├── tabs.rs           # Tabs, TabPane (signal-driven)
│           ├── modal.rs          # Modal (signal-driven)
│           ├── dropdown.rs       # Dropdown (signal-driven)
│           ├── collapse.rs       # Collapse (signal-driven)
│           ├── nav.rs            # Navbar, NavItem, NavLink
│           ├── breadcrumb.rs     # Breadcrumb
│           ├── icon.rs           # Icon (Bootstrap Icons wrapper)
│           └── types.rs          # Shared enums: Color, Size, etc.
├── examples/
│   ├── showcase/                 # Full example app showing all components
│   └── dashboard/                # Realistic dashboard example
└── docs/
    ├── PLAN.md                   # This file
    └── MIGRATION.md              # Guide for migrating Bootstrap HTML to dioxus-bootstrap
```

## Design Decisions

### D1: CSS Delivery — Static Assets (Offline-First)

Bundle `bootstrap.min.css` and `bootstrap-icons.min.css` as static assets inside the crate.
Provide a `BootstrapHead` component that loads them:

```rust
use dioxus_bootstrap::BootstrapHead;

rsx! {
    BootstrapHead {}  // Emits <link> tags for bundled CSS
    // ... your app
}
```

Rationale: offline support is a hard requirement. CDN is not an option for offline-capable
apps. Consumers who prefer CDN can skip `BootstrapHead` and link their own.

The CSS files are ~230KB (bootstrap) + ~100KB (icons font). Acceptable for WASM apps that
already ship multi-MB binaries.

### D2: Props Design — Typed Where It Matters, Strings Where It Doesn't

Component-specific semantics get typed enums:

```rust
// Typed — prevents typos, enables autocomplete
Button { color: Color::Primary, size: Size::Lg, onclick: handler }
Col { lg: 6, md: 12 }
Alert { color: Color::Danger, dismissible: true }
```

Arbitrary Bootstrap utility classes pass through a `class` prop:

```rust
// Raw string — for spacing, flex, display, etc.
Card { class: "mb-3 shadow-sm", ... }
Row { class: "g-3 align-items-center", ... }
```

Every component accepts an optional `class: &'a str` prop that appends to the generated
class string. This avoids an explosion of typed props for every utility class while keeping
the common cases type-safe.

### D3: Shared Type Enums

```rust
pub enum Color {
    Primary, Secondary, Success, Danger, Warning, Info, Light, Dark,
}

pub enum Size {
    Sm, Md, Lg,
}

pub enum ColumnSize {
    Auto,
    Span(u8),  // 1-12
}
```

These map to class suffixes: `Color::Primary` → `"primary"`, `Size::Lg` → `"lg"`.

### D4: Children & Slots

Use Dioxus `Element` for single-slot components and named props for multi-slot:

```rust
// Single slot — children
Card { body: rsx! { "Card content" } }

// Multi slot — named Element props
Card {
    header: rsx! { "Title" },
    body: rsx! { "Content" },
    footer: rsx! { "Actions" },
}
```

### D5: Interactive Components — Signals Replace Bootstrap JS

Bootstrap's JS components are simple state machines. Dioxus signals replace them:

**Tabs:**
```rust
let active_tab = use_signal(|| 0usize);
Tabs {
    active: active_tab,
    tabs: vec![
        TabDef { label: "Home", content: rsx! { "Home content" } },
        TabDef { label: "Profile", content: rsx! { "Profile content" } },
    ],
}
```

**Modal:**
```rust
let show_modal = use_signal(|| false);
Button { onclick: move |_| show_modal.set(true), "Open Modal" }
Modal {
    show: show_modal,
    title: "Confirm",
    body: rsx! { "Are you sure?" },
    footer: rsx! {
        Button { color: Color::Secondary, onclick: move |_| show_modal.set(false), "Cancel" }
        Button { color: Color::Primary, "Confirm" }
    },
}
```

**Dropdown:**
```rust
let open = use_signal(|| false);
Dropdown {
    open: open,
    toggle: rsx! { "Actions" },
    menu: rsx! {
        DropdownItem { onclick: handler1, "Edit" }
        DropdownItem { onclick: handler2, "Delete" }
    },
}
```

**Collapse:**
```rust
let expanded = use_signal(|| false);
Button { onclick: move |_| expanded.toggle(), "Toggle" }
Collapse {
    expanded: expanded,
    rsx! { "Collapsible content" },
}
```

### D6: Bootstrap Icons

Provide a thin wrapper. Don't bundle the font — it's loaded via `BootstrapHead`.

```rust
Icon { name: "shield-lock" }
// Renders: <i class="bi bi-shield-lock"></i>

Icon { name: "search", class: "me-2" }
// Renders: <i class="bi bi-search me-2"></i>
```

### D7: No Bootstrap JS Dependency

The crate ships zero JavaScript. All interactive behavior is pure Rust/Dioxus.
Consumers do NOT need `bootstrap.bundle.min.js`. This is a core design principle.

## Implementation Phases

### Phase 1: Foundation

Goal: Crate scaffold, asset bundling, core types, first component.

- [x] Workspace `Cargo.toml` with `crates/dioxus-bootstrap/`
- [x] Dioxus 0.7 dependency
- [x] `types.rs` — `Color`, `Size`, `ColumnSize` enums with `Display` impls
- [x] `assets/` — bundle `bootstrap.min.css` and `bootstrap-icons.min.css`
- [x] `head.rs` — `BootstrapHead` component
- [x] `button.rs` — `Button` and `ButtonGroup` as the proof-of-concept component
- [x] `icon.rs` — `Icon` component
- [x] `lib.rs` — re-export everything
- [x] Basic example app that renders buttons + icons with correct Bootstrap styling
- [x] Verify: loads offline, correct visual output, no JS needed

### Phase 2: CSS-Only Components

Goal: All layout and display components that need no interactive JS logic.

- [x] `grid.rs` — `Container`, `Row`, `Col` with responsive breakpoint props
- [x] `card.rs` — `Card` with header/body/footer slots
- [x] `alert.rs` — `Alert` with color + dismissible (dismiss is a signal, not JS)
- [x] `badge.rs` — `Badge` with color + pill option
- [x] `spinner.rs` — `Spinner` (border and grow variants)
- [x] `progress.rs` — `Progress` and `ProgressBar` with value/color props
- [x] `table.rs` — `Table` with striped/hover/bordered/responsive props
- [x] `form.rs` — `FormGroup`, `Input`, `Select`, `Textarea`, `Checkbox`, `Radio`
- [x] `list_group.rs` — `ListGroup`, `ListGroupItem`
- [x] Dashboard example using all Phase 2 components

### Phase 3: Interactive Components

Goal: Signal-driven replacements for Bootstrap JS components.

- [x] `tabs.rs` — `Tabs`, `TabPane` with active signal
- [x] `modal.rs` — `Modal` with show signal, backdrop click-to-close, ESC key handling
- [x] `dropdown.rs` — `Dropdown` with open signal, click-outside-to-close
- [x] `collapse.rs` — `Collapse` with expanded signal, CSS height transition
- [x] `alert.rs` update — wire dismissible alert to use Collapse internally
- [x] Showcase example demonstrating all interactive components

### Phase 4: Navigation Components

Goal: Navbar and structural navigation.

- [x] `nav.rs` — `Navbar`, `NavBrand`, `NavItem`, `NavLink`, `NavDropdown`
- [x] `nav.rs` — responsive collapse (hamburger menu) using Phase 3 Collapse
- [x] `breadcrumb.rs` — `Breadcrumb`, `BreadcrumbItem`
- [x] Full dashboard example with navbar + sidebar + tabs + cards

### Phase 5: Theme Integration & Migration Guide

Goal: CSS variable bridging for apps that use custom design systems, and a guide
for migrating existing Bootstrap HTML templates to dioxus-bootstrap.

- [x] Document the CSS custom property bridge pattern:
      how to map a parent app's CSS variables to Bootstrap's `--bs-*` variables
- [x] Document `data-bs-theme` light/dark mode switching from Dioxus
- [x] `docs/MIGRATION.md` — step-by-step guide for converting Bootstrap HTML
      templates (Tera, Askama, Jinja2) to dioxus-bootstrap RSX
- [x] Common patterns catalog: two-column layout, tabbed dashboard, card grid,
      form with validation, modal confirmation flow
- [x] Example: theme-aware component that responds to CSS variable changes

### Phase 6: Polish & Publish

Goal: Production-ready crate on crates.io.

- [x] API review — consistent naming, prop conventions, documentation
- [x] All components have doc comments with usage examples
- [x] All components have basic tests
- [x] README with quick-start, component gallery screenshots
- [x] `cargo publish` to crates.io
- [x] GitHub repo with CI (build + test + clippy)

## Component API Reference

Quick reference for all planned components and their key props.

### Layout
| Component | Key Props | Output |
|-----------|-----------|--------|
| `BootstrapHead` | — | `<link>` tags for CSS |
| `Container` | `fluid: bool, class` | `<div class="container[-fluid]">` |
| `Row` | `class` | `<div class="row {class}">` |
| `Col` | `xs, sm, md, lg, xl, xxl: ColumnSize, class` | `<div class="col-{bp}-{n}">` |

### Content
| Component | Key Props | Output |
|-----------|-----------|--------|
| `Button` | `color, size, outline, disabled, onclick, class` | `<button class="btn btn-{color}">` |
| `ButtonGroup` | `size, class` | `<div class="btn-group">` |
| `Card` | `header, body, footer, class` | `<div class="card">` |
| `Alert` | `color, dismissible, class` | `<div class="alert alert-{color}">` |
| `Badge` | `color, pill, class` | `<span class="badge bg-{color}">` |
| `Icon` | `name, class` | `<i class="bi bi-{name}">` |

### Data
| Component | Key Props | Output |
|-----------|-----------|--------|
| `Table` | `striped, hover, bordered, responsive, size, class` | `<table class="table">` |
| `ListGroup` | `flush, class` | `<ul class="list-group">` |
| `ListGroupItem` | `active, disabled, color, class` | `<li class="list-group-item">` |
| `Spinner` | `color, size, grow, class` | `<div class="spinner-border">` |
| `Progress` | `class` | `<div class="progress">` |
| `ProgressBar` | `value, color, striped, animated, class` | `<div class="progress-bar">` |

### Forms
| Component | Key Props | Output |
|-----------|-----------|--------|
| `FormGroup` | `label, class` | `<div class="mb-3"><label>...` |
| `Input` | `r#type, value, oninput, placeholder, size, class` | `<input class="form-control">` |
| `Select` | `value, onchange, size, class` | `<select class="form-select">` |
| `Textarea` | `value, oninput, rows, class` | `<textarea class="form-control">` |
| `Checkbox` | `checked, onchange, label, class` | `<div class="form-check">` |
| `Radio` | `checked, onchange, name, label, class` | `<div class="form-check">` |

### Interactive (Signal-Driven)
| Component | Key Props | Output |
|-----------|-----------|--------|
| `Tabs` | `active: Signal<usize>, class` | `<ul class="nav nav-tabs">` + panes |
| `TabPane` | `label, icon` | Tab button + content pane |
| `Modal` | `show: Signal<bool>, title, body, footer, size` | `<div class="modal">` |
| `Dropdown` | `open: Signal<bool>, toggle, menu, class` | `<div class="dropdown">` |
| `DropdownItem` | `onclick, class` | `<button class="dropdown-item">` |
| `Collapse` | `expanded: Signal<bool>, class` | `<div class="collapse">` |

### Navigation
| Component | Key Props | Output |
|-----------|-----------|--------|
| `Navbar` | `brand, color, expand, class` | `<nav class="navbar">` |
| `NavItem` | `active, class` | `<li class="nav-item">` |
| `NavLink` | `href, active, class` | `<a class="nav-link">` |
| `Breadcrumb` | `class` | `<nav><ol class="breadcrumb">` |
| `BreadcrumbItem` | `active, href, class` | `<li class="breadcrumb-item">` |

## Dependencies

```toml
[dependencies]
dioxus = { version = "0.7", default-features = false }
```

Minimal dependency footprint. The crate should work with any Dioxus platform feature
(web, desktop, mobile) — it only emits HTML/RSX, so it's platform-agnostic.

## Success Criteria

1. A Dioxus app using `dioxus-bootstrap` produces visually identical output to the same
   layout written in raw Bootstrap HTML
2. Zero JavaScript required — all interactivity is pure Rust
3. Works offline — all assets are bundled
4. Works on all Dioxus platforms (web WASM, desktop, mobile)
5. Existing Bootstrap HTML templates can be mechanically converted to RSX using
   dioxus-bootstrap components
