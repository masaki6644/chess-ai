use pipeline::filter::GameFilter;
use pipeline::feature::FeatureBuilder;
use pipeline::score::Scorer;
use pipeline::select::Selector;

pub struct ExperimentConfig<'a, F> {
    pub filter: &'a dyn GameFilter,
    pub feature_builder: &'a dyn FeatureBuilder<Output = F>,
    pub scorer: &'a dyn Scorer<F>,
    pub selector: &'a dyn Selector,
}