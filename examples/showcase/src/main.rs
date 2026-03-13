use dioxus::prelude::*;
use dioxus_bootstrap::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let active_tab = use_signal(|| 0usize);

    rsx! {
        BootstrapHead {}
        NavbarDemo {}
        Container { class: "py-4",
            h1 { class: "mb-4", "dioxus-bootstrap Showcase" }
            p { class: "lead mb-4",
                "All Bootstrap 5.3 components rendered by Dioxus — zero JavaScript."
            }

            TabList {
                active: active_tab,
                tabs: vec![
                    TabDef {
                        label: "Basics".into(),
                        icon: Some("grid".into()),
                        content: rsx! { BasicsSection {} },
                    },
                    TabDef {
                        label: "Forms".into(),
                        icon: Some("input-cursor-text".into()),
                        content: rsx! { FormsSection {} },
                    },
                    TabDef {
                        label: "Data".into(),
                        icon: Some("table".into()),
                        content: rsx! { DataSection {} },
                    },
                    TabDef {
                        label: "Interactive".into(),
                        icon: Some("lightning".into()),
                        content: rsx! { InteractiveSection {} },
                    },
                ],
            }
        }
    }
}

#[component]
fn NavbarDemo() -> Element {
    let collapsed = use_signal(|| true);

    rsx! {
        Navbar {
            color: Color::Dark,
            expand: NavbarExpand::Lg,
            brand: rsx! { a { class: "navbar-brand", href: "#", Icon { name: "bootstrap", class: "me-2" } "dioxus-bootstrap" } },
            NavbarToggler { collapsed: collapsed }
            NavbarCollapse { collapsed: collapsed,
                NavItem { NavLink { href: "#", active: true, "Showcase" } }
                NavItem { NavLink { href: "#", "Docs" } }
                NavItem { NavLink { href: "#", "GitHub" } }
            }
        }
    }
}

#[component]
fn BasicsSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Buttons
            h3 { class: "mb-3", "Buttons" }
            div { class: "d-flex flex-wrap gap-2 mb-4",
                Button { color: Color::Primary, "Primary" }
                Button { color: Color::Secondary, "Secondary" }
                Button { color: Color::Success, "Success" }
                Button { color: Color::Danger, "Danger" }
                Button { color: Color::Warning, "Warning" }
                Button { color: Color::Info, "Info" }
                Button { color: Color::Light, "Light" }
                Button { color: Color::Dark, "Dark" }
            }
            div { class: "d-flex flex-wrap gap-2 mb-4",
                Button { color: Color::Primary, outline: true, "Outline" }
                Button { color: Color::Primary, size: Size::Sm, "Small" }
                Button { color: Color::Primary, size: Size::Lg, "Large" }
                Button { color: Color::Primary, disabled: true, "Disabled" }
            }
            ButtonGroup { class: "mb-4",
                Button { color: Color::Primary, "Left" }
                Button { color: Color::Primary, "Middle" }
                Button { color: Color::Primary, "Right" }
            }

            // Grid
            h3 { class: "mb-3", "Grid" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-primary-subtle border rounded", "col-md-4" }
                }
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-primary-subtle border rounded", "col-md-4" }
                }
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-primary-subtle border rounded", "col-md-4" }
                }
            }
            Row { class: "g-3 mb-4",
                Col { lg: ColumnSize::Span(3),
                    div { class: "p-3 bg-success-subtle border rounded", "col-lg-3" }
                }
                Col { lg: ColumnSize::Span(9),
                    div { class: "p-3 bg-success-subtle border rounded", "col-lg-9" }
                }
            }

            // Cards
            h3 { class: "mb-3", "Cards" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4),
                    Card {
                        header: rsx! { "Card Header" },
                        body: rsx! {
                            h5 { class: "card-title", "Card Title" }
                            p { class: "card-text", "Some quick example text." }
                            Button { color: Color::Primary, size: Size::Sm, "Go somewhere" }
                        },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            h5 { class: "card-title", "Simple Card" }
                            p { class: "card-text", "A card with just a body." }
                        },
                        footer: rsx! { small { class: "text-muted", "2 days ago" } },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card { class: "text-bg-primary",
                        body: rsx! {
                            h5 { class: "card-title", "Colored Card" }
                            p { class: "card-text", "A primary-colored card." }
                        },
                    }
                }
            }

            // Alerts
            h3 { class: "mb-3", "Alerts" }
            Alert { color: Color::Primary, "A simple primary alert." }
            Alert { color: Color::Success, "A simple success alert." }
            Alert { color: Color::Danger, dismissible: true, "A dismissible danger alert." }
            Alert { color: Color::Warning, dismissible: true, "A dismissible warning alert." }

            // Badges
            h3 { class: "mb-3 mt-4", "Badges" }
            div { class: "d-flex flex-wrap gap-2 mb-4",
                Badge { color: Color::Primary, "Primary" }
                Badge { color: Color::Secondary, "Secondary" }
                Badge { color: Color::Success, "Success" }
                Badge { color: Color::Danger, pill: true, "Danger Pill" }
                Badge { color: Color::Warning, pill: true, "Warning Pill" }
                Badge { color: Color::Info, "Info" }
            }

            // Icons
            h3 { class: "mb-3", "Icons" }
            div { class: "d-flex flex-wrap gap-3 fs-4 mb-4",
                Icon { name: "house" }
                Icon { name: "search" }
                Icon { name: "gear" }
                Icon { name: "person" }
                Icon { name: "shield-lock" }
                Icon { name: "lightning" }
                Icon { name: "book" }
                Icon { name: "terminal" }
                Icon { name: "speedometer2" }
                Icon { name: "diagram-3" }
            }

            // Breadcrumb
            h3 { class: "mb-3", "Breadcrumb" }
            Breadcrumb {
                BreadcrumbItem { href: "#", "Home" }
                BreadcrumbItem { href: "#", "Products" }
                BreadcrumbItem { active: true, "Current Page" }
            }
        }
    }
}

#[component]
fn FormsSection() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut accept = use_signal(|| false);

    rsx! {
        div { class: "mt-3",
            Row { class: "g-3",
                Col { lg: ColumnSize::Span(6),
                    Card {
                        header: rsx! { "Login Form" },
                        body: rsx! {
                            FormGroup { label: "Email address",
                                Input {
                                    r#type: "email",
                                    placeholder: "you@example.com",
                                    value: "{email}",
                                    oninput: move |e: FormEvent| email.set(e.value()),
                                }
                            }
                            FormGroup { label: "Password",
                                Input {
                                    r#type: "password",
                                    placeholder: "Enter password",
                                    value: "{password}",
                                    oninput: move |e: FormEvent| password.set(e.value()),
                                }
                            }
                            Checkbox {
                                checked: *accept.read(),
                                label: "Remember me".to_string(),
                                onchange: move |_| {
                                    let current = *accept.read();
                                    accept.set(!current);
                                },
                            }
                            div { class: "mt-3",
                                Button { color: Color::Primary, r#type: "submit", "Sign In" }
                            }
                        },
                    }
                }
                Col { lg: ColumnSize::Span(6),
                    Card {
                        header: rsx! { "Other Controls" },
                        body: rsx! {
                            FormGroup { label: "Select",
                                Select {
                                    option { value: "", "Choose..." }
                                    option { value: "1", "Option 1" }
                                    option { value: "2", "Option 2" }
                                    option { value: "3", "Option 3" }
                                }
                            }
                            FormGroup { label: "Textarea",
                                Textarea {
                                    rows: 3,
                                    placeholder: "Write something...",
                                    value: "{message}",
                                    oninput: move |e: FormEvent| message.set(e.value()),
                                }
                            }
                            div { class: "mb-3",
                                label { class: "form-label d-block", "Radio Group" }
                                Radio { name: "color".to_string(), label: "Red".to_string(), checked: true }
                                Radio { name: "color".to_string(), label: "Green".to_string() }
                                Radio { name: "color".to_string(), label: "Blue".to_string() }
                            }
                            FormGroup { label: "Small Input",
                                Input { size: Size::Sm, placeholder: "Small" }
                            }
                            FormGroup { label: "Large Input",
                                Input { size: Size::Lg, placeholder: "Large" }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn DataSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Table
            h3 { class: "mb-3", "Table" }
            Card { class: "mb-4",
                body: rsx! {
                    Table { striped: true, hover: true, responsive: true,
                        thead {
                            tr {
                                th { "#" }
                                th { "Service" }
                                th { "Status" }
                                th { "Uptime" }
                            }
                        }
                        tbody {
                            tr {
                                td { "1" }
                                td { "API Gateway" }
                                td { Badge { color: Color::Success, "Running" } }
                                td { "99.9%" }
                            }
                            tr {
                                td { "2" }
                                td { "Database" }
                                td { Badge { color: Color::Success, "Running" } }
                                td { "99.7%" }
                            }
                            tr {
                                td { "3" }
                                td { "Cache" }
                                td { Badge { color: Color::Warning, "Degraded" } }
                                td { "98.2%" }
                            }
                            tr {
                                td { "4" }
                                td { "Worker" }
                                td { Badge { color: Color::Danger, "Down" } }
                                td { "—" }
                            }
                        }
                    }
                },
            }

            // List Group
            h3 { class: "mb-3", "List Group" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    ListGroup {
                        ListGroupItem { active: true, "Active item" }
                        ListGroupItem { "Second item" }
                        ListGroupItem { "Third item" }
                        ListGroupItem { disabled: true, "Disabled item" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    ListGroup { flush: true,
                        ListGroupItem { color: Color::Primary, "Primary" }
                        ListGroupItem { color: Color::Success, "Success" }
                        ListGroupItem { color: Color::Danger, "Danger" }
                        ListGroupItem { color: Color::Warning, "Warning" }
                    }
                }
            }

            // Progress & Spinners
            h3 { class: "mb-3", "Progress Bars" }
            div { class: "mb-4",
                Progress { class: "mb-2",
                    ProgressBar { value: 25.0, show_label: true }
                }
                Progress { class: "mb-2",
                    ProgressBar { value: 50.0, color: Color::Success, show_label: true }
                }
                Progress { class: "mb-2",
                    ProgressBar { value: 75.0, color: Color::Warning, striped: true }
                }
                Progress { class: "mb-2",
                    ProgressBar { value: 100.0, color: Color::Danger, striped: true, animated: true }
                }
                // Stacked
                Progress { class: "mb-2",
                    ProgressBar { value: 30.0, color: Color::Primary }
                    ProgressBar { value: 20.0, color: Color::Success }
                    ProgressBar { value: 15.0, color: Color::Warning }
                }
            }

            h3 { class: "mb-3", "Spinners" }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Spinner { "Loading..." }
                Spinner { color: Color::Primary, "Loading..." }
                Spinner { color: Color::Success, "Loading..." }
                Spinner { style: SpinnerStyle::Grow, color: Color::Danger, "Loading..." }
                Spinner { size: Size::Sm, color: Color::Info, "Loading..." }
            }
        }
    }
}

#[component]
fn InteractiveSection() -> Element {
    let mut show_modal = use_signal(|| false);
    let dropdown_open = use_signal(|| false);
    let mut collapse_expanded = use_signal(|| false);

    rsx! {
        div { class: "mt-3",
            // Modal
            h3 { class: "mb-3", "Modal" }
            div { class: "mb-4",
                Button { color: Color::Primary, onclick: move |_| show_modal.set(true),
                    Icon { name: "box-arrow-up-right", class: "me-1" }
                    "Open Modal"
                }
                Modal {
                    show: show_modal,
                    title: "Example Modal".to_string(),
                    centered: true,
                    body: rsx! {
                        p { "This modal is controlled entirely by a Dioxus signal." }
                        p { "No JavaScript involved — click the backdrop or close button to dismiss." }
                    },
                    footer: rsx! {
                        Button { color: Color::Secondary, onclick: move |_| show_modal.set(false), "Close" }
                        Button { color: Color::Primary, "Save changes" }
                    },
                }
            }

            // Dropdown
            h3 { class: "mb-3", "Dropdown" }
            div { class: "mb-4",
                Dropdown {
                    open: dropdown_open,
                    toggle: rsx! { "Dropdown Menu" },
                    menu: rsx! {
                        DropdownHeader { "Actions" }
                        DropdownItem { Icon { name: "pencil", class: "me-2" } "Edit" }
                        DropdownItem { Icon { name: "files", class: "me-2" } "Duplicate" }
                        DropdownDivider {}
                        DropdownItem { Icon { name: "trash", class: "me-2" } "Delete" }
                    },
                }
            }

            // Collapse
            h3 { class: "mb-3", "Collapse" }
            div { class: "mb-4",
                Button {
                    color: Color::Primary,
                    onclick: move |_| {
                        let current = *collapse_expanded.read();
                        collapse_expanded.set(!current);
                    },
                    if *collapse_expanded.read() { "Hide Content" } else { "Show Content" }
                }
                Collapse { expanded: collapse_expanded, class: "mt-2",
                    Card {
                        body: rsx! {
                            "This content is shown/hidden using a Dioxus signal driving the Bootstrap collapse classes. No JavaScript transitions needed."
                        },
                    }
                }
            }
        }
    }
}
