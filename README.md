# dioxus-bootstrap-css

[![Crates.io](https://img.shields.io/crates/v/dioxus-bootstrap-css.svg)](https://crates.io/crates/dioxus-bootstrap-css)
[![License](https://img.shields.io/crates/l/dioxus-bootstrap-css.svg)](LICENSE)

A strict 1-to-1 mapping of [Bootstrap 5.3](https://getbootstrap.com/) for [Dioxus](https://dioxuslabs.com/).

Real Bootstrap CSS. Real Bootstrap Icons. Zero JavaScript. Dioxus signals replace Bootstrap JS.

> **Design rule:** If Bootstrap does it, we do it. If Bootstrap doesn't, we don't.

## Why?

- **Pixel-perfect** — Uses the actual Bootstrap 5.3.3 CSS, not a reimplementation
- **Zero JavaScript** — Modals, dropdowns, tabs, collapse, accordion, offcanvas — all driven by Dioxus signals
- **Offline-first** — CSS and icon fonts bundled as static assets via `asset!()`
- **Type-safe** — Bootstrap classes become Rust enums and props; no magic strings
- **Drop-in migration** — Convert Bootstrap HTML templates to RSX with minimal changes

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-bootstrap-css = "0.1.4"
```

```rust
use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let theme = use_signal(|| Theme::Dark);

    rsx! {
        ThemeProvider { theme: theme }
        BootstrapHead {}

        Container { class: "py-4",
            h1 { "Hello, Bootstrap!" }
            Row { class: "g-3",
                Col { md: ColumnSize::Span(6),
                    Card {
                        header: rsx! { "Getting Started" },
                        body: rsx! {
                            p { "Bootstrap in Dioxus — fully offline, fully Rust." }
                            Button { color: Color::Primary,
                                Icon { name: "rocket-takeoff", class: "me-1" }
                                "Launch"
                            }
                        },
                    }
                }
                Col { md: ColumnSize::Span(6),
                    Alert { color: Color::Success,
                        Icon { name: "check-circle", class: "me-2" }
                        "Everything works out of the box."
                    }
                }
            }
        }
    }
}
```

## Components

### Layout & Head
| Component | Description |
|-----------|-------------|
| `BootstrapHead` | Loads Bootstrap CSS + Icons (bundled, offline) |
| `ThemeProvider` | Sets `data-bs-theme` on `<html>` reactively |
| `ThemeToggle` | Dark/light mode toggle button |
| `Container` | `.container` / `.container-fluid` |
| `Row` | `.row` with gutter support |
| `Col` | Responsive columns (`xs`, `sm`, `md`, `lg`, `xl`, `xxl`) |

### Content
| Component | Description |
|-----------|-------------|
| `Button` | All Bootstrap button variants, sizes, outlines |
| `ButtonGroup` | Button groups |
| `Card` | Cards with `header`, `body`, `footer` slots |
| `Alert` | Dismissible alerts with all color variants |
| `Badge` | Badges and pills |
| `Icon` | Bootstrap Icons (`bi-{name}`) |
| `Spinner` | Border and grow spinners |
| `Progress` / `ProgressBar` | Progress bars (striped, animated, stacked) |
| `Placeholder` / `PlaceholderParagraph` | Loading placeholder elements |

### Data Display
| Component | Description |
|-----------|-------------|
| `Table` | Striped, hover, bordered, responsive tables |
| `ListGroup` / `ListGroupItem` | List groups (flush, active, disabled) |
| `Pagination` | Page navigation with ellipsis |

### Forms
| Component | Description |
|-----------|-------------|
| `FormGroup` | Form group wrapper |
| `Input` | Text inputs with all HTML types |
| `Select` | Select dropdowns |
| `Textarea` | Multi-line text input |
| `Checkbox` / `Radio` | Check and radio inputs |
| `InputGroup` / `InputGroupText` | Input groups with addons |

### Interactive (Signal-Driven)
| Component | Bootstrap JS Replaced |
|-----------|----------------------|
| `Modal` | `data-bs-toggle="modal"` |
| `Dropdown` / `DropdownItem` / `DropdownDivider` / `DropdownHeader` | `data-bs-toggle="dropdown"` + click-outside-to-close |
| `Collapse` | `data-bs-toggle="collapse"` |
| `Tabs` / `Tab` / `TabList` | Tab switching logic |
| `Accordion` / `AccordionItem` | Accordion toggle logic |
| `Offcanvas` | `data-bs-toggle="offcanvas"` |
| `Toast` / `ToastContainer` | Toast show/dismiss |

### Navigation
| Component | Description |
|-----------|-------------|
| `Navbar` | Responsive navbar with color schemes |
| `NavbarToggler` / `NavbarCollapse` | Mobile hamburger toggle |
| `NavItem` / `NavLink` | Nav items and links |
| `Breadcrumb` / `BreadcrumbItem` | Breadcrumb navigation |

## Dark Mode

```rust
let theme = use_signal(|| Theme::Dark);
rsx! {
    ThemeProvider { theme: theme }
    BootstrapHead {}
    ThemeToggle { theme: theme }  // sun/moon toggle button
}
```

`ThemeProvider` reactively sets `data-bs-theme` on `<html>` — Bootstrap handles the rest.

## How It Works

```
┌─────────────────────────────────────┐
│          Your Dioxus App            │
│  ┌───────────────────────────────┐  │
│  │    dioxus-bootstrap-css       │  │
│  │  ┌─────────┐ ┌─────────────┐ │  │
│  │  │Bootstrap │ │  Dioxus RSX │ │  │
│  │  │ 5.3 CSS  │ │  Components │ │  │
│  │  │ + Icons  │ │  + Signals  │ │  │
│  │  └─────────┘ └─────────────┘ │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

- **CSS** — Real `bootstrap.min.css` and `bootstrap-icons.min.css`, bundled via `asset!()`
- **Icons** — Font files inlined as base64 data URIs (works with Dioxus asset hashing)
- **Signals** — Replace all Bootstrap JS behaviors; no `<script>` tags needed
- **Components** — Type-safe RSX wrappers that emit standard Bootstrap HTML classes

## Roadmap

Not yet implemented:
- Carousel
- Tooltips / Popovers
- Scrollspy

## Examples

See the [`examples/`](https://github.com/mik-tf/dioxus-bootstrap/tree/development/examples) directory:

- **showcase** — Every component demonstrated in a tabbed interface
- **dashboard** — Realistic admin dashboard with navbar, tables, modals, charts

## Migration from Bootstrap HTML

See [docs/MIGRATION.md](https://github.com/mik-tf/dioxus-bootstrap/blob/development/docs/MIGRATION.md) for a complete HTML-to-RSX migration guide.

## License

Apache 2.0 — Project Mycelium 2026
