use pipeline::filter::GameFilter;
use pipeline::score::Scorer;
use pipeline::select::Selector;

pub struct ExperimentConfig<'a> {
    pub filter: &'a dyn GameFilter,
    pub scorer: &'a dyn Scorer,
    pub selector: &'a dyn Selector,
}