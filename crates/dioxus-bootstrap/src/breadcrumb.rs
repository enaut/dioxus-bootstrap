use dioxus::prelude::*;

/// Bootstrap Breadcrumb component.
///
/// ```rust
/// rsx! {
///     Breadcrumb {
///         BreadcrumbItem { href: "/", "Home" }
///         BreadcrumbItem { href: "/products", "Products" }
///         BreadcrumbItem { active: true, "Current Page" }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (BreadcrumbItem components).
    pub children: Element,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let full_class = if props.class.is_empty() {
        "breadcrumb".to_string()
    } else {
        format!("breadcrumb {}", props.class)
    };

    rsx! {
        nav { "aria-label": "breadcrumb",
            ol { class: "{full_class}", {props.children} }
        }
    }
}

/// A single item in a Breadcrumb.
#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbItemProps {
    /// Link href. Not rendered if active.
    #[props(default)]
    pub href: String,
    /// Active (current page) state — renders as plain text instead of link.
    #[props(default)]
    pub active: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let mut classes = vec!["breadcrumb-item".to_string()];
    if props.active {
        classes.push("active".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    if props.active {
        rsx! {
            li {
                class: "{full_class}",
                "aria-current": "page",
                {props.children}
            }
        }
    } else {
        rsx! {
            li { class: "{full_class}",
                a { href: "{props.href}", {props.children} }
            }
        }
    }
}
