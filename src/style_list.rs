use smallvec::SmallVec;

use crate::StyleEntry;

/// Internal wrapper around `SmallVec<[StyleEntry; 4]>` that provides duplicate detection in debug builds.
///
/// Uses `SmallVec` to avoid heap allocation for the common case of ≤4 style entries.
///
/// Note: The TypedBuilder derive exposes this type in generated builder signatures.
#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct StyleList(SmallVec<[StyleEntry; 4]>);

impl StyleList {
    pub(crate) fn push(&mut self, style: StyleEntry) {
        #[cfg(debug_assertions)]
        {
            if self.0.iter().any(|it| it.property() == style.property()) {
                let backtrace = std::backtrace::Backtrace::force_capture();
                tracing::warn!(
                    "Duplicate style property '{}' added to Styles. This may indicate a bug. At: {backtrace}",
                    style.property()
                );
            }
        }
        self.0.push(style);
    }

    pub(crate) fn push_fallback(&mut self, style: StyleEntry) {
        self.0.push(style);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &StyleEntry> {
        self.0.iter()
    }

    pub(crate) fn into_entries(self) -> impl Iterator<Item = StyleEntry> {
        self.0.into_iter()
    }

    pub(crate) fn find_by_property(&self, property: &str) -> Option<&StyleEntry> {
        self.0.iter().find(|entry| entry.property() == property)
    }

    pub(crate) fn touch_reactive_dependencies(&self) {
        for entry in self.iter() {
            entry.touch_reactive_dependencies();
        }
    }
}
