# dioxus-bootstrap-css

[![Crates.io](https://img.shields.io/crates/v/dioxus-bootstrap-css.svg)](https://crates.io/crates/dioxus-bootstrap-css)
[![License](https://img.shields.io/crates/l/dioxus-bootstrap-css.svg)](LICENSE)
[![CI](https://github.com/mik-tf/dioxus-bootstrap/actions/workflows/ci.yml/badge.svg)](https://github.com/mik-tf/dioxus-bootstrap/actions/workflows/ci.yml)

Complete 1-to-1 [Bootstrap 5.3](https://getbootstrap.com/) component library for [Dioxus](https://dioxuslabs.com/). 50+ components covering every Bootstrap CSS class and JS behavior — modals, dropdowns, carousel, accordion, offcanvas, tooltips, and more — all driven by Dioxus signals. Zero JavaScript. Offline-first. Type-safe.

**[Live Showcase](https://mik-tf.github.io/dioxus-bootstrap/)** | **[API Docs](https://docs.rs/dioxus-bootstrap-css)**

> **Design rule:** If Bootstrap does it, we do it. If Bootstrap doesn't, we don't.

## Why?

- **Pixel-perfect** — Uses the actual Bootstrap 5.3.3 CSS, not a reimplementation
- **Zero JavaScript** — Modals, dropdowns, tabs, collapse, accordion, offcanvas, carousel — all driven by Dioxus signals
- **Offline-first** — CSS and icon fonts bundled as static assets via `asset!()`
- **Type-safe** — Bootstrap classes become Rust enums and props; no magic strings
- **Drop-in migration** — Convert Bootstrap HTML templates to RSX with minimal changes

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-bootstrap-css = "0.1.9"
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
| `ButtonGroup` / `ButtonToolbar` | Button groups and toolbars |
| `Card` | Cards with `header`, `body`, `footer` slots |
| `Alert` | Dismissible alerts with all color variants |
| `Badge` | Badges and pills |
| `Icon` | Bootstrap Icons (`bi-{name}`) |
| `Spinner` | Border and grow spinners |
| `Progress` / `ProgressBar` | Progress bars (striped, animated, stacked) |
| `Placeholder` / `PlaceholderParagraph` | Loading placeholder elements |
| `Figure` | Figures with captions |
| `Ratio` | Responsive aspect ratio embeds |

### Data Display
| Component | Description |
|-----------|-------------|
| `Table` | Striped, striped-columns, hover, bordered, responsive tables |
| `ListGroup` / `ListGroupItem` | List groups (flush, active, disabled, colored) |
| `Pagination` | Page navigation with ellipsis |

### Forms
| Component | Description |
|-----------|-------------|
| `FormGroup` | Form group wrapper |
| `Input` | Text inputs with all HTML types |
| `Select` | Select dropdowns |
| `Textarea` | Multi-line text input |
| `Checkbox` / `Radio` / `Switch` | Check, radio, and switch inputs |
| `Range` | Range slider |
| `FloatingLabel` | Floating label inputs |
| `InputGroup` / `InputGroupText` | Input groups with addons |
| `FormFeedback` / `FormText` | Validation feedback and help text |

### Interactive (Signal-Driven, Zero JS)
| Component | Bootstrap JS Replaced |
|-----------|----------------------|
| `Modal` | `data-bs-toggle="modal"` (sizes, fullscreen, centered, scrollable) |
| `Dropdown` / `DropdownItem` / `DropdownDivider` / `DropdownHeader` | `data-bs-toggle="dropdown"` + click-outside-to-close |
| `Collapse` | `data-bs-toggle="collapse"` |
| `Tabs` / `Tab` / `TabList` | Tab switching (pills, fill, justified, vertical) |
| `Accordion` / `AccordionItem` | Accordion toggle logic |
| `Offcanvas` | `data-bs-toggle="offcanvas"` (placements, responsive) |
| `Toast` / `ToastContainer` | Toast show/dismiss |
| `Carousel` | Slide/fade transitions, auto-play, pause-on-hover, keyboard nav, touch swipe |
| `Tooltip` | CSS-positioned tooltips (top, bottom, start, end) |
| `Popover` | Click-to-toggle popovers with click-outside-to-close |
| `Scrollspy` | Scroll-aware section tracking via signals |

### Navigation
| Component | Description |
|-----------|-------------|
| `Navbar` | Responsive navbar with color schemes |
| `NavbarToggler` / `NavbarCollapse` | Mobile hamburger toggle |
| `Nav` / `NavItem` / `NavLink` | Nav (pills, tabs, underline, fill, justified, vertical) |
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
+-------------------------------------+
|          Your Dioxus App            |
|  +-------------------------------+  |
|  |    dioxus-bootstrap-css       |  |
|  |  +-----------+ +------------+ |  |
|  |  | Bootstrap | | Dioxus RSX | |  |
|  |  | 5.3 CSS   | | Components | |  |
|  |  | + Icons   | | + Signals  | |  |
|  |  +-----------+ +------------+ |  |
|  +-------------------------------+  |
+-------------------------------------+
```

- **CSS** — Real `bootstrap.min.css` and `bootstrap-icons.min.css`, bundled via `asset!()`
- **Icons** — Font files inlined as base64 data URIs (works with Dioxus asset hashing)
- **Signals** — Replace all Bootstrap JS behaviors; no `<script>` tags needed
- **Components** — Type-safe RSX wrappers that emit standard Bootstrap HTML classes

## Examples

See the [`examples/`](https://github.com/mik-tf/dioxus-bootstrap/tree/development/examples) directory:

- **[showcase](https://mik-tf.github.io/dioxus-bootstrap/)** — Every component demonstrated in a tabbed interface ([live demo](https://mik-tf.github.io/dioxus-bootstrap/))
- **dashboard** — Realistic admin dashboard with navbar, tables, modals, charts

## Migration from Bootstrap HTML

See [docs/MIGRATION.md](https://github.com/mik-tf/dioxus-bootstrap/blob/development/docs/MIGRATION.md) for a complete HTML-to-RSX migration guide.

## License

Apache 2.0 — Project Mycelium 2026
