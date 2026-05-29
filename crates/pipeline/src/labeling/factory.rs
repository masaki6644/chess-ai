use crate::labeling::worker::Labeler;

pub trait LabelerFactory<F>:
    Send
    + Sync
    + 'static
{
    type LabelerType:
        Labeler<F>;

    fn create(
        &self,
        worker_id: usize,
    ) -> Self::LabelerType;
}