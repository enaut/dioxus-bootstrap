use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let active_tab = use_signal(|| 0usize);
    let theme = use_signal(|| Theme::Dark);

    rsx! {
        ThemeProvider { theme: theme }
        // Bundled assets are the defaults; passed explicitly so the props are exercised.
        BootstrapHead { css: BootstrapCss::Bundled, icons: BootstrapIcons::Bundled }
        NavbarDemo { theme: theme }
        Container { class: "py-4",
            h1 { class: "mb-4", "dioxus-bootstrap Showcase" }
            p { class: "lead mb-4",
                "All Bootstrap 5.3 components rendered by Dioxus — zero JavaScript."
            }

            TabList {
                active: active_tab,
                tabs: vec![
                    TabDef {
                        label: "New in 0.6.0".into(),
                        icon: Some("stars".into()),
                        content: rsx! { WhatsNewSection {} },
                    },
                    TabDef {
                        label: "Coverage".into(),
                        icon: Some("check2-all".into()),
                        content: rsx! { CoverageSection {} },
                    },
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
                    TabDef {
                        label: "Overlays".into(),
                        icon: Some("chat-square".into()),
                        content: rsx! { OverlaysSection {} },
                    },
                    TabDef {
                        label: "Media".into(),
                        icon: Some("image".into()),
                        content: rsx! { MediaSection {} },
                    },
                    TabDef {
                        label: "Navigation".into(),
                        icon: Some("signpost".into()),
                        content: rsx! { NavigationSection {} },
                    },
                    TabDef {
                        label: "More".into(),
                        icon: Some("plus-circle".into()),
                        content: rsx! { MoreSection {} },
                    },
                ],
            }

            // Docs section — scroll target for navbar link
            div { id: "docs-section", class: "mt-5 pt-4 border-top",
                h2 { class: "mb-3",
                    Icon { name: "book", class: "me-2" }
                    "Component Reference"
                }
                p { class: "lead", "All Bootstrap 5.3 components available in dioxus-bootstrap-css." }

                Row { class: "g-4",
                    Col { md: ColumnSize::Span(4),
                        h5 { "Layout" }
                        ul {
                            li { "Container / Container-fluid" }
                            li { "Row / Col (offset, order)" }
                            li { "BootstrapHead" }
                            li { "ThemeProvider / ThemeToggle" }
                        }
                        h5 { class: "mt-3", "Content" }
                        ul {
                            li { "Button / ButtonGroup / ButtonToolbar" }
                            li { "Card (header, body, footer)" }
                            li { "Alert (dismissible)" }
                            li { "Badge (pill)" }
                            li { "Icon (Bootstrap Icons)" }
                            li { "Spinner (border, grow)" }
                            li { "Progress / ProgressBar" }
                            li { "Placeholder / PlaceholderParagraph" }
                            li { "Figure / Ratio" }
                        }
                    }
                    Col { md: ColumnSize::Span(4),
                        h5 { "Forms" }
                        ul {
                            li { "Input / Select / Textarea" }
                            li { "Checkbox / Radio / Switch" }
                            li { "Range (slider)" }
                            li { "FloatingLabel" }
                            li { "FormGroup / FormFeedback / FormText" }
                            li { "InputGroup / InputGroupText" }
                        }
                        h5 { class: "mt-3", "Data Display" }
                        ul {
                            li { "Table (striped, hover, caption)" }
                            li { "ListGroup / ListGroupItem" }
                            li { "Pagination" }
                        }
                    }
                    Col { md: ColumnSize::Span(4),
                        h5 { "Interactive (Signal-Driven)" }
                        ul {
                            li { "Modal (sizes, fullscreen)" }
                            li { "Dropdown (split, directions)" }
                            li { "Collapse" }
                            li { "Tabs / Tab / TabList" }
                            li { "Accordion / AccordionItem" }
                            li { "Offcanvas (placements)" }
                            li { "Toast / ToastContainer" }
                            li { "Carousel" }
                            li { "Tooltip" }
                            li { "Popover" }
                            li { "Scrollspy" }
                        }
                        h5 { class: "mt-3", "Navigation" }
                        ul {
                            li { "Navbar / NavbarToggler / NavbarCollapse / NavbarNav" }
                            li { "Nav (pills, tabs, underline, fill)" }
                            li { "NavItem / NavLink / NavButton" }
                            li { "Breadcrumb / BreadcrumbItem" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct NavbarDemoProps {
    theme: Signal<Theme>,
}

#[component]
fn NavbarDemo(props: NavbarDemoProps) -> Element {
    let collapsed = use_signal(|| true);

    rsx! {
        Navbar {
            expand: NavbarExpand::Lg,
            class: "sticky-top border-bottom bg-body",
            brand: rsx! { a { class: "navbar-brand", href: "#", Icon { name: "bootstrap", class: "me-2" } "dioxus-bootstrap-css" } },
            NavbarToggler { collapsed: collapsed }
            NavbarCollapse { collapsed: collapsed,
                NavbarNav {
                    NavItem { NavLink { href: "#", active: true, "Showcase" } }
                    NavItem {
                        NavLink { href: "#docs-section", "Docs" }
                    }
                }
            }
            ThemeToggle { theme: props.theme }
        }
    }
}

// ── Basics ──────────────────────────────────────────────────────────────────

#[component]
fn BasicsSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Buttons
            h3 { class: "mb-3", "Buttons" }
            div { class: "d-flex flex-wrap gap-2 mb-3",
                Button { color: Color::Primary, "Primary" }
                Button { color: Color::Secondary, "Secondary" }
                Button { color: Color::Success, "Success" }
                Button { color: Color::Danger, "Danger" }
                Button { color: Color::Warning, "Warning" }
                Button { color: Color::Info, "Info" }
                Button { color: Color::Light, "Light" }
                Button { color: Color::Dark, "Dark" }
            }
            div { class: "d-flex flex-wrap gap-2 mb-3",
                Button { color: Color::Primary, outline: true, "Outline" }
                Button { color: Color::Primary, size: Size::Sm, "Small" }
                Button { color: Color::Primary, size: Size::Lg, "Large" }
                Button { color: Color::Primary, disabled: true, "Disabled" }
                Button { color: Color::Success, active: true, "Active" }
                Button { color: Color::Primary, href: "https://getbootstrap.com/", target: "_blank",
                    Icon { name: "box-arrow-up-right", class: "me-1" }
                    "Link Button"
                }
                Button { size: Size::Sm, href: "/example.json", download: "example.json",
                    Icon { name: "download", class: "me-1" }
                    "Download"
                }
            }

            // Button Group & Toolbar
            h4 { class: "mb-2 mt-3", "Button Group & Toolbar" }
            ButtonGroup { class: "mb-3",
                Button { color: Color::Primary, "Left" }
                Button { color: Color::Primary, "Middle" }
                Button { color: Color::Primary, "Right" }
            }
            ButtonToolbar { class: "mb-4 gap-2",
                ButtonGroup {
                    Button { color: Color::Primary, "1" }
                    Button { color: Color::Primary, "2" }
                    Button { color: Color::Primary, "3" }
                }
                ButtonGroup {
                    Button { color: Color::Secondary, "A" }
                    Button { color: Color::Secondary, "B" }
                }
            }

            // Grid
            h3 { class: "mb-3", "Grid" }
            Row { class: "g-3 mb-3",
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

            // Grid with offset and order
            h4 { class: "mb-2", "Grid Offset & Order" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4), offset_md: Some(2),
                    div { class: "p-3 bg-info-subtle border rounded", "col-md-4 offset-md-2" }
                }
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-info-subtle border rounded", "col-md-4" }
                }
            }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4), order: Some(3),
                    div { class: "p-3 bg-warning-subtle border rounded", "First in source, order-3" }
                }
                Col { md: ColumnSize::Span(4), order: Some(1),
                    div { class: "p-3 bg-warning-subtle border rounded", "Second in source, order-1" }
                }
                Col { md: ColumnSize::Span(4), order: Some(2),
                    div { class: "p-3 bg-warning-subtle border rounded", "Third in source, order-2" }
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
            Alert { color: Color::Info, dismissible: true,
                on_dismiss: move |_| { /* handle dismiss, e.g., clear state */ },
                Icon { name: "info-circle", class: "me-2" }
                "Dismissible with on_dismiss callback — check the console."
            }

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

// ── Forms ───────────────────────────────────────────────────────────────────

#[component]
fn FormsSection() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut accept = use_signal(|| false);
    let mut selected = use_signal(String::new);
    let mut switch_on = use_signal(|| true);
    // Off the 0–100 midpoint on purpose: a range with no `value` attribute
    // defaults to the midpoint, so an initial 20 witnesses controlled-value
    // reflection — the thumb must sit at 20, not the default 50.
    let mut range_val = use_signal(|| "20".to_string());

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
                                FormText { "Must be 8-20 characters long." }
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
                                    value: "{selected}",
                                    onchange: move |e: FormEvent| selected.set(e.value()),
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
                        },
                    }
                }
            }

            // Switch
            h3 { class: "mb-3 mt-4", "Switch" }
            Switch {
                checked: *switch_on.read(),
                label: "Enable notifications".to_string(),
                onchange: move |_| {
                    let current = *switch_on.read();
                    switch_on.set(!current);
                },
            }
            p { class: "text-muted", "Switch is: ", if *switch_on.read() { "ON" } else { "OFF" } }

            // Range
            h3 { class: "mb-3 mt-3", "Range" }
            FormGroup { label: "Volume",
                Range {
                    value: "{range_val}",
                    min: "0".to_string(),
                    max: "100".to_string(),
                    oninput: move |e: FormEvent| range_val.set(e.value()),
                }
            }
            p { class: "text-muted", "Value: {range_val}" }

            // Floating Labels
            h3 { class: "mb-3 mt-3", "Floating Labels" }
            Row { class: "g-3 mb-3",
                Col { md: ColumnSize::Span(6),
                    FloatingLabel { label: "Email address".to_string(),
                        Input { r#type: "email", placeholder: "name@example.com" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FloatingLabel { label: "Password".to_string(),
                        Input { r#type: "password", placeholder: "Password" }
                    }
                }
            }

            // Validation
            h3 { class: "mb-3 mt-3", "Validation Feedback" }
            Row { class: "g-3 mb-3",
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Valid input",
                        Input { value: "correct value", class: "is-valid".to_string() }
                        FormFeedback { valid: true, "Looks good!" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Invalid input",
                        Input { value: "", class: "is-invalid".to_string(), placeholder: "Required field" }
                        FormFeedback { "Please provide a value." }
                    }
                }
            }

            // Input sizes
            h3 { class: "mb-3 mt-3", "Input Sizes" }
            FormGroup { label: "Small Input",
                Input { size: Size::Sm, placeholder: "Small" }
            }
            FormGroup { label: "Default Input",
                Input { placeholder: "Default" }
            }
            FormGroup { label: "Large Input",
                Input { size: Size::Lg, placeholder: "Large" }
            }

            // Input Group
            h3 { class: "mb-3 mt-3", "Input Group" }
            InputGroup { class: "mb-2",
                InputGroupText { "@" }
                Input { placeholder: "Username".to_string() }
            }
            InputGroup { class: "mb-2",
                Input { placeholder: "Search...".to_string() }
                InputGroupText {
                    Button { color: Color::Primary,
                        Icon { name: "search" }
                    }
                }
            }
            InputGroup { class: "mb-2", size: Size::Sm,
                InputGroupText { "$" }
                Input { r#type: "number".to_string(), placeholder: "Amount".to_string() }
                InputGroupText { ".00" }
            }
        }
    }
}

// ── Data ────────────────────────────────────────────────────────────────────

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

            // Table with caption and striped columns
            h4 { class: "mb-3", "Table with Caption & Striped Columns" }
            Card { class: "mb-4",
                body: rsx! {
                    Table { striped_columns: true, bordered: true, caption: "List of users".to_string(), caption_top: true,
                        thead {
                            tr { th { "Name" } th { "Role" } th { "Status" } }
                        }
                        tbody {
                            tr { td { "Alice" } td { "Admin" } td { "Active" } }
                            tr { td { "Bob" } td { "Editor" } td { "Active" } }
                            tr { td { "Carol" } td { "Viewer" } td { "Inactive" } }
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

            // Pagination
            h3 { class: "mb-3", "Pagination" }
            div { class: "mb-4",
                Pagination { current: use_signal(|| 3usize), total: 20 }
                Pagination { current: use_signal(|| 3usize), total: 20, size: Size::Sm }
            }
        }
    }
}

// ── Interactive ─────────────────────────────────────────────────────────────

#[component]
fn InteractiveSection() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut show_modal_fs = use_signal(|| false);
    let dropdown_open = use_signal(|| false);
    let split_dropdown_open = use_signal(|| false);
    let end_dropdown_open = use_signal(|| false);
    let mut collapse_expanded = use_signal(|| false);
    let accordion_open = use_signal(|| Some(0usize));
    let mut toast_show = use_signal(|| false);
    let mut toast_headerless_show = use_signal(|| false);
    let mut offcanvas_show = use_signal(|| false);

    rsx! {
        div { class: "mt-3",
            // Modal
            h3 { class: "mb-3", "Modal" }
            div { class: "d-flex flex-wrap gap-2 mb-4",
                Button { color: Color::Primary, onclick: move |_| show_modal.set(true),
                    Icon { name: "box-arrow-up-right", class: "me-1" }
                    "Open Modal"
                }
                Button { color: Color::Secondary, onclick: move |_| show_modal_fs.set(true),
                    Icon { name: "arrows-fullscreen", class: "me-1" }
                    "Fullscreen Modal"
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
                Modal {
                    show: show_modal_fs,
                    title: "Fullscreen Modal".to_string(),
                    fullscreen: ModalFullscreen::Always,
                    body: rsx! {
                        p { "This modal takes up the entire screen." }
                        p { "Useful for complex forms or content that needs more space." }
                    },
                    footer: rsx! {
                        Button { color: Color::Secondary, onclick: move |_| show_modal_fs.set(false), "Close" }
                    },
                }
            }

            // Dropdown
            h3 { class: "mb-3", "Dropdown" }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Dropdown {
                    open: dropdown_open,
                    toggle: rsx! { "Dropdown Menu" },
                    menu: rsx! {
                        DropdownHeader { "Actions" }
                        DropdownItem { Icon { name: "pencil", class: "me-2" } "Edit" }
                        DropdownItem { Icon { name: "files", class: "me-2" } "Duplicate" }
                        DropdownDivider {}
                        // Anchor items: real links so middle-click / ctrl-click
                        // open in a background tab and copy-link works.
                        DropdownItem { href: "/settings", Icon { name: "gear", class: "me-2" } "Settings" }
                        DropdownItem { href: "https://getbootstrap.com", target: "_blank",
                            Icon { name: "box-arrow-up-right", class: "me-2" } "Docs (new tab)"
                        }
                        DropdownItem { href: "/disabled", disabled: true, "Disabled link" }
                        DropdownDivider {}
                        DropdownItem { Icon { name: "trash", class: "me-2" } "Delete" }
                    },
                }
                // Split dropdown
                Dropdown {
                    open: split_dropdown_open,
                    split: true,
                    color: Some(Color::Success),
                    toggle: rsx! { "Split Action" },
                    menu: rsx! {
                        DropdownItem { "Action 1" }
                        DropdownItem { "Action 2" }
                        DropdownDivider {}
                        DropdownItem { "Separated action" }
                    },
                }
                // End-aligned dropdown (JS-free right alignment)
                Dropdown {
                    open: end_dropdown_open,
                    align_end: true,
                    color: Some(Color::Secondary),
                    toggle: rsx! { "End-aligned" },
                    menu: rsx! {
                        DropdownItem { "Right-aligned 1" }
                        DropdownItem { "Right-aligned 2" }
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

            // Accordion
            h3 { class: "mb-3", "Accordion" }
            Accordion { open: accordion_open, class: "mb-4",
                AccordionItem { index: 0, title: "Accordion Item #1".to_string(), open: accordion_open,
                    "This is the first item's accordion body. It is shown by default."
                }
                AccordionItem { index: 1, title: "Accordion Item #2".to_string(), open: accordion_open,
                    "This is the second item's accordion body. Click the header to toggle."
                }
                AccordionItem { index: 2, title: "Accordion Item #3".to_string(), open: accordion_open,
                    "This is the third item's accordion body. Only one item is open at a time."
                }
            }

            // Tabs with different styles
            h3 { class: "mb-3", "Tabs" }
            h5 { "Standard Tabs" }
            TabList {
                active: use_signal(|| 0usize),
                tabs: vec![
                    TabDef { label: "Home".into(), icon: Some("house".into()), content: rsx! { p { class: "mt-2", "Home tab content." } } },
                    TabDef { label: "Profile".into(), icon: Some("person".into()), content: rsx! { p { class: "mt-2", "Profile tab content." } } },
                    TabDef { label: "Messages".into(), icon: None, content: rsx! { p { class: "mt-2", "Messages tab content." } } },
                ],
            }
            h5 { class: "mt-3", "Pills (fill)" }
            TabList {
                active: use_signal(|| 0usize),
                pills: true,
                tabs: vec![
                    TabDef { label: "Tab A".into(), icon: None, content: rsx! { p { class: "mt-2", "Tab A content." } } },
                    TabDef { label: "Tab B".into(), icon: None, content: rsx! { p { class: "mt-2", "Tab B content." } } },
                    TabDef { label: "Tab C".into(), icon: None, content: rsx! { p { class: "mt-2", "Tab C content." } } },
                ],
                class: "nav-fill".to_string(),
            }

            // Toast
            h3 { class: "mb-3 mt-4", "Toast" }
            div { class: "mb-4",
                div { class: "d-flex flex-wrap gap-2",
                    Button { color: Color::Success, onclick: move |_| toast_show.set(true),
                        Icon { name: "bell", class: "me-1" }
                        "Show Toast"
                    }
                    Button { color: Color::Primary, onclick: move |_| toast_headerless_show.set(true),
                        Icon { name: "chat-square-text", class: "me-1" }
                        "Headerless Toast"
                    }
                }
                ToastContainer { position: ToastPosition::TopEnd,
                    Toast { show: toast_show, title: "Notification".to_string(), subtitle: "just now".to_string(),
                        on_dismiss: move |_| { /* handle dismiss */ },
                        "This toast is controlled by a Dioxus signal. No JavaScript!"
                    }
                    Toast { show: toast_headerless_show, show_close: true, color: Color::Primary,
                        on_dismiss: move |_| { /* handle dismiss */ },
                        "A headerless toast with the d-flex close button pattern."
                    }
                }
            }

            // Offcanvas
            h3 { class: "mb-3", "Offcanvas" }
            div { class: "mb-4",
                Button { color: Color::Info, onclick: move |_| offcanvas_show.set(true),
                    Icon { name: "layout-sidebar", class: "me-1" }
                    "Open Offcanvas"
                }
                Offcanvas { show: offcanvas_show, title: "Sidebar Menu".to_string(),
                    p { "This sidebar slides in from the left. Powered by Dioxus signals." }
                    ListGroup { flush: true,
                        ListGroupItem { Icon { name: "house", class: "me-2" } "Home" }
                        ListGroupItem { Icon { name: "speedometer2", class: "me-2" } "Dashboard" }
                        ListGroupItem { Icon { name: "gear", class: "me-2" } "Settings" }
                    }
                }
            }
        }
    }
}

// ── Overlays (Tooltips & Popovers) ─────────────────────────────────────────

#[component]
fn OverlaysSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Tooltips
            h3 { class: "mb-3", "Tooltips" }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Tooltip { text: "Tooltip on top".to_string(), placement: TooltipPlacement::Top,
                    Button { id: "tooltip-hover-trigger", color: Color::Primary, "Hover" }
                }
                Tooltip {
                    text: "Tooltip on focus".to_string(),
                    placement: TooltipPlacement::Bottom,
                    trigger: TooltipTriggers::FOCUS,
                    Button { id: "tooltip-focus-trigger", color: Color::Secondary, "Focus" }
                }
                Tooltip {
                    text: "Tooltip on click".to_string(),
                    placement: TooltipPlacement::End,
                    trigger: TooltipTriggers::CLICK,
                    Button { id: "tooltip-click-trigger", color: Color::Info, "Click" }
                }
                Tooltip {
                    text: "Fallback below when top overflows".to_string(),
                    placement: TooltipPlacement::Top,
                    fallback_placements: vec![TooltipPlacement::Bottom],
                    Button { id: "tooltip-edge-trigger", color: Color::Warning, "Viewport fallback" }
                }
                Tooltip { text: "Disabled action unavailable".to_string(), placement: TooltipPlacement::Top,
                    TooltipDisabledTrigger {
                        Button { id: "tooltip-disabled-trigger", color: Color::Danger, disabled: true, "Disabled" }
                    }
                }
            }

            // Popovers
            h3 { class: "mb-3", "Popovers" }
            p { class: "text-muted mb-3", "Typed click, focus, viewport fallback, and disabled-trigger popovers." }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Popover {
                    title: "Click Popover".to_string(),
                    body: rsx! { "Default click trigger with Bootstrap's right/end placement." },
                    Button { id: "popover-click-trigger", color: Color::Primary, "Click" }
                }
                Popover {
                    title: "Focus Dismiss".to_string(),
                    body: rsx! { "Move focus away to dismiss this popover." },
                    placement: PopoverPlacement::Bottom,
                    trigger: PopoverTriggers::FOCUS,
                    Button { id: "popover-focus-trigger", color: Color::Secondary, "Focus" }
                }
                Popover {
                    title: "Outside Dismiss".to_string(),
                    body: rsx! { "Click outside the trigger or popover to close." },
                    placement: PopoverPlacement::Top,
                    Button { id: "popover-outside-trigger", color: Color::Success, "Outside" }
                }
                div { id: "popover-edge-container", style: "display: inline-block;",
                    Popover {
                        title: "Fallback Popover".to_string(),
                        body: rsx! { "Falls back below when top would overflow." },
                        placement: PopoverPlacement::Top,
                        fallback_placements: vec![PopoverPlacement::Bottom],
                        Button { id: "popover-edge-trigger", color: Color::Warning, "Viewport fallback" }
                    }
                }
                Popover {
                    title: "".to_string(),
                    body: rsx! {},
                    Button { id: "popover-empty-trigger", color: Color::Light, "Empty content" }
                }
                Popover {
                    title: "Disabled Popover".to_string(),
                    body: rsx! { "Disabled controls use a focusable wrapper." },
                    trigger: PopoverTriggers::HOVER_FOCUS,
                    PopoverDisabledTrigger {
                        Button { id: "popover-disabled-trigger", color: Color::Danger, disabled: true, "Disabled" }
                    }
                }
            }
        }
    }
}

fn showcase_svg(width: u32, height: u32, fill: &str, label: &str) -> String {
    let fill = fill.replace('#', "%23");
    let label = label.replace(' ', "%20");
    format!(
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'%3E%3Crect width='{width}' height='{height}' fill='{fill}'/%3E%3Ctext x='50%25' y='52%25' text-anchor='middle' font-family='Arial,sans-serif' font-size='34' font-weight='700' fill='white'%3E{label}%3C/text%3E%3C/svg%3E"
    )
}

// ── Media (Carousel, Figure, Ratio) ────────────────────────────────────────

#[component]
fn MediaSection() -> Element {
    let carousel_active = use_signal(|| 0usize);
    let slide_one = showcase_svg(600, 300, "#0d6efd", "First Slide");
    let slide_two = showcase_svg(600, 300, "#198754", "Second Slide");
    let slide_three = showcase_svg(600, 300, "#6f42c1", "Third Slide");
    let figure_image = showcase_svg(400, 250, "#0dcaf0", "Figure");

    rsx! {
        div { class: "mt-3",
            // Carousel
            h3 { class: "mb-3", "Carousel" }
            div { class: "mb-4", style: "max-width: 600px;",
                Carousel {
                    active: carousel_active,
                    ride: true,
                    interval: 4000,
                slides: vec![
                    CarouselSlide {
                        src: slide_one,
                            alt: "First slide".into(),
                            caption_title: Some("First Slide".into()),
                            caption_text: Some("This is the first slide caption.".into()),
                        },
                    CarouselSlide {
                        src: slide_two,
                            alt: "Second slide".into(),
                            caption_title: Some("Second Slide".into()),
                            caption_text: Some("Another slide with a caption.".into()),
                        },
                    CarouselSlide {
                        src: slide_three,
                            alt: "Third slide".into(),
                            caption_title: None,
                            caption_text: None,
                        },
                    ],
                }
                p { class: "text-muted mt-2", "Active slide: {carousel_active}" }
            }

            // Figure
            h3 { class: "mb-3", "Figure" }
            Figure {
                src: figure_image,
                alt: "Sample image".to_string(),
                caption: "A caption for this figure image.".to_string(),
                rounded: true,
            }

            // Ratio (responsive embeds)
            h3 { class: "mb-3 mt-4", "Ratio (Responsive Embeds)" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    h5 { "16x9" }
                    Ratio { aspect: "16x9".to_string(),
                        div { class: "bg-primary-subtle d-flex align-items-center justify-content-center h-100 rounded",
                            "16x9 content"
                        }
                    }
                }
                Col { md: ColumnSize::Span(3),
                    h5 { "4x3" }
                    Ratio { aspect: "4x3".to_string(),
                        div { class: "bg-success-subtle d-flex align-items-center justify-content-center h-100 rounded",
                            "4x3"
                        }
                    }
                }
                Col { md: ColumnSize::Span(3),
                    h5 { "1x1" }
                    Ratio { aspect: "1x1".to_string(),
                        div { class: "bg-warning-subtle d-flex align-items-center justify-content-center h-100 rounded",
                            "1x1"
                        }
                    }
                }
            }
        }
    }
}

// ── Navigation ──────────────────────────────────────────────────────────────

#[component]
fn NavigationSection() -> Element {
    let body_scrollspy_active = use_signal(String::new);
    let custom_scrollspy_active = use_signal(String::new);
    let mut show_dynamic_scrollspy_section = use_signal(|| false);
    let mut spa_section = use_signal(|| 0usize);
    let custom_refresh_key = if *show_dynamic_scrollspy_section.read() {
        1
    } else {
        0
    };

    rsx! {
        div { class: "mt-3",
            // Nav — Pills
            h3 { class: "mb-3", "Nav — Pills" }
            Nav { pills: true, class: "mb-4",
                NavItem { NavLink { active: true, "Active" } }
                NavItem { NavLink { "Link" } }
                NavItem { NavLink { "Another Link" } }
                NavItem { NavLink { disabled: true, "Disabled" } }
            }

            // Nav — SPA button switcher (button.nav-link, no navigation)
            h3 { class: "mb-3", "Nav — SPA switcher (NavButton)" }
            Nav { pills: true, class: "mb-2",
                NavItem { NavButton { active: spa_section() == 0, onclick: move |_| spa_section.set(0), "General" } }
                NavItem { NavButton { active: spa_section() == 1, onclick: move |_| spa_section.set(1), "Account" } }
                NavItem { NavButton { disabled: true, "Disabled" } }
            }
            p { class: "text-muted mb-4",
                if spa_section() == 0 { "General settings panel." } else { "Account settings panel." }
            }

            // Nav — Tabs
            h3 { class: "mb-3", "Nav — Tabs" }
            Nav { tabs: true, class: "mb-4",
                NavItem { NavLink { active: true, "Home" } }
                NavItem { NavLink { "Profile" } }
                NavItem { NavLink { "Messages" } }
            }

            // Nav — Underline
            h3 { class: "mb-3", "Nav — Underline" }
            Nav { underline: true, class: "mb-4",
                NavItem { NavLink { active: true, "Active" } }
                NavItem { NavLink { "Link" } }
                NavItem { NavLink { "Another" } }
            }

            // Nav — Fill
            h3 { class: "mb-3", "Nav — Fill" }
            Nav { pills: true, fill: true, class: "mb-4",
                NavItem { NavLink { active: true, "Home" } }
                NavItem { NavLink { "Much longer nav link" } }
                NavItem { NavLink { "Short" } }
            }

            // Nav — Justified
            h3 { class: "mb-3", "Nav — Justified" }
            Nav { pills: true, justified: true, class: "mb-4",
                NavItem { NavLink { active: true, "Equal" } }
                NavItem { NavLink { "Width" } }
                NavItem { NavLink { "Items" } }
            }

            // Nav — Vertical
            h3 { class: "mb-3", "Nav — Vertical" }
            Row { class: "mb-4",
                Col { md: ColumnSize::Span(3),
                    Nav { pills: true, vertical: true,
                        NavItem { NavLink { active: true, "Home" } }
                        NavItem { NavLink { "Profile" } }
                        NavItem { NavLink { "Messages" } }
                        NavItem { NavLink { disabled: true, "Disabled" } }
                    }
                }
                Col { md: ColumnSize::Span(9),
                    Card {
                        body: rsx! { "Content area next to vertical nav." },
                    }
                }
            }

            // Breadcrumb
            h3 { class: "mb-3", "Breadcrumb" }
            Breadcrumb {
                BreadcrumbItem { href: "#", "Home" }
                BreadcrumbItem { href: "#", "Library" }
                BreadcrumbItem { active: true, "Data" }
            }

            // Scrollspy
            h3 { class: "mb-3 mt-4", "Scrollspy" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    Scrollspy {
                        target: "#scrollspy-body-nav",
                        root: "body",
                        active: body_scrollspy_active,
                        offset: 96,
                        root_margin: "0px 0px -65%".to_string(),
                    }
                    Nav {
                        id: "scrollspy-body-nav",
                        pills: true,
                        vertical: true,
                        class: "gap-1 mb-2",
                        NavItem { NavLink { href: "#scrollspy-body-alpha", "Alpha" } }
                        NavItem { NavLink { href: "#scrollspy-body-beta", "Beta" } }
                        NavItem { NavLink { href: "#scrollspy-body-gamma", "Gamma" } }
                    }
                    div { class: "small text-muted mb-2",
                        "Body active: "
                        span { id: "scrollspy-body-active", "{body_scrollspy_active}" }
                    }
                    div { id: "scrollspy-body-content", class: "border rounded p-3",
                        section { id: "scrollspy-body-alpha", style: "min-height: 260px;",
                            h4 { "Alpha" }
                            p { "First body-scrolled section." }
                        }
                        section { id: "scrollspy-body-beta", style: "min-height: 260px;",
                            h4 { "Beta" }
                            p { "Second body-scrolled section." }
                        }
                        section { id: "scrollspy-body-gamma", style: "min-height: 260px;",
                            h4 { "Gamma" }
                            p { "Third body-scrolled section." }
                        }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    Scrollspy {
                        smooth_scroll: true,
                        threshold: vec![0.0, 0.25, 0.5, 1.0],
                        target: "#scrollspy-custom-nav",
                        root: "#scrollspy-custom-root",
                        active: custom_scrollspy_active,
                        offset: 12,
                        root_margin: "0px 0px -60%".to_string(),
                        refresh_key: custom_refresh_key,
                    }
                    Nav {
                        id: "scrollspy-custom-nav",
                        pills: true,
                        class: "gap-1 mb-2",
                        NavItem { NavLink { href: "#scrollspy-custom-one", "One" } }
                        NavItem { NavLink { href: "#scrollspy-custom-two", "Two" } }
                        NavItem { NavLink { href: "#scrollspy-custom-three", "Three" } }
                        if *show_dynamic_scrollspy_section.read() {
                            NavItem { NavLink { href: "#scrollspy-custom-four", "Four" } }
                        }
                    }
                    div { class: "d-flex align-items-center gap-2 mb-2",
                        span { class: "small text-muted",
                            "Custom active: "
                            span { id: "scrollspy-custom-active", "{custom_scrollspy_active}" }
                        }
                        Button {
                            id: "scrollspy-add-section",
                            color: Color::Secondary,
                            size: Size::Sm,
                            onclick: move |_| show_dynamic_scrollspy_section.set(true),
                            "Add section"
                        }
                    }
                    div {
                        id: "scrollspy-custom-root",
                        class: "border rounded p-3",
                        tabindex: "0",
                        style: "height: 260px; overflow-y: auto; position: relative;",
                        section { id: "scrollspy-custom-one", style: "min-height: 230px;",
                            h4 { "One" }
                            p { "First custom-scroll section." }
                        }
                        section { id: "scrollspy-custom-two", style: "min-height: 230px;",
                            h4 { "Two" }
                            p { "Second custom-scroll section." }
                        }
                        section { id: "scrollspy-custom-three", style: "min-height: 230px;",
                            h4 { "Three" }
                            p { "Third custom-scroll section." }
                        }
                        if *show_dynamic_scrollspy_section.read() {
                            section { id: "scrollspy-custom-four", style: "min-height: 230px;",
                                h4 { "Four" }
                                p { "Dynamically added custom-scroll section." }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── More ────────────────────────────────────────────────────────────────────

#[component]
fn MoreSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Placeholders
            h3 { class: "mb-3", "Placeholders (Loading Skeletons)" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            Placeholder { width: 7, glow: true }
                            Placeholder { width: 4, glow: true }
                            Placeholder { width: 6, glow: true }
                            Placeholder { width: 8, glow: true }
                        },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            Placeholder { width: 6, wave: true, color: Color::Primary }
                            Placeholder { width: 9, wave: true, color: Color::Primary }
                            Placeholder { width: 5, wave: true, color: Color::Primary }
                        },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            h5 { class: "card-title", "Real Content" }
                            p { class: "card-text", "Compare this card with the loading placeholders on the left." }
                            Button { color: Color::Primary, size: Size::Sm, "Action" }
                        },
                    }
                }
            }
        }
    }
}

// ── What's new in 0.6.0 ─────────────────────────────────────────────────────
//
// Every component and prop 0.6.0 added, rendered. The point of this section is
// that a released feature nobody can see is indistinguishable from one that was
// never shipped — and because e2e/showcase.spec.ts drives this page in a real
// browser, everything demonstrated here is also everything covered there.

#[component]
fn WhatsNewSection() -> Element {
    let mut checked_bold = use_signal(|| true);
    let mut checked_italic = use_signal(|| false);
    let mut radio_choice = use_signal(|| "center".to_string());
    let mut card_log = use_signal(String::new);
    let mut badge_clicks = use_signal(|| 0u32);
    let mut crumb_log = use_signal(String::new);
    let mut mouse_log = use_signal(String::new);
    let show_slots = use_signal(|| false);
    let inner_tab = use_signal(|| 0usize);
    let mut toast_a = use_signal(|| true);
    let mut toast_b = use_signal(|| true);

    rsx! {
        div { class: "mt-3",
            Alert {
                color: Color::Info,
                heading: "New in 0.6.0".to_string(),
                "Two new components and 27 typed props. Everything below is the additions — "
                "the other tabs cover the rest of Bootstrap 5.3."
            }

            // ── Toggle buttons (btn-check) ──────────────────────────────────
            h3 { class: "mb-2", "Toggle Buttons" }
            p { class: "text-muted",
                "Bootstrap's "
                code { "btn-check" }
                " idiom: a visually-hidden checkbox or radio paired with a "
                code { "<label class=\"btn\">" }
                ". Keyboard-focusable and screen-reader-correct, unlike a button that only looks pressed."
            }

            h5 { class: "mt-3", "CheckboxButton — independent toggles" }
            div { class: "d-flex flex-wrap gap-2 mb-2",
                CheckboxButton {
                    id: "nw-chk-bold",
                    label: "Bold".to_string(),
                    color: Color::Primary,
                    checked: checked_bold(),
                    onchange: move |_| checked_bold.set(!checked_bold()),
                }
                CheckboxButton {
                    id: "nw-chk-italic",
                    label: "Italic".to_string(),
                    color: Color::Primary,
                    outline: true,
                    checked: checked_italic(),
                    onchange: move |_| checked_italic.set(!checked_italic()),
                }
                CheckboxButton {
                    id: "nw-chk-sm",
                    label: "Small".to_string(),
                    color: Color::Secondary,
                    size: Size::Sm,
                    name: "nw-opts",
                    value: "small",
                    autocomplete: "off",
                    class: "text-uppercase",
                }
                CheckboxButton {
                    id: "nw-chk-lg",
                    label: "Large".to_string(),
                    color: Color::Success,
                    size: Size::Lg,
                }
                CheckboxButton {
                    id: "nw-chk-disabled",
                    label: "Disabled".to_string(),
                    color: Color::Dark,
                    disabled: true,
                }
            }
            p { class: "small text-muted mb-4",
                "Bold is " strong { if checked_bold() { "on" } else { "off" } }
                ", italic is " strong { if checked_italic() { "on" } else { "off" } } "."
            }

            h5 { "RadioButton — one of a group" }
            p { class: "text-muted small",
                "Grouped by a shared " code { "name" } "; each carries its own " code { "value" } "."
            }
            div { class: "d-flex flex-wrap gap-2 mb-2",
                RadioButton {
                    id: "nw-radio-start",
                    name: "nw-align",
                    value: "start",
                    label: "Start".to_string(),
                    color: Color::Info,
                    outline: true,
                    checked: radio_choice() == "start",
                    onchange: move |_| radio_choice.set("start".to_string()),
                }
                RadioButton {
                    id: "nw-radio-center",
                    name: "nw-align",
                    value: "center",
                    label: "Center".to_string(),
                    color: Color::Info,
                    outline: true,
                    checked: radio_choice() == "center",
                    onchange: move |_| radio_choice.set("center".to_string()),
                }
                RadioButton {
                    id: "nw-radio-end",
                    name: "nw-align",
                    value: "end",
                    label: "End".to_string(),
                    color: Color::Info,
                    outline: true,
                    autocomplete: "off",
                    class: "fst-italic",
                    checked: radio_choice() == "end",
                    onchange: move |_| radio_choice.set("end".to_string()),
                }
                RadioButton {
                    id: "nw-radio-justify",
                    name: "nw-align",
                    value: "justify",
                    label: "Justify (disabled, large)".to_string(),
                    color: Color::Info,
                    outline: true,
                    size: Size::Lg,
                    disabled: true,
                }
            }
            p { class: "small text-muted mb-4", "Selected: " strong { "{radio_choice}" } }

            // ── Button additions ────────────────────────────────────────────
            h3 { class: "mb-2", "Button — link, plain, rel, role, onmousedown" }
            div { class: "d-flex flex-wrap gap-2 align-items-center mb-2",
                Button { color: Color::Primary, link: true, "Link style" }
                Button { plain: true, "Plain (no variant)" }
                Button { plain: true, size: Size::Sm, class: "border", "Plain small" }
                Button {
                    href: "https://getbootstrap.com/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    color: Color::Secondary,
                    outline: true,
                    Icon { name: "box-arrow-up-right", class: "me-1" }
                    "rel=noopener"
                }
                Button { color: Color::Dark, role: "switch", "role=switch" }
                Button {
                    color: Color::Warning,
                    onmousedown: move |_| mouse_log.set("mousedown fired before click".to_string()),
                    "onmousedown"
                }
            }
            p { class: "small text-muted mb-4",
                if mouse_log().is_empty() { "Press the last button to see onmousedown fire." } else { "{mouse_log}" }
            }

            // ── Badge fill ──────────────────────────────────────────────────
            h3 { class: "mb-2", "Badge — fill and onclick" }
            p { class: "text-muted",
                code { "BadgeFill::TextBg" }
                " (the default) emits "
                code { "text-bg-*" }
                ", which sets a contrasting foreground as well as the background. "
                code { "BadgeFill::Bg" }
                " emits the plain "
                code { "bg-*" }
                " for markup that sets its own text colour."
            }
            div { class: "d-flex flex-wrap gap-2 align-items-center mb-2",
                Badge { color: Color::Primary, "text-bg (default)" }
                Badge { color: Color::Primary, fill: BadgeFill::Bg, "bg only" }
                Badge { color: Color::Warning, "text-bg warning" }
                Badge { color: Color::Warning, fill: BadgeFill::Bg, "bg warning" }
                Badge { color: Color::Success, pill: true, "pill" }
                Badge {
                    color: Color::Danger,
                    class: "user-select-none",
                    onclick: move |_| badge_clicks += 1,
                    Icon { name: "hand-index", class: "me-1" }
                    "clicked {badge_clicks}×"
                }
            }
            p { class: "small text-muted mb-4",
                "The two fills differ most on light backgrounds — compare the warning pair."
            }

            // ── Card additions ──────────────────────────────────────────────
            h3 { class: "mb-2", "Card — link, ids, styles, click handlers" }
            Row { class: "g-3 mb-2",
                Col { md: ColumnSize::Span(4),
                    Card {
                        href: "https://getbootstrap.com/",
                        target: "_blank",
                        class: "h-100",
                        header: rsx! { strong { "Whole card is a link" } },
                        body: rsx! {
                            p { class: "mb-0", "Rendered as an anchor wrapping the card." }
                        },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        class: "h-100",
                        header_class: "bg-primary-subtle fw-semibold",
                        body_class: "small",
                        footer_class: "text-muted small",
                        header: rsx! { "Per-slot classes" },
                        body: rsx! { "header_class, body_class and footer_class each target one slot." },
                        footer: rsx! { "footer_class" },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        class: "h-100",
                        body_id: "nw-card-body",
                        body_style: "border-left: 4px solid var(--bs-info);",
                        onclick: move |_| card_log.set("card clicked".to_string()),
                        oncontextmenu: move |_| card_log.set("card right-clicked".to_string()),
                        body: rsx! {
                            p { class: "mb-1", "body_id + body_style, and both click handlers." }
                            p { class: "mb-0 small text-muted", "Try left- and right-clicking." }
                        },
                    }
                }
            }
            p { class: "small text-muted mb-4",
                if card_log().is_empty() { "No card interaction yet." } else { "{card_log}" }
            }

            // ── Alert heading ───────────────────────────────────────────────
            h3 { class: "mb-2", "Alert — heading" }
            Alert {
                color: Color::Success,
                heading: "Well done".to_string(),
                "The heading renders as a proper "
                code { "alert-heading" }
                " element above the body, rather than being hand-rolled inside it."
            }
            Alert {
                color: Color::Warning,
                heading: "Dismissible with a heading".to_string(),
                dismissible: true,
                "Both features compose."
            }

            // ── List group ──────────────────────────────────────────────────
            h3 { class: "mb-2 mt-4", "ListGroup — tag and numbered" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    p { class: "small text-muted mb-1",
                        code { "numbered: true" } " with " code { "tag: \"ol\"" }
                        " — a real ordered list, so the numbers are semantic."
                    }
                    ListGroup {
                        numbered: true,
                        tag: "ol",
                        ListGroupItem { tag: "li", "First" }
                        ListGroupItem { tag: "li", "Second" }
                        ListGroupItem { tag: "li", "Third" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    p { class: "small text-muted mb-1",
                        "Default " code { "div" } " rendering, with a flush variant."
                    }
                    ListGroup {
                        flush: true,
                        ListGroupItem { active: true, "Active" }
                        ListGroupItem { color: Some(Color::Success), "Success" }
                        ListGroupItem { disabled: true, "Disabled" }
                    }
                }
            }

            // ── Breadcrumb onclick ──────────────────────────────────────────
            h3 { class: "mb-2", "BreadcrumbItem — onclick" }
            Breadcrumb {
                BreadcrumbItem {
                    href: "#",
                    onclick: move |_| crumb_log.set("navigated: Home".to_string()),
                    "Home"
                }
                BreadcrumbItem {
                    href: "#",
                    onclick: move |_| crumb_log.set("navigated: Library".to_string()),
                    "Library"
                }
                BreadcrumbItem { active: true, "Data" }
            }
            p { class: "small text-muted mb-4",
                if crumb_log().is_empty() { "Click a crumb — the handler fires without a page load." } else { "{crumb_log}" }
            }

            // ── Navbar container ────────────────────────────────────────────
            h3 { class: "mb-2", "Navbar — container" }
            p { class: "text-muted",
                "Controls the wrapper Bootstrap puts inside the navbar: a fixed-width "
                code { "container" } ", a " code { "container-fluid" } ", or none at all."
            }
            div { class: "border rounded mb-2 overflow-hidden",
                Navbar {
                    color: Some(Color::Dark),
                    container: NavbarContainer::Fluid,
                    class: "position-static",
                    brand: rsx! { span { class: "navbar-brand mb-0", "container-fluid" } },
                }
            }
            div { class: "border rounded mb-4 overflow-hidden",
                Navbar {
                    color: Some(Color::Primary),
                    container: NavbarContainer::None,
                    class: "position-static px-3",
                    brand: rsx! { span { class: "navbar-brand mb-0", "no container" } },
                }
            }

            // ── Modal slot classes ──────────────────────────────────────────
            h3 { class: "mb-2", "Modal — custom header and per-slot classes" }
            p { class: "text-muted",
                code { "header" } " replaces the default title bar entirely; "
                code { "content_class" } ", " code { "header_class" } ", "
                code { "body_class" } " and " code { "footer_class" }
                " style each region without wrapping it."
            }
            ModalSlotsDemo { show: show_slots }
            p { class: "mb-4" }

            // ── TabList ─────────────────────────────────────────────────────
            h3 { class: "mb-2", "TabList — fill, justified, content styling" }
            TabList {
                active: inner_tab,
                fill: true,
                content_class: "border border-top-0 rounded-bottom p-3",
                content_style: "min-height: 6rem; background: var(--bs-tertiary-bg);",
                tabs: vec![
                    TabDef {
                        label: "One".into(),
                        icon: Some("1-circle".into()),
                        content: rsx! { p { class: "mb-0", "content_style sets the panel background and a minimum height." } },
                    },
                    TabDef {
                        label: "Two".into(),
                        icon: Some("2-circle".into()),
                        content: rsx! { p { class: "mb-0", "fill spreads the tabs across the full width." } },
                    },
                    TabDef {
                        label: "Three".into(),
                        icon: None,
                        content: rsx! { p { class: "mb-0", "content_class styles the panel itself." } },
                    },
                ],
            }

            // ── Toast positioned ────────────────────────────────────────────
            h3 { class: "mb-2 mt-4", "ToastContainer — positioned" }
            p { class: "text-muted",
                code { "positioned: false" }
                " drops the "
                code { "position-fixed" }
                " and placement utilities so the host's own CSS can win. Note that Bootstrap's base "
                code { ".toast-container" }
                " rule is still "
                code { "position: absolute" }
                " — so to place the stack in normal document flow you supply that yourself, as this "
                "example does with " code { "position-static" } ". Dropping the utilities is the prop's "
                "job; choosing the replacement is yours."
            }
            div { class: "border rounded p-3 mb-4",
                ToastContainer {
                    positioned: false,
                    class: "gap-2 position-static",
                    Toast {
                        show: toast_a,
                        title: "Inline".to_string(),
                        autohide: false,
                        "This stack is in the document flow."
                    }
                    Toast {
                        show: toast_b,
                        title: "Second".to_string(),
                        subtitle: "no overlay".to_string(),
                        color: Some(Color::Success),
                        autohide: false,
                        "Stacked underneath, not overlaid."
                    }
                }
            }
            Button {
                color: Color::Secondary,
                outline: true,
                size: Size::Sm,
                class: "mb-4",
                onclick: move |_| { toast_a.set(true); toast_b.set(true); },
                "Restore both toasts"
            }

            // ── Input / Textarea additions ──────────────────────────────────
            h3 { class: "mb-2", "Input and Textarea — typed attributes and events" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4),
                    FormGroup { label: "Number with step/min/max".to_string(),
                        Input {
                            r#type: "number",
                            step: "0.25",
                            min: "0",
                            max: "10",
                            value: "2.5",
                        }
                    }
                }
                Col { md: ColumnSize::Span(4),
                    FormGroup { label: "File with accept".to_string(),
                        Input { r#type: "file", accept: "image/png,image/jpeg" }
                    }
                }
                Col { md: ColumnSize::Span(4),
                    FormGroup { label: "autocorrect / autocomplete".to_string(),
                        Input {
                            placeholder: "no autocorrect",
                            autocorrect: "off",
                            autocomplete: "off",
                        }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Textarea with typed attributes".to_string(),
                        Textarea {
                            rows: 3,
                            placeholder: "autocomplete and autocorrect are typed props",
                            autocomplete: "off",
                            autocorrect: "off",
                        }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Readonly and disabled".to_string(),
                        Textarea { rows: 3, readonly: true, value: "readonly" }
                    }
                }
            }
        }
    }
}

#[component]
fn ModalSlotsDemo(show: Signal<bool>) -> Element {
    let mut show = show;
    rsx! {
        Button { color: Color::Primary, onclick: move |_| show.set(true), "Open slotted modal" }
        Modal {
            show: show,
            size: ModalSize::Lg,
            scrollable: true,
            content_class: "border border-primary",
            header_class: "bg-primary-subtle",
            body_class: "bg-body-tertiary",
            footer_class: "justify-content-between",
            header: rsx! {
                div { class: "d-flex align-items-center gap-2",
                    Icon { name: "stars", class: "text-primary" }
                    strong { "A fully custom header" }
                    Badge { color: Color::Primary, fill: BadgeFill::Bg, "0.6.0" }
                }
            },
            body: rsx! {
                p { "The header slot replaces the default title bar, so it can hold anything." }
                p { class: "mb-0",
                    "Each region also takes its own class prop, which is how the border, "
                    "tinted header and split footer below are done."
                }
            },
            footer: rsx! {
                span { class: "small text-muted", "footer_class: justify-content-between" }
                Button { color: Color::Primary, onclick: move |_| show.set(false), "Done" }
            },
        }
    }
}

// ── Complete prop coverage ──────────────────────────────────────────────────
//
// The props the themed sections above do not reach. This exists so the site can
// answer "does it support X" by showing X rather than by a reader inferring it
// from a type signature, and so the browser suite covers them.
//
// Two props are deliberately absent, and the reason is recorded rather than left
// to look like an oversight:
//
//   BootstrapThemeProvider { theme } injects a global <style> overriding
//   Bootstrap's colour variables. Demonstrating it here would recolour the whole
//   page; demonstrating it with an empty theme would render nothing at all, and a
//   demo that behaves identically whether or not the feature works is not
//   evidence the feature works. It belongs in a dedicated example, not a section.

#[component]
fn CoverageSection() -> Element {
    let mut event_log = use_signal(String::new);
    let mut click_log = use_signal(String::new);
    let acc_open = use_signal(|| Some(0usize));
    let carousel = use_signal(|| 0usize);
    let collapse_h = use_signal(|| false);
    let drop_up = use_signal(|| false);
    let drop_end = use_signal(|| false);
    let page = use_signal(|| 3usize);
    let oc_top = use_signal(|| false);
    let oc_bottom = use_signal(|| false);
    let oc_responsive = use_signal(|| false);
    let mut check_a = use_signal(|| false);
    let mut switch_on = use_signal(|| true);
    let mut radio_pick = use_signal(|| "a".to_string());
    let mut range_val = use_signal(|| "40".to_string());
    let mut select_val = use_signal(|| "two".to_string());
    let toast_timed = use_signal(|| false);
    let modal_strict = use_signal(|| false);
    let nested_tab = use_signal(|| 0usize);
    let theme_local = use_signal(|| Theme::Dark);

    rsx! {
        div { class: "mt-3",
            p { class: "lead",
                "Everything not covered by the themed tabs. If a prop exists, it renders here."
            }

            // ── Layout ──────────────────────────────────────────────────────
            h3 { class: "mb-2", "Layout" }
            h5 { "Container — fluid" }
            Container { fluid: true, class: "bg-primary-subtle border rounded py-2 mb-3",
                "container-fluid spans the full width at every breakpoint."
            }

            h5 { "Col — every breakpoint, offset and order" }
            Row { class: "g-2 mb-2",
                Col { xs: ColumnSize::Span(6), sm: ColumnSize::Span(4), xl: ColumnSize::Span(3), xxl: ColumnSize::Span(2),
                    div { class: "p-2 bg-body-tertiary border rounded small", "xs-6 sm-4 xl-3 xxl-2" }
                }
                Col { xs: ColumnSize::Span(6), sm: ColumnSize::Span(8), xl: ColumnSize::Span(9), xxl: ColumnSize::Span(10),
                    div { class: "p-2 bg-body-tertiary border rounded small", "the complement" }
                }
            }
            Row { class: "g-2 mb-2",
                Col { md: ColumnSize::Span(4), offset: Some(4), offset_sm: Some(2), offset_lg: Some(4),
                    div { class: "p-2 bg-success-subtle border rounded small", "offset + offset_sm + offset_lg" }
                }
                Col { md: ColumnSize::Span(4), offset_xl: Some(0), offset_xxl: Some(0),
                    div { class: "p-2 bg-success-subtle border rounded small", "offset_xl / offset_xxl" }
                }
            }
            Row { class: "g-2 mb-4",
                Col { md: ColumnSize::Span(4), order: Some(3), order_sm: Some(3), order_lg: Some(3),
                    div { class: "p-2 bg-warning-subtle border rounded small", "order 3 (renders last)" }
                }
                Col { md: ColumnSize::Span(4), order: Some(1), order_md: Some(1),
                    div { class: "p-2 bg-warning-subtle border rounded small", "order 1 (renders first)" }
                }
                Col { md: ColumnSize::Span(4), order: Some(2),
                    div { class: "p-2 bg-warning-subtle border rounded small", "order 2" }
                }
            }

            // ── Collapse / Accordion ────────────────────────────────────────
            h3 { class: "mb-2", "Collapse and Accordion" }
            CollapseHorizontalDemo { expanded: collapse_h }
            h5 { class: "mt-3", "Accordion — flush" }
            Accordion { open: acc_open, flush: true, class: "mb-4 border rounded",
                AccordionItem { index: 0, open: acc_open, title: "Flush removes the outer borders".to_string(),
                    "Useful when the accordion sits inside a card or list group."
                }
                AccordionItem { index: 1, open: acc_open, title: "Second item".to_string(),
                    "Only one panel opens at a time."
                }
            }

            // ── Tables ──────────────────────────────────────────────────────
            h3 { class: "mb-2", "Table — borderless, size, colour" }
            Table { borderless: true, size: Size::Sm, color: Some(Color::Dark), hover: true,
                caption: "borderless + sm + dark".to_string(), caption_top: true,
                thead { tr { th { "Prop" } th { "Effect" } } }
                tbody {
                    tr { td { "borderless" } td { "drops every rule" } }
                    tr { td { "size: Sm" } td { "table-sm, tighter padding" } }
                    tr { td { "color" } td { "table-dark and friends" } }
                }
            }

            // ── Dropdowns ───────────────────────────────────────────────────
            h3 { class: "mb-2 mt-4", "Dropdown — direction, toggle_class, item states" }
            div { class: "d-flex flex-wrap gap-3 mb-3",
                Dropdown {
                    open: drop_up,
                    direction: DropDirection::Up,
                    toggle_class: "fw-bold",
                    color: Some(Color::Secondary),
                    toggle: rsx! { "Drops up" },
                    menu: rsx! {
                        DropdownItem { active: true, "Active item" }
                        DropdownItem {
                            onclick: move |_| click_log.set("dropdown item clicked".to_string()),
                            "With onclick"
                        }
                        DropdownItem { disabled: true, "Disabled" }
                    },
                }
                Dropdown {
                    open: drop_end,
                    direction: DropDirection::End,
                    color: Some(Color::Info),
                    toggle: rsx! { "Drops end" },
                    menu: rsx! {
                        DropdownItem { href: "https://getbootstrap.com/", target: "_blank", "External link" }
                    },
                }
            }
            h5 { "DropdownMenu — standalone, always shown, end-aligned" }
            div { class: "position-relative border rounded p-3 mb-4", style: "min-height: 9rem;",
                DropdownMenu { show: true, align_end: true, class: "position-static d-inline-block",
                    DropdownItem { "align_end + show" }
                    DropdownItem { "rendered without a toggle" }
                }
            }

            // ── Form controls ───────────────────────────────────────────────
            h3 { class: "mb-2", "Form controls — disabled, sizes, events" }
            Row { class: "g-3 mb-3",
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Input events (focus, type, blur)".to_string(),
                        Input {
                            placeholder: "watch the log below",
                            list: "nw-suggestions",
                            onfocus: move |_| event_log.set("onfocus".to_string()),
                            onblur: move |_| event_log.set("onblur".to_string()),
                            onchange: move |_| event_log.set("onchange".to_string()),
                            onkeydown: move |_| event_log.set("onkeydown".to_string()),
                            onkeyup: move |_| event_log.set("onkeyup".to_string()),
                        }
                    }
                    datalist { id: "nw-suggestions",
                        option { value: "alpha" }
                        option { value: "beta" }
                        option { value: "gamma" }
                    }
                    p { class: "small text-muted",
                        "Last event: " strong { if event_log().is_empty() { "—" } else { "{event_log}" } }
                        " · the " code { "list" } " prop wires the datalist."
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Uncontrolled, readonly, disabled".to_string(),
                        Input { uncontrolled: true, value: "uncontrolled — type freely", class: "mb-2" }
                        Input { readonly: true, value: "readonly", class: "mb-2" }
                        Input { disabled: true, value: "disabled" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Textarea — size, events, uncontrolled".to_string(),
                        Textarea {
                            rows: 2,
                            size: Size::Sm,
                            uncontrolled: true,
                            placeholder: "small, uncontrolled",
                            onfocus: move |_| event_log.set("textarea onfocus".to_string()),
                            onblur: move |_| event_log.set("textarea onblur".to_string()),
                            onchange: move |_| event_log.set("textarea onchange".to_string()),
                            onkeydown: move |_| event_log.set("textarea onkeydown".to_string()),
                            onkeyup: move |_| event_log.set("textarea onkeyup".to_string()),
                        }
                        Textarea { rows: 2, disabled: true, value: "disabled", class: "mt-2" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Select — size and disabled".to_string(),
                        Select {
                            value: "{select_val}",
                            size: Size::Lg,
                            onchange: move |e: FormEvent| select_val.set(e.value()),
                            class: "mb-2",
                            option { value: "one", "Large select — one" }
                            option { value: "two", "Large select — two" }
                        }
                        Select { disabled: true,
                            option { "Disabled select" }
                        }
                    }
                }
            }
            div { class: "d-flex flex-wrap gap-4 mb-4",
                div {
                    Checkbox {
                        checked: check_a(),
                        input_id: "cov-check",
                        label: "Checkbox with input_id + onclick".to_string(),
                        onchange: move |_| check_a.set(!check_a()),
                        onclick: move |_| click_log.set("checkbox onclick".to_string()),
                    }
                    Checkbox { checked: false, label: "Disabled checkbox".to_string(), disabled: true }
                }
                div {
                    Radio {
                        name: "cov-radio",
                        checked: radio_pick() == "a",
                        label: "Radio A".to_string(),
                        onchange: move |_| radio_pick.set("a".to_string()),
                    }
                    Radio {
                        name: "cov-radio",
                        checked: radio_pick() == "b",
                        label: "Radio B".to_string(),
                        onchange: move |_| radio_pick.set("b".to_string()),
                    }
                    Radio { name: "cov-radio", checked: false, label: "Disabled".to_string(), disabled: true }
                }
                div {
                    Switch {
                        checked: switch_on(),
                        label: "Switch".to_string(),
                        onchange: move |_| switch_on.set(!switch_on()),
                    }
                    Switch { checked: false, label: "Disabled switch".to_string(), disabled: true }
                }
            }
            h5 { "Range — step and disabled" }
            div { class: "mb-4",
                Range {
                    value: "{range_val}",
                    min: "0",
                    max: "100",
                    step: "10",
                    oninput: move |e: FormEvent| range_val.set(e.value()),
                }
                p { class: "small text-muted mb-1", "step 10 → value {range_val}" }
                Range { value: "50", disabled: true }
            }

            // ── Buttons / list group ────────────────────────────────────────
            h3 { class: "mb-2", "ButtonGroup size, ListGroupItem onclick" }
            ButtonGroup { size: Size::Lg, class: "mb-2",
                Button { color: Color::Primary, "Large" }
                Button { color: Color::Primary, "Group" }
            }
            ButtonGroup { size: Size::Sm, class: "mb-3 ms-2",
                Button { color: Color::Secondary, "Small" }
                Button { color: Color::Secondary, "Group" }
            }
            ListGroup { class: "mb-4",
                ListGroupItem {
                    onclick: move |_| click_log.set("list item clicked".to_string()),
                    "Clickable list item"
                }
                ListGroupItem { "Inert list item" }
            }
            p { class: "small text-muted mb-4",
                "Click log: " strong { if click_log().is_empty() { "—" } else { "{click_log}" } }
            }

            // ── Pagination ──────────────────────────────────────────────────
            h3 { class: "mb-2", "Pagination — window and show_prev_next" }
            Pagination { current: page, total: 20, window: 3, class: "mb-2" }
            Pagination { current: page, total: 20, window: 7, show_prev_next: false, size: Size::Sm, class: "mb-4" }

            // ── Placeholders ────────────────────────────────────────────────
            h3 { class: "mb-2", "Placeholder — size, tag, wave and glow" }
            div { class: "mb-2",
                Placeholder { width: 6, size: Size::Lg, tag: "span", color: Some(Color::Primary), class: "me-2" }
                Placeholder { width: 4, size: Size::Sm, tag: "span", color: Some(Color::Secondary) }
            }
            div { class: "mb-2", PlaceholderParagraph { lines: 3, glow: true } }
            div { class: "mb-4", PlaceholderParagraph { lines: 2, wave: true } }

            // ── Figure ──────────────────────────────────────────────────────
            h3 { class: "mb-2", "Figure — thumbnail, fluid, caption alignment" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    Figure {
                        src: "{showcase_svg(320, 160, \"%230d6efd\", \"thumbnail\")}",
                        alt: "Thumbnail figure",
                        caption: "thumbnail + img_class, caption end-aligned".to_string(),
                        caption_align: "end",
                        thumbnail: true,
                        img_class: "border border-primary",
                    }
                }
                Col { md: ColumnSize::Span(6),
                    Figure {
                        src: "{showcase_svg(320, 160, \"%23198754\", \"fluid\")}",
                        alt: "Fluid figure",
                        caption: "fluid scales to the column".to_string(),
                        caption_align: "center",
                        fluid: true,
                    }
                }
            }

            // ── Offcanvas ───────────────────────────────────────────────────
            h3 { class: "mb-2", "Offcanvas — placement and behaviour flags" }
            OffcanvasDemos { top: oc_top, bottom: oc_bottom, responsive: oc_responsive }

            // ── Overlays ────────────────────────────────────────────────────
            h3 { class: "mb-2 mt-4", "Tooltip and Popover — offset, delay, forced open" }
            p { class: "text-muted",
                code { "open: Some(true)" }
                " pins an overlay open with no interaction. It stays glued to its trigger while you "
                "scroll, and suppresses itself entirely when the trigger leaves the viewport — an "
                "overlay with no on-screen anchor would otherwise be clamped into view and float over "
                "unrelated content."
            }
            div { class: "d-flex flex-wrap gap-3 align-items-center mb-3",
                Tooltip {
                    text: "Offset 16px away from the trigger".to_string(),
                    offset: OverlayOffset { skidding: 0.0, distance: 16.0 },
                    delay: TooltipDelay { show_ms: 300, hide_ms: 100 },
                    boundary_padding: 12.0,
                    Button { color: Color::Secondary, outline: true, "offset + delay" }
                }
                Tooltip {
                    text: "Forced open — open: Some(true)".to_string(),
                    open: Some(true),
                    placement: TooltipPlacement::Bottom,
                    Button { color: Color::Info, outline: true, "always visible" }
                }
                TooltipDisabledTrigger { style: "display:inline-block;",
                    Button { color: Color::Secondary, disabled: true, "disabled + tooltip" }
                }
            }
            div { class: "d-flex flex-wrap gap-3 align-items-center mb-4",
                Popover {
                    title: "Offset and delay".to_string(),
                    body: rsx! { "Pushed 20px out, with show/hide delays." },
                    offset: OverlayOffset { skidding: 0.0, distance: 20.0 },
                    delay: PopoverDelay { show_ms: 200, hide_ms: 150 },
                    boundary_padding: 16.0,
                    dismiss_on_outside_click: true,
                    Button { color: Color::Primary, outline: true, "offset + delay" }
                }
                Popover {
                    title: "Forced open".to_string(),
                    body: rsx! { "open: Some(true) pins it." },
                    open: Some(true),
                    placement: PopoverPlacement::Bottom,
                    Button { color: Color::Warning, outline: true, "pinned" }
                }
                PopoverDisabledTrigger { style: "display:inline-block;",
                    Button { color: Color::Primary, disabled: true, "disabled + popover" }
                }
            }

            // ── Carousel ────────────────────────────────────────────────────
            h3 { class: "mb-2", "Carousel — fade, dark, indicators, controls" }
            Carousel {
                active: carousel,
                fade: true,
                dark: true,
                indicators: true,
                controls: true,
                class: "mb-4 border rounded overflow-hidden",
                slides: vec![
                    CarouselSlide {
                        src: showcase_svg(960, 320, "%230d6efd", "fade"),
                        alt: "Fade transition slide".into(),
                        caption_title: Some("Fade transition".into()),
                        caption_text: Some("fade: true crossfades instead of sliding.".into()),
                    },
                    CarouselSlide {
                        src: showcase_svg(960, 320, "%23198754", "dark"),
                        alt: "Dark controls slide".into(),
                        caption_title: Some("Dark controls".into()),
                        caption_text: Some("dark: true darkens the indicators and arrows.".into()),
                    },
                ],
            }

            // ── Toast timing / theme toggle / tabs ──────────────────────────
            h3 { class: "mb-2", "Toast autohide, ThemeToggle colour, TabList justified" }
            ToastAutohideDemo { show: toast_timed }
            div { class: "d-flex align-items-center gap-3 my-3",
                span { class: "small text-muted", "ThemeToggle with an explicit colour:" }
                ThemeToggle { theme: theme_local, color: Some(Color::Warning) }
            }
            TabList {
                active: nested_tab,
                justified: true,
                pills: true,
                class: "mb-2",
                tabs: vec![
                    TabDef { label: "Justified".into(), icon: None, content: rsx! { p { class: "mb-0 small", "justified gives every tab equal width." } } },
                    TabDef { label: "Pills".into(), icon: None, content: rsx! { p { class: "mb-0 small", "combined with the pills variant." } } },
                ],
            }

            // ── Navigation odds and ends ────────────────────────────────────
            h3 { class: "mb-2 mt-4", "NavLink prevent_default, NavbarNav scroll" }
            Nav {
                NavItem {
                    NavLink {
                        href: "#",
                        prevent_default: true,
                        onclick: move |_| click_log.set("nav link — default prevented".to_string()),
                        "prevent_default + onclick"
                    }
                }
                NavItem { NavLink { href: "#", disabled: true, "Disabled" } }
            }
            div { class: "border rounded mt-2 mb-4 overflow-hidden",
                Navbar {
                    color: Some(Color::Dark),
                    container: NavbarContainer::Fluid,
                    class: "position-static",
                    brand: rsx! { span { class: "navbar-brand mb-0", "scrollable nav" } },
                    NavbarNav { scroll: true, class: "flex-row gap-3",
                        NavItem { NavLink { href: "#", "One" } }
                        NavItem { NavLink { href: "#", "Two" } }
                        NavItem { NavLink { href: "#", "Three" } }
                    }
                }
            }

            // ── Modal behaviour flags ───────────────────────────────────────
            h3 { class: "mb-2", "Modal — dismissal behaviour" }
            ModalFlagsDemo { show: modal_strict }

            // ── BootstrapHead defaults, stated explicitly ───────────────────
            h3 { class: "mb-2", "BootstrapHead — asset sources" }
            p { class: "text-muted mb-4",
                code { "BootstrapHead" }
                " takes " code { "css" } " and " code { "icons" }
                ", each of which selects bundled assets (the default), a custom URL, or none. "
                "This page passes the bundled defaults explicitly — see the top of "
                code { "main.rs" }
                " — rather than demonstrating an override here, which would change what the whole page loads."
            }
        }
    }
}

#[component]
fn CollapseHorizontalDemo(expanded: Signal<bool>) -> Element {
    let mut expanded = expanded;
    rsx! {
        h5 { "Collapse — horizontal" }
        Button {
            color: Color::Secondary,
            outline: true,
            size: Size::Sm,
            class: "mb-2",
            onclick: move |_| expanded.set(!expanded()),
            "Toggle horizontal collapse"
        }
        div { style: "min-height: 4.5rem;",
            Collapse { expanded: expanded, horizontal: true,
                div { class: "p-3 bg-body-tertiary border rounded", style: "width: 18rem;",
                    "Collapses along the horizontal axis rather than the vertical one."
                }
            }
        }
    }
}

#[component]
fn OffcanvasDemos(top: Signal<bool>, bottom: Signal<bool>, responsive: Signal<bool>) -> Element {
    let mut top = top;
    let mut bottom = bottom;
    let mut responsive = responsive;
    rsx! {
        div { class: "d-flex flex-wrap gap-2",
            Button { color: Color::Primary, outline: true, onclick: move |_| top.set(true), "Open from top" }
            Button { color: Color::Primary, outline: true, onclick: move |_| bottom.set(true), "Open from bottom (no backdrop)" }
            Button { color: Color::Secondary, outline: true, onclick: move |_| responsive.set(true), "Open responsive (md)" }
        }
        p { class: "small text-muted mt-2 mb-0",
            "The responsive one swaps the "
            code { "offcanvas" }
            " base class for "
            code { "offcanvas-md" }
            ": above the md breakpoint Bootstrap renders it inline rather than as an overlay, "
            "so on a desktop viewport it appears in the page flow with the backdrop still over it. "
            "Dismiss it with Escape or the backdrop."
        }
        Offcanvas {
            show: top,
            title: "placement: Top".to_string(),
            placement: OffcanvasPlacement::Top,
            backdrop_close: true,
            keyboard_close: true,
            show_close: true,
            p { class: "mb-0", "backdrop_close, keyboard_close and show_close are all on — backdrop, Escape or the × all dismiss it." }
        }
        Offcanvas {
            show: bottom,
            title: "placement: Bottom".to_string(),
            placement: OffcanvasPlacement::Bottom,
            backdrop: false,
            backdrop_close: false,
            keyboard_close: false,
            show_close: true,
            on_dismiss: move |_| {},
            p { class: "mb-0", "No backdrop, and neither the backdrop nor Escape dismisses it — use the close button." }
        }
        Offcanvas {
            show: responsive,
            title: "responsive: md".to_string(),
            placement: OffcanvasPlacement::Start,
            responsive: "md",
            backdrop_close: true,
            keyboard_close: true,
            show_close: true,
            p { class: "mb-0", "Rendered as offcanvas-md — inline above the breakpoint, an overlay below it." }
        }
    }
}

#[component]
fn ToastAutohideDemo(show: Signal<bool>) -> Element {
    let mut show = show;
    rsx! {
        Button {
            color: Color::Success,
            size: Size::Sm,
            onclick: move |_| show.set(true),
            "Show a toast that autohides after 3s"
        }
        ToastContainer { positioned: false, class: "mt-2 position-static",
            Toast {
                show: show,
                title: "Autohide".to_string(),
                autohide: true,
                delay_ms: 3000,
                color: Some(Color::Success),
                "delay_ms: 3000 — this dismisses itself."
            }
        }
    }
}

#[component]
fn ModalFlagsDemo(show: Signal<bool>) -> Element {
    let mut show = show;
    rsx! {
        Button { color: Color::Danger, outline: true, onclick: move |_| show.set(true), "Open a strict modal" }
        Modal {
            show: show,
            title: "Strict dismissal".to_string(),
            backdrop_close: false,
            keyboard_close: false,
            show_close: false,
            body: rsx! {
                p { "backdrop_close, keyboard_close and show_close are all off." }
                p { class: "mb-0",
                    "Clicking the backdrop does nothing, Escape does nothing, and there is no × in "
                    "the header — the only way out is the button below. This is the shape to use when "
                    "a dialog must be answered rather than dismissed."
                }
            },
            footer: rsx! {
                Button { color: Color::Primary, onclick: move |_| show.set(false), "Acknowledge" }
            },
        }
    }
}
