# Roadmap

Tracking remaining Bootstrap 5.3 features not yet implemented in dioxus-bootstrap-css.

## Completed

All Bootstrap 5.3 components and features are implemented except the items listed below.

### Components
- Accordion, Alert, Badge, Breadcrumb, Button, ButtonGroup, ButtonToolbar
- Card, Carousel, Collapse
- Dropdown (including split button, directions, alignment)
- Figure, Ratio (responsive embed)
- Form: Input, Select, Textarea, Checkbox, Radio, Switch, Range, FloatingLabel, FormFeedback, FormText, FormGroup, InputGroup
- Grid: Container, Row, Col (with offset and order)
- Icon, ListGroup, Modal (sizes, fullscreen, centered, scrollable)
- Nav (pills, tabs, underline, fill, justified, vertical), Navbar, NavbarToggler, NavbarCollapse
- Offcanvas (all placements, responsive variants)
- Pagination, Placeholder, Progress/ProgressBar
- Spinner, Table (striped, striped-columns, caption, responsive)
- Tabs (pills, fill, justified, vertical), Toast/ToastContainer
- Theme: ThemeProvider, ThemeToggle

## Not Yet Implemented

### Tooltips
Bootstrap uses Popper.js for tooltip positioning. Implementing this requires either:
- A pure Rust/WASM positioning engine
- Using `document::eval` with lightweight JS positioning logic

**Status:** Deferred — requires positioning strategy decision.

### Popovers
Same positioning challenge as Tooltips. Popovers are essentially tooltips with richer content (title + body).

**Status:** Deferred — blocked on same positioning solution as Tooltips.

### Scrollspy
Tracks scroll position and highlights the corresponding nav link. Requires:
- Scroll event listener on a container
- Intersection Observer or manual offset calculation
- Signal-driven nav link active state updates

**Status:** Deferred — medium complexity, requires scroll event integration.

## Design Decisions

See [DESIGN.md](DESIGN.md) for the 1-to-1 Bootstrap parity rule that governs all implementation decisions.
