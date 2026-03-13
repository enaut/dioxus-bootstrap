# dioxus-bootstrap

Bootstrap 5.3 components for Dioxus — type-safe RSX wrappers powered by Bootstrap CSS.

## What is this?

A Rust crate that provides Dioxus components matching Bootstrap 5.3.3 — pixel-perfect,
offline-capable, zero JavaScript. Bootstrap's CSS does all the styling. Dioxus signals
replace Bootstrap's JS for interactive components (modals, tabs, dropdowns, collapse).

## Quick start

```rust
use dioxus::prelude::*;
use dioxus_bootstrap::prelude::*;

fn app() -> Element {
    rsx! {
        BootstrapHead {}
        Container {
            Row { class: "g-3",
                Col { lg: 6,
                    Card {
                        header: rsx! { "Hello" },
                        body: rsx! {
                            p { "Bootstrap in Dioxus — fully offline, fully Rust." }
                            Button { color: Color::Primary, "Get started" }
                        },
                    }
                }
            }
        }
    }
}
```

## Features

- All major Bootstrap 5.3 components as type-safe Dioxus components
- Zero JavaScript — interactive components (Modal, Tabs, Dropdown, Collapse) use Dioxus signals
- Offline-first — CSS and icon fonts bundled as static assets
- Works on all Dioxus platforms (web/WASM, desktop, mobile)
- Drop-in migration path from Bootstrap HTML templates

## Components

### Layout
`BootstrapHead`, `Container`, `Row`, `Col`

### Content
`Button`, `ButtonGroup`, `Card`, `Alert`, `Badge`, `Icon`

### Data
`Table`, `ListGroup`, `ListGroupItem`, `Spinner`, `Progress`, `ProgressBar`

### Forms
`FormGroup`, `Input`, `Select`, `Textarea`, `Checkbox`, `Radio`

### Interactive (signal-driven, no JS)
`Tabs`, `TabPane`, `Modal`, `Dropdown`, `DropdownItem`, `Collapse`

### Navigation
`Navbar`, `NavItem`, `NavLink`, `Breadcrumb`, `BreadcrumbItem`

## License

Apache 2.0 — Project Mycelium 2026
