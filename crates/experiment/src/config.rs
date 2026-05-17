use std::sync::Arc;

use pipeline::filter::GameFilter;
use pipeline::feature::FeatureBuilder;
use pipeline::score::Scorer;
use pipeline::select::Selector;
use pipeline::label::Labeler;

pub struct ExperimentConfig<F> {
    pub filter: Arc<dyn GameFilter + Send + Sync>,
    pub feature_builder: Arc<dyn FeatureBuilder<Output = F> + Send + Sync>,
    pub scorer: Arc<dyn Scorer<F> + Send + Sync>,
    pub selector: Arc<dyn Selector + Send + Sync>,
    pub labeler:Arc<dyn Labeler + Send + Sync>
}

impl<F> Clone for ExperimentConfig<F> {
    fn clone(&self) -> Self {
        Self {
            filter: self.filter.clone(),
            feature_builder: self.feature_builder.clone(),
            scorer: self.scorer.clone(),
            selector: self.selector.clone(),
            labeler: self.labeler.clone(),
        }
    }
}