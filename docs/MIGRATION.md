# Migration Guide: Bootstrap HTML to dioxus-bootstrap

This guide shows how to convert existing Bootstrap 5.3 HTML templates (Tera, Askama,
Jinja2, or plain HTML) to Dioxus RSX using `dioxus-bootstrap` components.

## Setup

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-bootstrap-css = "0.2.5"
```

Import the prelude in your Dioxus app:

```rust
use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;
```

Add `BootstrapHead` at the top of your app to load the bundled CSS:

```rust
fn app() -> Element {
    rsx! {
        BootstrapHead {}
        // your content
    }
}
```

## HTML to RSX Syntax Conversion

### Basic Elements

```html
<!-- HTML -->
<div class="container">
    <h1>Title</h1>
    <p class="text-muted">Description</p>
</div>
```

```rust
// RSX
rsx! {
    div { class: "container",
        h1 { "Title" }
        p { class: "text-muted", "Description" }
    }
}
```

### Self-Closing Tags

```html
<input type="text" class="form-control" placeholder="Name">
<br>
<hr>
```

```rust
rsx! {
    input { class: "form-control", r#type: "text", placeholder: "Name" }
    br {}
    hr {}
}
```

### Attributes

| HTML | RSX |
|------|-----|
| `class="..."` | `class: "..."` |
| `id="..."` | `id: "..."` |
| `type="..."` | `r#type: "..."` (type is a Rust keyword) |
| `for="..."` | `r#for: "..."` |
| `href="..."` | `href: "..."` |
| `data-bs-*="..."` | Not needed — use signals instead |
| `aria-*="..."` | `"aria-label": "..."` |

## Component-by-Component Migration

### Buttons

```html
<button type="button" class="btn btn-primary">Click me</button>
<button type="button" class="btn btn-outline-danger btn-sm">Delete</button>
```

```rust
rsx! {
    Button { color: Color::Primary, "Click me" }
    Button { color: Color::Danger, outline: true, size: Size::Sm, "Delete" }
}
```

### Cards

```html
<div class="card mb-3">
    <div class="card-header">Title</div>
    <div class="card-body">
        <p>Content</p>
    </div>
    <div class="card-footer">Footer</div>
</div>
```

```rust
rsx! {
    Card { class: "mb-3",
        header: rsx! { "Title" },
        body: rsx! { p { "Content" } },
        footer: rsx! { "Footer" },
    }
}
```

### Grid Layout

```html
<div class="container">
    <div class="row g-3">
        <div class="col-lg-3">Sidebar</div>
        <div class="col-lg-9">Main</div>
    </div>
</div>
```

```rust
rsx! {
    Container {
        Row { class: "g-3",
            Col { lg: ColumnSize::Span(3), "Sidebar" }
            Col { lg: ColumnSize::Span(9), "Main" }
        }
    }
}
```

### Alerts

```html
<div class="alert alert-success alert-dismissible fade show" role="alert">
    Saved successfully!
    <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
</div>
```

```rust
rsx! {
    Alert { color: Color::Success, dismissible: true,
        "Saved successfully!"
    }
}
```

### Tables

```html
<div class="table-responsive">
    <table class="table table-striped table-hover table-sm">
        <thead>
            <tr><th>Name</th><th>Status</th></tr>
        </thead>
        <tbody>
            <tr><td>Service A</td><td>Running</td></tr>
        </tbody>
    </table>
</div>
```

```rust
rsx! {
    Table { striped: true, hover: true, size: Size::Sm, responsive: true,
        thead {
            tr { th { "Name" } th { "Status" } }
        }
        tbody {
            tr { td { "Service A" } td { "Running" } }
        }
    }
}
```

### Forms

```html
<div class="mb-3">
    <label class="form-label">Email</label>
    <input type="email" class="form-control" placeholder="you@example.com">
</div>
<div class="mb-3">
    <label class="form-label">Message</label>
    <textarea class="form-control" rows="3"></textarea>
</div>
<div class="form-check">
    <input class="form-check-input" type="checkbox" checked>
    <label class="form-check-label">Accept terms</label>
</div>
```

```rust
rsx! {
    FormGroup { label: "Email",
        Input { r#type: "email", placeholder: "you@example.com" }
    }
    FormGroup { label: "Message",
        Textarea { rows: 3 }
    }
    Checkbox { checked: true, label: "Accept terms" }
}
```

### Tabs (JS → Signal)

```html
<!-- HTML + Bootstrap JS -->
<ul class="nav nav-tabs" role="tablist">
    <li class="nav-item">
        <button class="nav-link active" data-bs-toggle="tab" data-bs-target="#home">Home</button>
    </li>
    <li class="nav-item">
        <button class="nav-link" data-bs-toggle="tab" data-bs-target="#profile">Profile</button>
    </li>
</ul>
<div class="tab-content">
    <div class="tab-pane fade show active" id="home">Home content</div>
    <div class="tab-pane fade" id="profile">Profile content</div>
</div>
```

```rust
// RSX — signal replaces data-bs-toggle
let active_tab = use_signal(|| 0usize);
rsx! {
    TabList {
        active: active_tab,
        tabs: vec![
            TabDef { label: "Home".into(), icon: None, content: rsx! { "Home content" } },
            TabDef { label: "Profile".into(), icon: None, content: rsx! { "Profile content" } },
        ],
    }
}
```

### Modal (JS → Signal)

```html
<!-- HTML + Bootstrap JS -->
<button data-bs-toggle="modal" data-bs-target="#myModal">Open</button>
<div class="modal fade" id="myModal">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h5 class="modal-title">Confirm</h5>
                <button class="btn-close" data-bs-dismiss="modal"></button>
            </div>
            <div class="modal-body">Are you sure?</div>
            <div class="modal-footer">
                <button class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
                <button class="btn btn-primary">Confirm</button>
            </div>
        </div>
    </div>
</div>
```

```rust
// RSX — signal replaces data-bs-toggle/dismiss
let show = use_signal(|| false);
rsx! {
    Button { onclick: move |_| show.set(true), "Open" }
    Modal {
        show: show,
        title: "Confirm",
        body: rsx! { "Are you sure?" },
        footer: rsx! {
            Button { color: Color::Secondary, onclick: move |_| show.set(false), "Cancel" }
            Button { color: Color::Primary, "Confirm" }
        },
    }
}
```

### Dropdown (JS → Signal)

```html
<div class="dropdown">
    <button class="btn btn-secondary dropdown-toggle" data-bs-toggle="dropdown">Actions</button>
    <ul class="dropdown-menu">
        <li><button class="dropdown-item">Edit</button></li>
        <li><button class="dropdown-item">Delete</button></li>
    </ul>
</div>
```

```rust
let open = use_signal(|| false);
rsx! {
    Dropdown { open: open,
        toggle: rsx! { "Actions" },
        menu: rsx! {
            DropdownItem { "Edit" }
            DropdownItem { "Delete" }
        },
    }
}
```

### Navbar

```html
<nav class="navbar navbar-expand-lg bg-dark" data-bs-theme="dark">
    <div class="container-fluid">
        <a class="navbar-brand" href="#">MyApp</a>
        <button class="navbar-toggler" data-bs-toggle="collapse" data-bs-target="#navContent">
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navContent">
            <ul class="navbar-nav me-auto">
                <li class="nav-item">
                    <a class="nav-link active" href="/">Home</a>
                </li>
            </ul>
        </div>
    </div>
</nav>
```

```rust
let collapsed = use_signal(|| true);
rsx! {
    Navbar { color: Color::Dark, expand: NavbarExpand::Lg,
        brand: rsx! { a { class: "navbar-brand", href: "#", "MyApp" } },
        NavbarToggler { collapsed: collapsed }
        NavbarCollapse { collapsed: collapsed,
            NavItem { NavLink { href: "/", active: true, "Home" } }
        }
    }
}
```

### Icons

```html
<i class="bi bi-search"></i>
<i class="bi bi-shield-lock me-2 fs-4"></i>
```

```rust
rsx! {
    Icon { name: "search" }
    Icon { name: "shield-lock", class: "me-2 fs-4" }
}
```

## Template Logic Conversion

### Tera/Jinja2 Conditionals

```html
{% if user.is_admin %}
    <span class="badge text-bg-danger">Admin</span>
{% endif %}
```

```rust
if user.is_admin {
    rsx! { Badge { color: Color::Danger, "Admin" } }
}
```

### Tera/Jinja2 Loops

```html
{% for item in items %}
    <li class="list-group-item">{{ item.name }}</li>
{% endfor %}
```

```rust
for item in items.iter() {
    rsx! { ListGroupItem { "{item.name}" } }
}
```

### Tera/Jinja2 Variables

```html
<h1>{{ page.title }}</h1>
<p>{{ page.description }}</p>
```

```rust
rsx! {
    h1 { "{page.title}" }
    p { "{page.description}" }
}
```

## Common Patterns

### Two-Column Dashboard Layout

```rust
rsx! {
    BootstrapHead {}
    Container { fluid: true, class: "py-3",
        Row { class: "g-3",
            // Sidebar
            Col { lg: ColumnSize::Span(3),
                Card {
                    header: rsx! { "Stats" },
                    body: rsx! {
                        // stats content
                    },
                }
            }
            // Main content
            Col { lg: ColumnSize::Span(9),
                // Tabbed interface
                TabList {
                    active: active_tab,
                    tabs: vec![
                        TabDef { label: "Overview".into(), icon: Some("speedometer2".into()), content: rsx! { /* ... */ } },
                        TabDef { label: "Settings".into(), icon: Some("gear".into()), content: rsx! { /* ... */ } },
                    ],
                }
            }
        }
    }
}
```

### Card Grid

```rust
rsx! {
    Row { class: "g-3",
        for item in items.iter() {
            Col { md: ColumnSize::Span(6), lg: ColumnSize::Span(4),
                Card {
                    body: rsx! {
                        h5 { class: "card-title", "{item.name}" }
                        p { class: "card-text", "{item.description}" }
                        Button { color: Color::Primary, size: Size::Sm, "View" }
                    },
                }
            }
        }
    }
}
```

### Form with Validation Feedback

```rust
rsx! {
    form {
        onsubmit: handle_submit,
        FormGroup { label: "Email",
            Input { r#type: "email", value: "{email}", oninput: update_email }
        }
        FormGroup { label: "Password",
            Input { r#type: "password", value: "{password}", oninput: update_password }
        }
        if !error.is_empty() {
            Alert { color: Color::Danger, "{error}" }
        }
        Button { color: Color::Primary, r#type: "submit", "Sign In" }
    }
}
```

## CSS Custom Properties Bridge

If your app uses a custom design system with CSS variables, you can bridge them
to Bootstrap's `--bs-*` variables. Create a bridge CSS file:

```css
:root {
    /* Map your app's variables to Bootstrap's */
    --bs-primary: var(--your-app-primary);
    --bs-body-bg: var(--your-app-bg);
    --bs-body-color: var(--your-app-text);
}
```

For dark/light mode, set the `data-bs-theme` attribute on the root element:

```rust
// In your app's theme toggle logic
let document = web_sys::window().unwrap().document().unwrap();
let root = document.document_element().unwrap();
root.set_attribute("data-bs-theme", "dark").unwrap();
```

## Utility Classes Over Custom CSS

Prefer Bootstrap utility classes over custom CSS. The library exposes the full Bootstrap 5.3 CSS, so classes like `py-2`, `small`, `mb-0`, `btn-sm`, `d-flex`, `text-muted`, and `gap-2` all work directly via the `class` prop.

**Instead of custom CSS:**
```css
/* Don't do this */
.my-compact-card-header { padding: 0.25rem 0.5rem; font-size: 0.875rem; }
```

**Use Bootstrap utilities:**
```rust
Card {
    header_class: "py-1 small",
    body_class: "py-2",
    header: rsx! { span { class: "small", "Title" } },
    body: rsx! { /* ... */ },
}
```

Use component props (`header_class`, `body_class`, `responsive: true`, `fluid: true`) instead of adding wrapper divs.

## Gold Standard Migration Pattern

This pattern demonstrates the recommended way to structure a dashboard-style layout:

```rust
Navbar { expand: NavbarExpand::Lg, class: "bg-body-tertiary border-bottom",
    brand: rsx! { a { class: "navbar-brand", href: "#", "MyApp" } },
}
Container { fluid: true, class: "py-4",
    Row { class: "g-3",
        Col { lg: ColumnSize::Span(3),
            Card { class: "mb-3", header_class: "py-2", body_class: "py-2",
                header: rsx! { span { class: "small", "Server Status" } },
                body: rsx! {
                    Table { size: Size::Sm, class: "mb-0 small",
                        tbody {
                            tr { td { "API" } td { Badge { color: Color::Success, "Up" } } }
                            tr { td { "DB" } td { Badge { color: Color::Success, "Up" } } }
                        }
                    }
                },
            }
        }
        Col { lg: ColumnSize::Span(9),
            TabList { active: active_tab, tabs: vec![
                TabDef { label: "Overview".into(), icon: Some("speedometer2".into()),
                    content: rsx! { /* ... */ } },
                TabDef { label: "Settings".into(), icon: Some("gear".into()),
                    content: rsx! { /* ... */ } },
            ] }
        }
    }
}
```

Key principles in this pattern:
- Bootstrap utility classes (`py-2`, `small`, `mb-0`, `g-3`) replace custom CSS
- Component props (`header_class`, `body_class`, `fluid: true`) replace wrapper divs
- Signals replace all JavaScript behaviors

## Key Differences from Bootstrap HTML

| Bootstrap HTML | dioxus-bootstrap |
|---------------|-----------------|
| `data-bs-toggle="modal"` | `Signal<bool>` controls visibility |
| `data-bs-toggle="tab"` | `Signal<usize>` controls active tab |
| `data-bs-toggle="dropdown"` | `Signal<bool>` controls open state |
| `data-bs-toggle="collapse"` | `Signal<bool>` controls expanded state |
| `data-bs-dismiss="alert"` | Built into `Alert { dismissible: true }` |
| `bootstrap.bundle.min.js` | Not needed — zero JavaScript |
| CDN `<link>` for CSS | `BootstrapHead {}` loads bundled CSS |
| `{% include "partial.html" %}` | Extract to a Dioxus `#[component]` |
| `{% block content %}` | Component composition with `Element` props |
