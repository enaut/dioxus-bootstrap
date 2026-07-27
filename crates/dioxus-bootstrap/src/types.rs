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

/// How a badge takes its colour.
///
/// Bootstrap 5.3 offers three distinct colour idioms for a badge, and they are
/// not interchangeable. `text-bg-*` sets a background **and** a contrasting
/// foreground; `bg-*` sets only the background and lets the text colour be
/// inherited; the subtle pair is a background and text colour that are designed
/// together. Without this choice a caller can only get the first, and reaching
/// for the others means hand-writing the classes the component exists to type.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML class | Dioxus |
/// |---|---|
/// | `badge text-bg-primary` | `Badge { color: Color::Primary }` |
/// | `badge bg-secondary` | `Badge { color: Color::Secondary, fill: BadgeFill::Bg }` |
/// | `badge bg-info-subtle text-info-emphasis` | `Badge { color: Color::Info, fill: BadgeFill::Subtle }` |
/// | `badge` | `Badge { fill: BadgeFill::None }` |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BadgeFill {
    /// `text-bg-<color>` — background plus contrasting text. Bootstrap's badge
    /// helper, and the default, so omitting the prop changes nothing.
    #[default]
    TextBg,
    /// `bg-<color>` — the background utility alone, text colour inherited.
    Bg,
    /// `bg-<color>-subtle text-<color>-emphasis` — Bootstrap's low-contrast
    /// pair. Both classes together: the subtle background is near-invisible
    /// without its matching emphasis foreground, so this is one idiom rather
    /// than two utilities a caller should have to remember to combine.
    Subtle,
    /// No colour idiom — a bare `badge`, the chip geometry only. Use this when
    /// painting the background yourself: `text-bg-*` also sets a foreground, so
    /// an inline background override would otherwise inherit a text colour it
    /// never asked for.
    None,
}

/// The container a navbar wraps its contents in.
///
/// Bootstrap's navbar examples put the brand and links inside a `.container` or
/// `.container-fluid`; the gutter belongs to that container, not to `<nav>`.
/// A navbar that supplies its own padding needs to be able to omit it.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="container-fluid">` | `Navbar { container: NavbarContainer::Fluid }` |
/// | `<div class="container">` | `Navbar { container: NavbarContainer::Fixed }` |
/// | (no wrapper) | `Navbar { container: NavbarContainer::None }` |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum NavbarContainer {
    /// `container-fluid` — full width with gutters. Bootstrap's own default and
    /// this component's, so omitting the prop is unchanged behaviour.
    #[default]
    Fluid,
    /// `container` — the responsive fixed-width container.
    Fixed,
    /// No container element at all: brand and children are direct children of
    /// `<nav>`.
    None,
}

impl NavbarContainer {
    /// The container class, or `None` when no wrapper element should be
    /// emitted. Returning an `Option` rather than an empty string keeps
    /// "no class" and "no element" from being the same answer — an empty
    /// `<div class="">` is still a box in the layout.
    pub fn class(&self) -> Option<&'static str> {
        match self {
            NavbarContainer::Fluid => Some("container-fluid"),
            NavbarContainer::Fixed => Some("container"),
            NavbarContainer::None => None,
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
