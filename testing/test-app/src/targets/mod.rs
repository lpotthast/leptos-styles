//! One [`leptos::component`] per browser-test scenario.
//!
//! Each module exports a single self-contained component that owns its signals
//! and renders its assertion target plus any control buttons. To add a new
//! browser-tested behavior, add a module here, expose its component, and
//! include it in `App` in `crate::app`.

mod calc_target;
mod checked_declaration_target;
mod coexist_target;
mod drilling_target;
mod empty_target;
mod fallback_target;
mod hydrate_target;
mod layout_target;
mod lifecycle_target;
mod managed_target;
mod normalized_target;
mod overlap_target;
mod parse_target;
mod quoted_target;
mod reactive_target;
mod shorthand_target;
mod single_target;
mod static_target;
mod transition_target;
mod typed_value_target;
mod units_target;
mod update_target;
mod var_target;

pub use calc_target::CalcTarget;
pub use checked_declaration_target::CheckedDeclarationTarget;
pub use coexist_target::CoexistTarget;
pub use drilling_target::DrillingTarget;
pub use empty_target::EmptyTarget;
pub use fallback_target::FallbackTarget;
pub use hydrate_target::HydrateTarget;
pub use layout_target::LayoutTarget;
pub use lifecycle_target::LifecycleTarget;
pub use managed_target::ManagedTarget;
pub use normalized_target::NormalizedTarget;
pub use overlap_target::OverlapTarget;
pub use parse_target::ParseTarget;
pub use quoted_target::QuotedTarget;
pub use reactive_target::ReactiveTarget;
pub use shorthand_target::ShorthandTarget;
pub use single_target::SingleTarget;
pub use static_target::StaticTarget;
pub use transition_target::TransitionTarget;
pub use typed_value_target::TypedValueTarget;
pub use units_target::UnitsTarget;
pub use update_target::UpdateTarget;
pub use var_target::VarTarget;
