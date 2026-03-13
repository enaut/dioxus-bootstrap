# dioxus-bootstrap-css

Bootstrap 5.3 components for Dioxus — type-safe RSX wrappers powered by Bootstrap CSS.

## What is this?

A strict 1-to-1 mapping of Bootstrap 5.3 for Dioxus. Real Bootstrap CSS, real Bootstrap Icons,
zero JavaScript. Dioxus signals replace Bootstrap JS for interactive components.

> **Design rule:** If Bootstrap does it, we do it. If Bootstrap doesn't, we don't.
> See [docs/DESIGN.md](docs/DESIGN.md) for details.

## Quick start

```rust
use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

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
`Table`, `ListGroup`, `ListGroupItem`, `Spinner`, `Progress`, `ProgressBar`, `Pagination`, `Placeholder`

### Forms
`FormGroup`, `Input`, `Select`, `Textarea`, `Checkbox`, `Radio`, `InputGroup`, `InputGroupText`

### Interactive (signal-driven, no JS)
`Tabs`, `Tab`, `TabList`, `Modal`, `Dropdown`, `DropdownItem`, `Collapse`, `Accordion`, `AccordionItem`, `Toast`, `ToastContainer`, `Offcanvas`

### Navigation
`Navbar`, `NavbarToggler`, `NavbarCollapse`, `NavItem`, `NavLink`, `Breadcrumb`, `BreadcrumbItem`

## License

Apache 2.0 — Project Mycelium 2026
