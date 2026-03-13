use dioxus::prelude::*;

use crate::types::{Color, Size};

/// Bootstrap Button component.
///
/// ```rust
/// rsx! {
///     Button { color: Color::Primary, "Click me" }
///     Button { color: Color::Danger, outline: true, size: Size::Sm, "Delete" }
///     Button { color: Color::Success, disabled: true, "Saved" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// Button color variant.
    #[props(default)]
    pub color: Color,
    /// Use outline style instead of filled.
    #[props(default)]
    pub outline: bool,
    /// Button size.
    #[props(default)]
    pub size: Size,
    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: bool,
    /// HTML button type attribute.
    #[props(default = "button".to_string())]
    pub r#type: String,
    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let style = if props.outline { "btn-outline" } else { "btn" };
    let color = props.color;
    let color_class = format!("{style}-{color}");

    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" btn-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("btn {color_class}{size_class}")
    } else {
        format!("btn {color_class}{size_class} {}", props.class)
    };

    rsx! {
        button {
            class: "{full_class}",
            r#type: "{props.r#type}",
            disabled: props.disabled,
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

/// Bootstrap ButtonGroup component.
///
/// ```rust
/// rsx! {
///     ButtonGroup {
///         Button { color: Color::Primary, "Left" }
///         Button { color: Color::Primary, "Middle" }
///         Button { color: Color::Primary, "Right" }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ButtonGroupProps {
    /// Button group size.
    #[props(default)]
    pub size: Size,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (buttons).
    pub children: Element,
}

#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" btn-group-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("btn-group{size_class}")
    } else {
        format!("btn-group{size_class} {}", props.class)
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "group",
            {props.children}
        }
    }
}
