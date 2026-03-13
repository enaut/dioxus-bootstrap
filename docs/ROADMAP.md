# Roadmap

## Status: Complete

All Bootstrap 5.3 components are implemented.

### Components

**Layout & Head:**
Container, Row, Col (offset, order), BootstrapHead, ThemeProvider, ThemeToggle

**Content:**
Button (active, outline, sizes), ButtonGroup, ButtonToolbar, Card, Alert,
Badge, Icon, Spinner, Progress/ProgressBar, Placeholder/PlaceholderParagraph,
Figure, Ratio

**Data Display:**
Table (striped, striped-columns, hover, bordered, responsive, caption),
ListGroup/ListGroupItem, Pagination

**Forms:**
FormGroup, Input, Select, Textarea, Checkbox, Radio, Switch, Range,
FloatingLabel, InputGroup/InputGroupText, FormFeedback, FormText

**Interactive (Signal-Driven, Zero JS):**
Modal (sizes, fullscreen, centered, scrollable), Dropdown (split, directions),
Collapse, Tabs/Tab/TabList (pills, fill, justified, vertical),
Accordion/AccordionItem, Offcanvas (placements, responsive), Toast/ToastContainer,
Carousel (indicators, controls, fade, dark), Tooltip, Popover, Scrollspy

**Navigation:**
Navbar, NavbarToggler, NavbarCollapse, Nav (pills, tabs, underline, fill,
justified, vertical), NavItem, NavLink, Breadcrumb/BreadcrumbItem

### Notes

- **Tooltip** uses CSS-based positioning relative to the trigger element.
  For most use cases this works well. Edge cases near viewport boundaries
  may require the app to adjust placement manually.

- **Popover** follows the same CSS positioning approach as Tooltip, with
  click-to-toggle and click-outside-to-close behavior.

- **Scrollspy** uses `document::eval` with a scroll event listener to track
  visible sections and update a signal with the active section id.

### Design Principles

See [DESIGN.md](DESIGN.md) — strict 1-to-1 Bootstrap 5.3 parity.
