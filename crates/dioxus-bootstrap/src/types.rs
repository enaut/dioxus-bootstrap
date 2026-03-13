use std::fmt;

/// Bootstrap contextual color variants.
///
/// Maps to Bootstrap's color classes: `primary`, `secondary`, `success`,
/// `danger`, `warning`, `info`, `light`, `dark`.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML class | Dioxus |
/// |---|---|
/// | `btn-primary` | `Button { color: Color::Primary }` |
/// | `alert-danger` | `Alert { color: Color::Danger }` |
/// | `text-bg-success` | `Badge { color: Color::Success }` |
/// | `bg-warning` | `Toast { color: Color::Warning }` |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Primary => write!(f, "primary"),
            Color::Secondary => write!(f, "secondary"),
            Color::Success => write!(f, "success"),
            Color::Danger => write!(f, "danger"),
            Color::Warning => write!(f, "warning"),
            Color::Info => write!(f, "info"),
            Color::Light => write!(f, "light"),
            Color::Dark => write!(f, "dark"),
        }
    }
}

/// Bootstrap component size variants.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML class | Dioxus |
/// |---|---|
/// | `btn-sm` | `Button { size: Size::Sm }` |
/// | `btn-lg` | `Button { size: Size::Lg }` |
/// | `form-control-sm` | `Input { size: Size::Sm }` |
/// | `pagination-lg` | `Pagination { size: Size::Lg }` |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Size {
    Sm,
    #[default]
    Md,
    Lg,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Sm => write!(f, "sm"),
            Size::Md => write!(f, "md"),
            Size::Lg => write!(f, "lg"),
        }
    }
}

/// Bootstrap column span (1–12 or auto).
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML class | Dioxus |
/// |---|---|
/// | `col-6` | `Col { xs: ColumnSize::Span(6) }` |
/// | `col-md-4` | `Col { md: ColumnSize::Span(4) }` |
/// | `col-auto` | `Col { xs: ColumnSize::Auto }` |
/// | `col-lg-auto` | `Col { lg: ColumnSize::Auto }` |
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnSize {
    Auto,
    Span(u8),
}

impl fmt::Display for ColumnSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnSize::Auto => write!(f, "auto"),
            ColumnSize::Span(n) => write!(f, "{n}"),
        }
    }
}

/// Bootstrap navbar responsive expand breakpoints.
///
/// Controls when the navbar switches from collapsed (hamburger) to expanded (horizontal).
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML class | Dioxus |
/// |---|---|
/// | `navbar-expand-lg` | `Navbar { expand: NavbarExpand::Lg }` |
/// | `navbar-expand` | `Navbar { expand: NavbarExpand::Always }` |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum NavbarExpand {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xxl,
    Always,
}

impl fmt::Display for NavbarExpand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NavbarExpand::Sm => write!(f, "navbar-expand-sm"),
            NavbarExpand::Md => write!(f, "navbar-expand-md"),
            NavbarExpand::Lg => write!(f, "navbar-expand-lg"),
            NavbarExpand::Xl => write!(f, "navbar-expand-xl"),
            NavbarExpand::Xxl => write!(f, "navbar-expand-xxl"),
            NavbarExpand::Always => write!(f, "navbar-expand"),
        }
    }
}

/// Bootstrap modal size.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML class | Dioxus |
/// |---|---|
/// | `modal-sm` | `Modal { size: ModalSize::Sm }` |
/// | (default) | `Modal { size: ModalSize::Default }` |
/// | `modal-lg` | `Modal { size: ModalSize::Lg }` |
/// | `modal-xl` | `Modal { size: ModalSize::Xl }` |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ModalSize {
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

/// Spinner animation style.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML class | Dioxus |
/// |---|---|
/// | `spinner-border` | `Spinner { style: SpinnerStyle::Border }` |
/// | `spinner-grow` | `Spinner { style: SpinnerStyle::Grow }` |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SpinnerStyle {
    #[default]
    Border,
    Grow,
}
