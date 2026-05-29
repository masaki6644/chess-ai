use crate::types::SearchResult;

pub trait Search {

    fn search(
        &mut self,

        fen: &str,
    ) -> SearchResult;
}