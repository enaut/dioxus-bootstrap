use dioxus::prelude::*;

use crate::types::{Color, Size};

/// Bootstrap FormGroup — label + control wrapper.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="mb-3">
///   <label class="form-label">Email</label>
///   <input type="email" class="form-control" placeholder="you@example.com">
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     FormGroup { label: "Email",
///         Input { r#type: "email", placeholder: "you@example.com" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FormGroupProps {
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Additional CSS classes for the wrapper div.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
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
            ..props.attributes,
            if !props.label.is_empty() {
                label { class: "form-label", "{props.label}" }
            }
            {props.children}
        }
    }
}

/// Bootstrap Input component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<input class="form-control" type="text">` | `Input { r#type: "text" }` |
/// | `<input class="form-control form-control-sm" type="email">` | `Input { r#type: "email", size: Size::Sm }` |
/// | `<input class="form-control" disabled>` | `Input { disabled: true }` |
/// | `<input class="form-control" list="opts">` | `Input { list: "opts" }` |
///
/// Bind a `<datalist>` for autocomplete with `list`, and observe focus with
/// `onfocus` / `onblur`:
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Input { r#type: "text", value: "hello", placeholder: "Enter text" }
///     Input { r#type: "email", size: Size::Sm, oninput: move |evt| { /* handle */ } }
///     Input { r#type: "password", disabled: true }
///     Input {
///         list: "icon-options",
///         onfocus: move |_| { /* open suggestions */ },
///         onblur: move |_| { /* close suggestions */ },
///     }
///     datalist { id: "icon-options",
///         option { value: "star" }
///         option { value: "heart" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
    /// Input type (text, email, password, number, etc.).
    #[props(default = "text".to_string())]
    pub r#type: String,
    /// Current value.
    #[props(default)]
    pub value: String,
    /// When `true`, the `value` attribute is omitted so the field is
    /// *uncontrolled*: the DOM keeps whatever value the user or an external
    /// script writes, instead of Dioxus forcing it back to `value` on every
    /// render. Use for a field another script streams into (e.g. a live
    /// transcript box).
    #[props(default)]
    pub uncontrolled: bool,
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
    /// Minimum value for numeric/date inputs.
    #[props(default)]
    pub min: Option<String>,
    /// Maximum value for numeric/date inputs.
    #[props(default)]
    pub max: Option<String>,
    /// Browser autocomplete hint.
    #[props(default)]
    pub autocomplete: Option<String>,
    /// Safari's `autocorrect` hint (`on` / `off`). Not a `GlobalAttributes`
    /// attribute, so it needs its own prop rather than riding `..attributes` —
    /// the same reason `list` has one.
    #[props(default)]
    pub autocorrect: Option<String>,
    /// Step granularity for numeric and date inputs. The typed sibling of
    /// `min`/`max`, which are already props: a number field that constrains its
    /// range but not its increment is only two-thirds typed.
    #[props(default)]
    pub step: Option<String>,
    /// Which file types a `type="file"` input will accept.
    #[props(default)]
    pub accept: Option<String>,
    /// Datalist id to bind for autocomplete (rendered as the input `list`
    /// attribute). `list` is not a `GlobalAttributes` attribute, so it needs
    /// its own typed prop rather than riding through `..attributes`.
    #[props(default)]
    pub list: Option<String>,
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
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Focus event handler.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    /// Blur event handler.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    /// Key down event handler.
    #[props(default)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// Key up event handler.
    #[props(default)]
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
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
            value: if props.uncontrolled { None } else { Some(props.value.clone()) },
            placeholder: "{props.placeholder}",
            min: props.min.clone(),
            max: props.max.clone(),
            step: props.step.clone(),
            accept: props.accept.clone(),
            autocomplete: props.autocomplete.clone(),
            autocorrect: props.autocorrect.clone(),
            list: props.list.clone(),
            disabled: props.disabled,
            readonly: props.readonly,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            onfocus: move |evt| {
                if let Some(handler) = &props.onfocus {
                    handler.call(evt);
                }
            },
            onblur: move |evt| {
                if let Some(handler) = &props.onblur {
                    handler.call(evt);
                }
            },
            onkeydown: move |evt| {
                if let Some(handler) = &props.onkeydown {
                    handler.call(evt);
                }
            },
            onkeyup: move |evt| {
                if let Some(handler) = &props.onkeyup {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
    }
}

/// Bootstrap Select (dropdown) component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <select class="form-select">
///   <option value="opt1">Option 1</option>
///   <option value="opt2" selected>Option 2</option>
/// </select>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Select { value: "opt2", onchange: move |evt| { /* handle */ },
///         option { value: "opt1", "Option 1" }
///         option { value: "opt2", "Option 2" }
///     }
/// }
/// # }
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
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (option elements).
    pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    use wasm_bindgen::JsCast;

    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" form-select-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("form-select{size_class}")
    } else {
        format!("form-select{size_class} {}", props.class)
    };

    // A `<select>`'s selection is controlled by its `.value` property (or an
    // `<option selected>`), NOT by a `value` content attribute — browsers ignore
    // the latter on a select, so Dioxus's declarative `value` silently does
    // nothing and the element shows its first option. Hold the mounted element
    // and set `.value` imperatively on mount and whenever `value` changes, so the
    // control reflects the value it is given.
    let mut select_el = use_signal(|| None as Option<web_sys::HtmlSelectElement>);
    let value = props.value.clone();
    use_effect(use_reactive!(|value| {
        if let Some(el) = select_el.peek().clone() {
            el.set_value(&value);
        }
    }));

    let mount_value = props.value.clone();
    rsx! {
        select {
            class: "{full_class}",
            disabled: props.disabled,
            onmounted: move |evt: MountedEvent| {
                if let Some(el) = evt
                    .downcast::<web_sys::Element>()
                    .and_then(|e| e.clone().dyn_into::<web_sys::HtmlSelectElement>().ok())
                {
                    el.set_value(&mount_value);
                    select_el.set(Some(el));
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Bootstrap Textarea component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<textarea class="form-control" rows="5">` | `Textarea { rows: 5 }` |
/// | `<textarea class="form-control form-control-sm">` | `Textarea { size: Size::Sm }` |
/// | `<textarea class="form-control" placeholder="..." disabled>` | `Textarea { placeholder: "...", disabled: true }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Textarea { rows: 5, placeholder: "Enter description..." }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct TextareaProps {
    /// Current value.
    #[props(default)]
    pub value: String,
    /// When `true`, the `value` attribute is omitted so the field is
    /// *uncontrolled*: the DOM keeps whatever value the user or an external
    /// script writes, instead of Dioxus forcing it back to `value` on every
    /// render. Use for a field another script streams into (e.g. a live
    /// transcript box).
    #[props(default)]
    pub uncontrolled: bool,
    /// Browser autocomplete hint.
    #[props(default)]
    pub autocomplete: Option<String>,
    /// Safari's `autocorrect` hint (`on` / `off`) — the attribute a compose box
    /// most often needs turned off, and not one `GlobalAttributes` carries.
    #[props(default)]
    pub autocorrect: Option<String>,
    /// Number of visible rows.
    #[props(default = 3)]
    pub rows: u32,
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
    /// Textarea size.
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
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Focus event handler.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    /// Blur event handler.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    /// Key down event handler.
    #[props(default)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// Key up event handler.
    #[props(default)]
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
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
        textarea {
            class: "{full_class}",
            rows: "{props.rows}",
            placeholder: "{props.placeholder}",
            autocomplete: props.autocomplete.clone(),
            autocorrect: props.autocorrect.clone(),
            disabled: props.disabled,
            readonly: props.readonly,
            value: if props.uncontrolled { None } else { Some(props.value.clone()) },
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            onfocus: move |evt| {
                if let Some(handler) = &props.onfocus {
                    handler.call(evt);
                }
            },
            onblur: move |evt| {
                if let Some(handler) = &props.onblur {
                    handler.call(evt);
                }
            },
            onkeydown: move |evt| {
                if let Some(handler) = &props.onkeydown {
                    handler.call(evt);
                }
            },
            onkeyup: move |evt| {
                if let Some(handler) = &props.onkeyup {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
    }
}

/// Bootstrap Checkbox component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="form-check">
///   <input class="form-check-input" type="checkbox" checked>
///   <label class="form-check-label">Accept terms</label>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Checkbox { checked: true, label: "Accept terms",
///         onchange: move |evt| { /* handle */ },
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked.
    #[props(default)]
    pub checked: bool,
    /// Optional id applied to the checkbox input.
    #[props(default)]
    pub input_id: Option<String>,
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Click event handler for the checkbox input.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Additional CSS classes for the wrapper.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-check".to_string()
    } else {
        format!("form-check {}", props.class)
    };
    let label_for = props.input_id.clone().unwrap_or_default();

    rsx! {
            div { class: "{full_class}",
                ..props.attributes,
    input {
    class: "form-check-input",
    r#type: "checkbox",
                    id: props.input_id.unwrap_or_default(),
                    checked: props.checked,
                    disabled: props.disabled,
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    onchange: move |evt| {
                        if let Some(handler) = &props.onchange {
                            handler.call(evt);
                        }
                    },
                }
    if !props.label.is_empty() {
    label { class: "form-check-label", r#for: "{label_for}", "{props.label}" }
    }
            }
        }
}

/// Bootstrap Switch (toggle) component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="form-check form-switch">
///   <input class="form-check-input" type="checkbox" role="switch" checked>
///   <label class="form-check-label">Enable notifications</label>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Switch { checked: true, label: "Enable notifications",
///         onchange: move |evt| { /* handle */ },
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct SwitchProps {
    /// Whether the switch is on.
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
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-check form-switch".to_string()
    } else {
        format!("form-check form-switch {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            input {
                class: "form-check-input",
                r#type: "checkbox",
                role: "switch",
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

/// Bootstrap Range (slider) input.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<input type="range" class="form-range" min="0" max="100">` | `Range { min: "0", max: "100" }` |
/// | `<input type="range" class="form-range" step="5" disabled>` | `Range { step: "5".into(), disabled: true }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Range { value: "50", min: "0", max: "100" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RangeProps {
    /// Current value.
    #[props(default)]
    pub value: String,
    /// Minimum value.
    #[props(default = "0".to_string())]
    pub min: String,
    /// Maximum value.
    #[props(default = "100".to_string())]
    pub max: String,
    /// Step increment.
    #[props(default)]
    pub step: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Range(props: RangeProps) -> Element {
    use wasm_bindgen::JsCast;

    let full_class = if props.class.is_empty() {
        "form-range".to_string()
    } else {
        format!("form-range {}", props.class)
    };

    // A range slider's thumb position is controlled by its `.value` DOM property,
    // NOT by a `value` content attribute (the attribute only seeds the default).
    // Dioxus's declarative `value` sets the attribute, so a server-reported value
    // that differs from the default leaves the thumb at the default — the same
    // property-vs-attribute gap the Select had. Hold the mounted element and set
    // `.value` imperatively on mount and whenever `value` changes.
    let mut range_el = use_signal(|| None as Option<web_sys::HtmlInputElement>);
    let value = props.value.clone();
    use_effect(use_reactive!(|value| {
        if let Some(el) = range_el.peek().clone() {
            el.set_value(&value);
        }
    }));

    let mount_value = props.value.clone();
    rsx! {
        input {
            class: "{full_class}",
            r#type: "range",
            min: "{props.min}",
            max: "{props.max}",
            step: if props.step.is_empty() { None } else { Some(props.step.clone()) },
            disabled: props.disabled,
            onmounted: move |evt: MountedEvent| {
                if let Some(el) = evt
                    .downcast::<web_sys::Element>()
                    .and_then(|e| e.clone().dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    el.set_value(&mount_value);
                    range_el.set(Some(el));
                }
            },
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
    }
}

/// Bootstrap Floating Label wrapper.
///
/// Wraps an Input or Textarea with a floating label that moves
/// above the control when focused or filled.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="form-floating"><input class="form-control" placeholder="..."><label>Email</label></div>` | `FloatingLabel { label: "Email", Input { placeholder: "..." } }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     FloatingLabel { label: "Email address",
///         Input { r#type: "email", placeholder: "name@example.com" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FloatingLabelProps {
    /// Label text.
    pub label: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child element (Input or Textarea).
    pub children: Element,
}

#[component]
pub fn FloatingLabel(props: FloatingLabelProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-floating".to_string()
    } else {
        format!("form-floating {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            {props.children}
            label { "{props.label}" }
        }
    }
}

/// Bootstrap form validation feedback text.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="valid-feedback">Looks good!</div>` | `FormFeedback { valid: true, "Looks good!" }` |
/// | `<div class="invalid-feedback">Required.</div>` | `FormFeedback { "Required." }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Input { class: "is-valid".to_string(), value: "correct" }
///     FormFeedback { valid: true, "Looks good!" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FormFeedbackProps {
    /// True for valid feedback, false for invalid.
    #[props(default)]
    pub valid: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Feedback text.
    pub children: Element,
}

#[component]
pub fn FormFeedback(props: FormFeedbackProps) -> Element {
    let base = if props.valid {
        "valid-feedback"
    } else {
        "invalid-feedback"
    };
    let full_class = if props.class.is_empty() {
        base.to_string()
    } else {
        format!("{base} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap form text (help text below a control).
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="form-text">Must be 8-20 characters.</div>` | `FormText { "Must be 8-20 characters." }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Input { r#type: "password" }
///     FormText { "Must be 8-20 characters long." }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FormTextProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Help text content.
    pub children: Element,
}

#[component]
pub fn FormText(props: FormTextProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-text".to_string()
    } else {
        format!("form-text {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap Radio button component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="form-check">
///   <input class="form-check-input" type="radio" name="color" checked>
///   <label class="form-check-label">Red</label>
/// </div>
/// <div class="form-check">
///   <input class="form-check-input" type="radio" name="color">
///   <label class="form-check-label">Blue</label>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Radio { name: "color", label: "Red", checked: true }
///     Radio { name: "color", label: "Blue" }
/// }
/// # }
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
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
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
            ..props.attributes,
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

/// The class string a `btn-check` toggle's `<label>` carries. Shared by
/// [`CheckboxButton`] and [`RadioButton`] and extracted so it is assertable
/// without rendering: the whole contract of a toggle button is that it emits
/// the same button classes a real [`Button`](crate::button::Button) does, and a
/// drift between the two is exactly what would go unnoticed.
fn toggle_button_label_class(color: Color, outline: bool, size: Size, class: &str) -> String {
    let style = if outline { "btn-outline" } else { "btn" };
    let variant_class = format!(" {style}-{color}");

    let size_class = match size {
        Size::Md => String::new(),
        s => format!(" btn-{s}"),
    };

    if class.is_empty() {
        format!("btn{variant_class}{size_class}")
    } else {
        format!("btn{variant_class}{size_class} {class}")
    }
}

/// Bootstrap checkbox toggle button (`btn-check`).
///
/// Bootstrap 5.3's "Checkbox toggle buttons": a visually hidden checkbox paired
/// with a `<label class="btn …">` whose `for` targets it. The label is what the
/// user sees and clicks; the checkbox holds the state and submits the value.
/// This is a Bootstrap component in its own right, not a styled
/// [`Checkbox`] — the markup, the classes and the CSS that drives them are
/// different.
///
/// The `id` is required rather than optional: the `for`/`id` pair *is* the
/// mechanism. A toggle whose ids do not match renders correctly and does
/// nothing when clicked, which is the worst kind of broken.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<input class="btn-check" type="checkbox" id="c1"><label class="btn btn-primary" for="c1">Mute</label>` | `CheckboxButton { id: "c1", label: "Mute" }` |
/// | `<label class="btn btn-outline-secondary btn-sm" …>` | `CheckboxButton { id: "c1", color: Color::Secondary, outline: true, size: Size::Sm, … }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     CheckboxButton { id: "mute", label: "Mute", checked: true }
///     CheckboxButton { id: "wide", label: "Wide", color: Color::Secondary, outline: true }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CheckboxButtonProps {
    /// The input's id and the label's `for` target. Required: the pair is what
    /// makes the label toggle the input.
    pub id: String,
    /// Optional `name`, for when several toggles submit under one form field.
    #[props(default)]
    pub name: Option<String>,
    /// The value submitted when checked.
    #[props(default)]
    pub value: Option<String>,
    /// Whether the toggle is on.
    #[props(default)]
    pub checked: bool,
    /// Disable the control.
    #[props(default)]
    pub disabled: bool,
    /// The visible text, rendered after any `children`.
    #[props(default)]
    pub label: String,
    /// Rich label content (a leading icon, say), rendered inside the label
    /// before `label`.
    #[props(default)]
    pub children: Element,
    /// Button colour variant.
    #[props(default)]
    pub color: Color,
    /// Use the outline style.
    #[props(default)]
    pub outline: bool,
    /// Button size.
    #[props(default)]
    pub size: Size,
    /// Bootstrap's own examples set `autocomplete="off"` so a browser does not
    /// restore a stale toggle state on reload. Left unset by default so the
    /// rendered attributes match the markup being ported rather than silently
    /// adding one.
    #[props(default)]
    pub autocomplete: Option<String>,
    /// Additional CSS classes, appended to the **label**'s button classes.
    #[props(default)]
    pub class: String,
    /// Change handler, fired on the input.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Any additional HTML attributes, applied to the **input**.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn CheckboxButton(props: CheckboxButtonProps) -> Element {
    let label_class =
        toggle_button_label_class(props.color, props.outline, props.size, &props.class);

    rsx! {
        input {
            class: "btn-check",
            r#type: "checkbox",
            name: props.name,
            id: "{props.id}",
            value: props.value,
            checked: props.checked,
            disabled: props.disabled,
            autocomplete: props.autocomplete,
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
        label { class: "{label_class}", r#for: "{props.id}", {props.children} "{props.label}" }
    }
}

/// Bootstrap radio toggle button (`btn-check`).
///
/// Bootstrap 5.3's "Radio toggle buttons" — the radio sibling of
/// [`CheckboxButton`], and the markup behind a segmented button group: several
/// radios sharing one `name`, each with its own label, wrapped in a
/// [`ButtonGroup`](crate::button::ButtonGroup).
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<input class="btn-check" type="radio" name="view" id="r1"><label class="btn btn-primary" for="r1">List</label>` | `RadioButton { id: "r1", name: "view", label: "List" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     ButtonGroup {
///         RadioButton { id: "v-list", name: "view", value: "list", label: "List", checked: true }
///         RadioButton { id: "v-grid", name: "view", value: "grid", label: "Grid" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RadioButtonProps {
    /// The input's id and the label's `for` target. Required, as for
    /// [`CheckboxButton`].
    pub id: String,
    /// The radio group name. Radios sharing a `name` are mutually exclusive —
    /// which is the whole point of a radio, so this is where a segmented
    /// control is actually defined.
    #[props(default)]
    pub name: Option<String>,
    /// The value submitted when this option is selected.
    #[props(default)]
    pub value: Option<String>,
    /// Whether this option is selected.
    #[props(default)]
    pub checked: bool,
    /// Disable the control.
    #[props(default)]
    pub disabled: bool,
    /// The visible text, rendered after any `children`.
    #[props(default)]
    pub label: String,
    /// Rich label content, rendered inside the label before `label`.
    #[props(default)]
    pub children: Element,
    /// Button colour variant.
    #[props(default)]
    pub color: Color,
    /// Use the outline style.
    #[props(default)]
    pub outline: bool,
    /// Button size.
    #[props(default)]
    pub size: Size,
    /// See [`CheckboxButtonProps::autocomplete`].
    #[props(default)]
    pub autocomplete: Option<String>,
    /// Additional CSS classes, appended to the **label**'s button classes.
    #[props(default)]
    pub class: String,
    /// Change handler, fired on the input.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Any additional HTML attributes, applied to the **input**.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn RadioButton(props: RadioButtonProps) -> Element {
    let label_class =
        toggle_button_label_class(props.color, props.outline, props.size, &props.class);

    rsx! {
        input {
            class: "btn-check",
            r#type: "radio",
            name: props.name,
            id: "{props.id}",
            value: props.value,
            checked: props.checked,
            disabled: props.disabled,
            autocomplete: props.autocomplete,
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
        label { class: "{label_class}", r#for: "{props.id}", {props.children} "{props.label}" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_button_label_matches_a_plain_button() {
        // The contract: a toggle's label carries the same classes the equivalent
        // Button emits. If Button's composition changes and this does not, a
        // toggle and a button styled identically stop looking identical.
        assert_eq!(
            toggle_button_label_class(Color::Primary, false, Size::Md, ""),
            "btn btn-primary"
        );
    }

    #[test]
    fn toggle_button_label_outline_and_size() {
        assert_eq!(
            toggle_button_label_class(Color::Secondary, true, Size::Sm, ""),
            "btn btn-outline-secondary btn-sm"
        );
    }

    #[test]
    fn toggle_button_label_appends_extra_classes_last() {
        assert_eq!(
            toggle_button_label_class(Color::Danger, false, Size::Lg, "w-100"),
            "btn btn-danger btn-lg w-100"
        );
    }

    #[test]
    fn toggle_button_medium_size_adds_no_class() {
        // Bootstrap has no `btn-md`; the default size is the absence of a class.
        assert!(!toggle_button_label_class(Color::Primary, false, Size::Md, "").contains("btn-md"));
    }
}
