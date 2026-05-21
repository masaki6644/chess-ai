use crossbeam::channel::Sender;

use crate::utils::files::
    list_files_with_extension;

pub fn list_pgn_files(
    dir: &str,
) -> Vec<String> {

    list_files_with_extension(
        dir,
        "pgn",
    )
}

pub fn enqueue_pgn_jobs(

    files: &[String],

    job_tx:
        Sender<(usize, String)>,
) {
    for (
        file_id,
        path,
    ) in files.iter().enumerate()
    {
        job_tx
            .send((
                file_id,
                path.clone(),
            ))
            .unwrap();
    }
}