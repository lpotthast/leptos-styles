//! One [`leptos::component`] per browser-test scenario.
//!
//! Each module exports a single self-contained component that owns its signals
//! and renders its assertion target plus any control buttons. To add a new
//! browser-tested behavior, add a module here, expose its component, and
//! include it in `App` in `crate::app`.

mod calc_target;
mod checked_declaration_target;
mod drilling_target;
mod empty_target;
mod hydrate_target;
mod managed_target;
mod merge_fallback_target;
mod normalized_target;
mod parse_css_target;
mod reactive_target;
mod static_target;
mod transition_target;
mod typed_value_target;
mod units_target;
mod var_target;

pub use calc_target::CalcTarget;
pub use checked_declaration_target::CheckedDeclarationTarget;
pub use drilling_target::DrillingTarget;
pub use empty_target::EmptyTarget;
pub use hydrate_target::HydrateTarget;
pub use managed_target::ManagedTarget;
pub use merge_fallback_target::MergeFallbackTarget;
pub use normalized_target::NormalizedTarget;
pub use parse_css_target::ParseCssTarget;
pub use reactive_target::ReactiveTarget;
pub use static_target::StaticTarget;
pub use transition_target::TransitionTarget;
pub use typed_value_target::TypedValueTarget;
pub use units_target::UnitsTarget;
pub use var_target::VarTarget;
