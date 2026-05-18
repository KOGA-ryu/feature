use feature_registry::RegisteredFeature;
use wiki_browser::WikiBrowserEntry;

use super::{FeatureCounts, FeatureLabApp, KindFilter};

impl FeatureLabApp {
    pub(crate) fn selected_feature(&self) -> Option<RegisteredFeature> {
        self.browser
            .selected_feature_id()
            .and_then(|feature_id| self.registry.get(feature_id))
            .cloned()
    }

    pub(crate) fn selected_browser_entry(&self) -> Option<&WikiBrowserEntry> {
        self.browser.selected_entry()
    }

    pub(crate) fn selected_feature_id(&self) -> Option<&str> {
        self.browser.selected_feature_id()
    }

    pub(crate) fn visible_browser_entries(&self) -> Vec<&WikiBrowserEntry> {
        self.browser.visible_entries()
    }

    pub(crate) fn visible_feature_count(&self) -> usize {
        self.browser.visible_entries().len()
    }

    pub(crate) fn feature_counts(&self) -> FeatureCounts {
        count_entries(self.browser.entries().iter())
    }

    pub(crate) fn visible_feature_counts(&self) -> FeatureCounts {
        let visible = self.browser.visible_entries();
        count_entries(visible.into_iter())
    }

    pub(crate) fn kind_filter(&self) -> KindFilter {
        KindFilter::from_browser_kind(self.browser.kind_filter())
    }

    pub(crate) fn set_kind_filter(&mut self, kind_filter: KindFilter) {
        self.browser
            .set_kind_filter(kind_filter.into_browser_kind());
    }

    pub(crate) fn select_feature(&mut self, feature_id: String) {
        self.browser.select_feature(&feature_id);
    }
}

fn count_entries<'a>(entries: impl IntoIterator<Item = &'a WikiBrowserEntry>) -> FeatureCounts {
    let mut counts = FeatureCounts::default();
    for entry in entries {
        counts.total += 1;
        if entry.manifest.id.starts_with("ui.") {
            counts.ui += 1;
        } else if entry.manifest.id.starts_with("logic.") {
            counts.logic += 1;
        } else if entry.manifest.id.starts_with("sim.") {
            counts.sim += 1;
        } else if entry.manifest.id.starts_with("workflow.") {
            counts.workflow += 1;
        }
    }
    counts
}
