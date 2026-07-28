use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap ListGroup component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <ul class="list-group list-group-flush">
///   <li class="list-group-item active">Active</li>
///   <li class="list-group-item">Normal</li>
///   <li class="list-group-item list-group-item-danger">Danger</li>
///   <li class="list-group-item disabled">Disabled</li>
/// </ul>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// # let handler = move |_: MouseEvent| {};
/// rsx! {
///     ListGroup { flush: true,
///         ListGroupItem { active: true, "Active" }
///         ListGroupItem { "Normal" }
///         ListGroupItem { color: Color::Danger, "Danger" }
///         ListGroupItem { disabled: true, "Disabled" }
///     }
///     // Clickable list group
///     ListGroup {
///         ListGroupItem { onclick: handler, "Click me" }
///     }
///     // Numbered list
///     ListGroup { numbered: true,
///         ListGroupItem { "First" }
///         ListGroupItem { "Second" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ListGroupProps {
    /// Remove borders and rounded corners for use inside cards.
    #[props(default)]
    pub flush: bool,
    /// Use numbered list style.
    #[props(default)]
    pub numbered: bool,
    /// Container element. Empty (the default) renders `<ul>`, or `<ol>` when
    /// `numbered`; `"div"` renders Bootstrap's generic `<div class="list-group">`
    /// form, which is what a list of links or buttons requires — `<a>` and
    /// `<button>` are not valid children of `<ul>`.
    #[props(default)]
    pub tag: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (ListGroupItem components).
    pub children: Element,
}

#[component]
pub fn ListGroup(props: ListGroupProps) -> Element {
    let mut classes = vec!["list-group".to_string()];
    if props.flush {
        classes.push("list-group-flush".to_string());
    }
    if props.numbered {
        classes.push("list-group-numbered".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    // `div` first: the generic form is the one that can hold `<a>`/`<button>`
    // children, so it must win over the numbered `<ol>` rather than be
    // unreachable behind it.
    if props.tag == "div" {
        rsx! {
            div { class: "{full_class}", ..props.attributes, {props.children} }
        }
    } else if props.numbered {
        rsx! {
            ol { class: "{full_class}", ..props.attributes, {props.children} }
        }
    } else {
        rsx! {
            ul { class: "{full_class}", ..props.attributes, {props.children} }
        }
    }
}

/// Bootstrap ListGroupItem component.
#[derive(Clone, PartialEq, Props)]
pub struct ListGroupItemProps {
    /// Active state.
    #[props(default)]
    pub active: bool,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Item color variant.
    #[props(default)]
    pub color: Option<Color>,
    /// Click event handler. With the default `tag`, this renders the item as a
    /// `<button>` — Bootstrap's actionable form.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Item element. Empty (the default) renders `<li>`, or `<button>` when
    /// `onclick` is set; `"div"` renders `<div class="list-group-item">`, the
    /// generic form.
    ///
    /// `"div"` is honoured **even with `onclick`**. A `<button>` may not contain
    /// interactive descendants, so a clickable row that holds its own input or
    /// buttons cannot be one — the browser's tag-soup recovery reparents them and
    /// the controls stop working. Such a row is an ordinary element carrying an
    /// ordinary handler, which is what the escape hatches are for; it still gets
    /// `list-group-item-action`, because that class is what Bootstrap defines for
    /// the look, and the caller has said this row is actionable by handing it a
    /// handler.
    #[props(default)]
    pub tag: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

/// Which element a list-group item renders as, given its `tag` and whether a
/// click handler was supplied. `tag` wins: see the note on the prop.
fn list_group_item_element(tag: &str, has_onclick: bool) -> &'static str {
    if tag == "div" {
        "div"
    } else if has_onclick {
        "button"
    } else {
        "li"
    }
}

#[component]
pub fn ListGroupItem(props: ListGroupItemProps) -> Element {
    let mut classes = vec!["list-group-item".to_string()];
    if props.active {
        classes.push("active".to_string());
    }
    if props.disabled {
        classes.push("disabled".to_string());
    }
    if let Some(ref c) = props.color {
        classes.push(format!("list-group-item-{c}"));
    }
    if props.onclick.is_some() {
        classes.push("list-group-item-action".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    // The element is chosen by one function, which the tests exercise directly —
    // `tag` wins over `onclick`, for the reason on the prop.
    match list_group_item_element(&props.tag, props.onclick.is_some()) {
        "div" => {
            let handler = props.onclick;
            rsx! {
                div {
                    class: "{full_class}",
                    onclick: move |evt| {
                        if let Some(handler) = &handler {
                            handler.call(evt);
                        }
                    },
                    ..props.attributes,
                    {props.children}
                }
            }
        }
        "button" => {
            let handler = props.onclick.expect("button form implies a handler");
            rsx! {
                button {
                    class: "{full_class}",
                    r#type: "button",
                    disabled: props.disabled,
                    onclick: move |evt| handler.call(evt),
                    ..props.attributes,
                    {props.children}
                }
            }
        }
        _ => rsx! {
            li { class: "{full_class}", ..props.attributes, {props.children} }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_defaults_to_li() {
        assert_eq!(list_group_item_element("", false), "li");
    }

    #[test]
    fn a_handler_alone_makes_it_a_button() {
        // Bootstrap's actionable item is an `<a>` or a `<button>`, and with no
        // element named this is the one to pick.
        assert_eq!(list_group_item_element("", true), "button");
    }

    #[test]
    fn an_explicit_div_wins_over_the_handler() {
        // The whole point of the prop. A `<button>` may not contain interactive
        // descendants, so a clickable row holding its own input or buttons has to
        // stay a `<div>` — if the handler silently overrode `tag` here, the markup
        // would be invalid and the nested controls would stop working, while every
        // class-level check still passed.
        assert_eq!(list_group_item_element("div", true), "div");
        assert_eq!(list_group_item_element("div", false), "div");
    }
}
