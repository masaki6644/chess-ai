use crate::labeling::dummy::DummyLabeler;
use crate::labeling::factory::LabelerFactory;

pub struct DummyLabelerFactory;

impl<F> LabelerFactory<F>
    for DummyLabelerFactory
where
    F: Clone,
{
    type LabelerType =
        DummyLabeler;

    fn create(
        &self,
        _worker_id: usize,
    ) -> Self::LabelerType {

        DummyLabeler
    }
}