# Design Principles

## 1-to-1 Bootstrap Parity

`dioxus-bootstrap-css` is a strict 1-to-1 mapping of Bootstrap 5.3 for Dioxus.

- **CSS** — Real Bootstrap 5.3.3 CSS, bundled and served via `asset!()`. Not a reimplementation, not a subset, not a superset.
- **JS → Signals** — Bootstrap's JavaScript behaviors (dropdowns, modals, tabs, collapse, etc.) are replaced with Dioxus signals. Same behavior, same classes, no JS dependency.
- **Icons** — Real Bootstrap Icons CSS with inlined fonts.

### What belongs in this library

- Type-safe RSX wrappers for Bootstrap components
- Signal-driven replacements for Bootstrap JS behaviors
- Bundled Bootstrap CSS and Bootstrap Icons

### What does NOT belong in this library

- Custom CSS beyond what Bootstrap provides
- App-specific styling (gradients, scroll offsets, page layouts)
- Opinionated defaults that Bootstrap doesn't have
- Extra components that Bootstrap doesn't define

### The rule

> If Bootstrap 5.3 does it, we do it. If Bootstrap doesn't, we don't.

App-specific styling, custom themes, and layout decisions belong in the consuming application's own CSS, not in this library.

## Architecture

```
┌─────────────────────────────────────┐
│          Your Dioxus App            │
│  ┌───────────────────────────────┐  │
│  │   app-specific CSS / logic    │  │
│  └───────────────────────────────┘  │
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

## Component Design

Each component follows the same pattern:

1. **Props** mirror Bootstrap's HTML attributes and classes
2. **Signals** replace `data-bs-toggle` / `data-bs-dismiss` JS behaviors
3. **Output** is standard Bootstrap HTML with the right CSS classes
4. **No custom CSS** — if Bootstrap's classes handle it, that's all we emit
