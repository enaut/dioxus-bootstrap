use dioxus::prelude::*;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_ICONS_CSS: Asset = asset!("/assets/bootstrap-icons.min.css");

/// Loads Bootstrap 5.3.3 CSS and Bootstrap Icons as bundled static assets.
///
/// Place this at the top of your app, before any Bootstrap components.
///
/// ```rust
/// rsx! {
///     BootstrapHead {}
///     // your app content
/// }
/// ```
#[component]
pub fn BootstrapHead() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_CSS }
    }
}
