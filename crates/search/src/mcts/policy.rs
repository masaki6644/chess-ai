use crate::types::SearchPolicy;

pub fn visits_to_policy(

    visits:
        &[(String, u32)],
)
-> Vec<SearchPolicy>
{
    let total: u32 =
        visits
            .iter()
            .map(|(_, v)| *v)
            .sum();

    if total == 0 {
        return vec![];
    }

    visits
        .iter()
        .map(|(mv, v)| {

            SearchPolicy {

                mv: mv.clone(),

                visits: *v,

                probability:
                    *v as f32
                    / total as f32,
            }
        })
        .collect()
}