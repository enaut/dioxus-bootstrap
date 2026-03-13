pub mod alert;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod collapse;
pub mod dropdown;
pub mod form;
pub mod grid;
pub mod head;
pub mod icon;
pub mod list_group;
pub mod modal;
pub mod nav;
pub mod progress;
pub mod spinner;
pub mod table;
pub mod tabs;
pub mod types;

/// Prelude — import everything with `use dioxus_bootstrap::prelude::*`.
pub mod prelude {
    pub use crate::alert::*;
    pub use crate::badge::*;
    pub use crate::breadcrumb::*;
    pub use crate::button::*;
    pub use crate::card::*;
    pub use crate::collapse::*;
    pub use crate::dropdown::*;
    pub use crate::form::*;
    pub use crate::grid::*;
    pub use crate::head::*;
    pub use crate::icon::*;
    pub use crate::list_group::*;
    pub use crate::modal::*;
    pub use crate::nav::*;
    pub use crate::progress::*;
    pub use crate::spinner::*;
    pub use crate::table::*;
    pub use crate::tabs::*;
    pub use crate::types::*;
}
