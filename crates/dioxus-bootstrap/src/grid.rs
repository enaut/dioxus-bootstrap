use dioxus::prelude::*;

use crate::types::ColumnSize;

/// Bootstrap Container component.
///
/// ```rust
/// rsx! {
///     Container { "Fixed width content" }
///     Container { fluid: true, "Full width content" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ContainerProps {
    /// Use container-fluid for full width.
    #[props(default)]
    pub fluid: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let base = if props.fluid { "container-fluid" } else { "container" };
    let full_class = if props.class.is_empty() {
        base.to_string()
    } else {
        format!("{base} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", {props.children} }
    }
}

/// Bootstrap Row component.
///
/// ```rust
/// rsx! {
///     Row { class: "g-3",
///         Col { lg: ColumnSize::Span(6), "Left" }
///         Col { lg: ColumnSize::Span(6), "Right" }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RowProps {
    /// Additional CSS classes (e.g., "g-3", "align-items-center").
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let full_class = if props.class.is_empty() {
        "row".to_string()
    } else {
        format!("row {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", {props.children} }
    }
}

/// Bootstrap Col (column) component with responsive breakpoint props.
///
/// ```rust
/// rsx! {
///     Col { xs: ColumnSize::Span(12), md: ColumnSize::Span(6), lg: ColumnSize::Span(4),
///         "Responsive column"
///     }
///     Col { lg: ColumnSize::Auto, "Auto-width column" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ColProps {
    /// Column size at xs breakpoint (default, no breakpoint prefix).
    #[props(default)]
    pub xs: Option<ColumnSize>,
    /// Column size at sm breakpoint.
    #[props(default)]
    pub sm: Option<ColumnSize>,
    /// Column size at md breakpoint.
    #[props(default)]
    pub md: Option<ColumnSize>,
    /// Column size at lg breakpoint.
    #[props(default)]
    pub lg: Option<ColumnSize>,
    /// Column size at xl breakpoint.
    #[props(default)]
    pub xl: Option<ColumnSize>,
    /// Column size at xxl breakpoint.
    #[props(default)]
    pub xxl: Option<ColumnSize>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    let mut classes = Vec::new();

    if let Some(size) = &props.xs {
        classes.push(format!("col-{size}"));
    }
    if let Some(size) = &props.sm {
        classes.push(format!("col-sm-{size}"));
    }
    if let Some(size) = &props.md {
        classes.push(format!("col-md-{size}"));
    }
    if let Some(size) = &props.lg {
        classes.push(format!("col-lg-{size}"));
    }
    if let Some(size) = &props.xl {
        classes.push(format!("col-xl-{size}"));
    }
    if let Some(size) = &props.xxl {
        classes.push(format!("col-xxl-{size}"));
    }

    // Default to "col" if no breakpoints specified
    if classes.is_empty() {
        classes.push("col".to_string());
    }

    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let full_class = classes.join(" ");

    rsx! {
        div { class: "{full_class}", {props.children} }
    }
}
