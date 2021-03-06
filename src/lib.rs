#![allow(clippy::clippy::new_without_default)]

pub use druid::{kurbo, piet};

pub mod app;
pub mod context;
pub mod id;
pub mod key;
pub mod object;
pub mod state;
pub mod tree;
pub mod ui;
pub mod widgets;

pub mod bloom;
pub mod text;

pub mod event {
    pub use druid::{Event, LifeCycle};
    pub use druid::{MouseButton, MouseButtons, MouseEvent};
}

pub use druid::BoxConstraints;

pub trait VisualEq {
    /// Determine whether two values are the same.
    ///
    /// This is intended to always be a fast operation. If it returns
    /// `true`, the two values *must* be equal, but two equal values
    /// need not be considered the same here, as will often be the
    /// case when two copies are separately allocated.
    ///
    /// Note that "equal" above has a slightly different meaning than
    /// `PartialEq`, for example two floating point NaN values should
    /// be considered equal when they have the same bit representation.
    fn eq(&self, other: &Self) -> bool;
}
