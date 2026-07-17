use smallvec::SmallVec;

use crate::StyleEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StylePriority {
    Normal,
    Fallback(u16),
}

impl StylePriority {
    pub(crate) const fn group(self) -> u16 {
        match self {
            Self::Normal => 0,
            Self::Fallback(group) => group,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct StyleListEntry {
    pub(crate) style: StyleEntry,
    pub(crate) priority: StylePriority,
}

/// Internal wrapper around `SmallVec` that provides duplicate detection in debug builds.
///
/// Uses `SmallVec` to avoid heap allocation for the common case of ≤4 style entries. The
/// `has_reactive` flag is maintained at insertion time so [`Self::is_reactive`] is O(1).
#[derive(Clone, Debug, Default)]
pub(crate) struct StyleList {
    entries: SmallVec<[StyleListEntry; 4]>,
    has_reactive: bool,
}

impl StyleList {
    pub(crate) fn push(&mut self, style: StyleEntry) {
        #[cfg(debug_assertions)]
        {
            let duplicate_property = style.static_property_name().filter(|property| {
                self.entries.iter().any(|entry| {
                    entry.priority == StylePriority::Normal
                        && entry.style.static_property_name() == Some(*property)
                })
            });
            if let Some(property) = duplicate_property {
                let backtrace = std::backtrace::Backtrace::force_capture();
                tracing::warn!(
                    "Duplicate style property '{property}' added to Styles. This may indicate a bug. At: {backtrace}"
                );
            }
        }
        self.has_reactive |= style.is_reactive();
        self.entries.push(StyleListEntry {
            style,
            priority: StylePriority::Normal,
        });
    }

    pub(crate) fn push_fallback(&mut self, style: StyleEntry, group: u16) {
        self.has_reactive |= style.is_reactive();
        self.entries.push(StyleListEntry {
            style,
            priority: StylePriority::Fallback(group),
        });
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &StyleEntry> {
        self.entries.iter().map(|entry| &entry.style)
    }

    pub(crate) fn iter_entries(&self) -> impl Iterator<Item = (&StyleEntry, StylePriority)> {
        self.entries
            .iter()
            .map(|entry| (&entry.style, entry.priority))
    }

    pub(crate) fn into_entries(self) -> impl Iterator<Item = StyleListEntry> {
        self.entries.into_iter()
    }

    pub(crate) fn next_fallback_group(&self) -> u16 {
        self.entries
            .iter()
            .filter_map(|entry| match entry.priority {
                StylePriority::Normal => None,
                StylePriority::Fallback(group) => Some(group),
            })
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }

    pub(crate) fn touch_reactive_dependencies(&self) {
        for entry in self.iter() {
            entry.touch_reactive_dependencies();
        }
    }

    pub(crate) const fn is_reactive(&self) -> bool {
        self.has_reactive
    }
}
