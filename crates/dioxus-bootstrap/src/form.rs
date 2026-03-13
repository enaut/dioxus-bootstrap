use dioxus::prelude::*;

use crate::types::Size;

/// Bootstrap FormGroup — label + control wrapper with spacing.
///
/// ```rust
/// rsx! {
///     FormGroup { label: "Email",
///         Input { r#type: "email", placeholder: "you@example.com" }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FormGroupProps {
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Additional CSS classes for the wrapper div.
    #[props(default)]
    pub class: String,
    /// Child elements (form control).
    pub children: Element,
}

#[component]
pub fn FormGroup(props: FormGroupProps) -> Element {
    let full_class = if props.class.is_empty() {
        "mb-3".to_string()
    } else {
        format!("mb-3 {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            if !props.label.is_empty() {
                label { class: "form-label", "{props.label}" }
            }
            {props.children}
        }
    }
}

/// Bootstrap Input component.
///
/// ```rust
/// rsx! {
///     Input { r#type: "text", value: "hello", placeholder: "Enter text" }
///     Input { r#type: "password", size: Size::Sm }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
    /// Input type (text, email, password, number, etc.).
    #[props(default = "text".to_string())]
    pub r#type: String,
    /// Current value.
    #[props(default)]
    pub value: String,
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
    /// Input size.
    #[props(default)]
    pub size: Size,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Readonly state.
    #[props(default)]
    pub readonly: bool,
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" form-control-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("form-control{size_class}")
    } else {
        format!("form-control{size_class} {}", props.class)
    };

    rsx! {
        input {
            class: "{full_class}",
            r#type: "{props.r#type}",
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            readonly: props.readonly,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
        }
    }
}

/// Bootstrap Select component.
///
/// ```rust
/// rsx! {
///     Select { value: "opt1",
///         option { value: "opt1", "Option 1" }
///         option { value: "opt2", "Option 2" }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
    /// Current selected value.
    #[props(default)]
    pub value: String,
    /// Select size.
    #[props(default)]
    pub size: Size,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (option elements).
    pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" form-select-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("form-select{size_class}")
    } else {
        format!("form-select{size_class} {}", props.class)
    };

    rsx! {
        select {
            class: "{full_class}",
            value: "{props.value}",
            disabled: props.disabled,
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

/// Bootstrap Textarea component.
///
/// ```rust
/// rsx! {
///     Textarea { rows: 5, placeholder: "Enter description..." }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct TextareaProps {
    /// Current value.
    #[props(default)]
    pub value: String,
    /// Number of visible rows.
    #[props(default = 3)]
    pub rows: u32,
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Readonly state.
    #[props(default)]
    pub readonly: bool,
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-control".to_string()
    } else {
        format!("form-control {}", props.class)
    };

    rsx! {
        textarea {
            class: "{full_class}",
            rows: "{props.rows}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            readonly: props.readonly,
            value: "{props.value}",
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
        }
    }
}

/// Bootstrap Checkbox component.
///
/// ```rust
/// rsx! {
///     Checkbox { checked: true, label: "Accept terms" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked.
    #[props(default)]
    pub checked: bool,
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes for the wrapper.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-check".to_string()
    } else {
        format!("form-check {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            input {
                class: "form-check-input",
                r#type: "checkbox",
                checked: props.checked,
                disabled: props.disabled,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                },
            }
            if !props.label.is_empty() {
                label { class: "form-check-label", "{props.label}" }
            }
        }
    }
}

/// Bootstrap Radio component.
///
/// ```rust
/// rsx! {
///     Radio { name: "color", label: "Red", checked: true }
///     Radio { name: "color", label: "Blue" }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RadioProps {
    /// Radio group name.
    pub name: String,
    /// Whether the radio is checked.
    #[props(default)]
    pub checked: bool,
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes for the wrapper.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-check".to_string()
    } else {
        format!("form-check {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            input {
                class: "form-check-input",
                r#type: "radio",
                name: "{props.name}",
                checked: props.checked,
                disabled: props.disabled,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                },
            }
            if !props.label.is_empty() {
                label { class: "form-check-label", "{props.label}" }
            }
        }
    }
}
