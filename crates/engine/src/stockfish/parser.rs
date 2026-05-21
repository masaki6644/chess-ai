use crate::types::Evaluation;

pub fn parse_info_line(
    line: &str,
) -> Option<Evaluation> {

    let parts: Vec<_> =
        line.split_whitespace()
            .collect();

    if parts.is_empty() {
        return None;
    }

    if parts[0] != "info" {
        return None;
    }

    let mut depth = 0u32;

    let mut nodes = 0u64;

    let mut cp = None;

    let mut mate = None;

    let mut pv = vec![];

    let mut i = 1;

    while i < parts.len() {

        match parts[i] {

            "depth" => {

                if let Some(v) =
                    parts.get(i + 1)
                {
                    depth =
                        v.parse().ok()?;
                }

                i += 2;
            }

            "nodes" => {

                if let Some(v) =
                    parts.get(i + 1)
                {
                    nodes =
                        v.parse().ok()?;
                }

                i += 2;
            }

            "cp" => {

                if let Some(v) =
                    parts.get(i + 1)
                {
                    cp =
                        v.parse().ok();
                }

                i += 2;
            }

            "mate" => {

                if let Some(v) =
                    parts.get(i + 1)
                {
                    mate =
                        v.parse().ok();
                }

                i += 2;
            }

            "pv" => {

                pv =
                    parts[i + 1..]
                        .iter()
                        .map(|s| s.to_string())
                        .collect();

                break;
            }

            _ => {
                i += 1;
            }
        }
    }

    Some(Evaluation {
        cp,
        mate,
        depth,
        nodes,
        pv,
    })
}