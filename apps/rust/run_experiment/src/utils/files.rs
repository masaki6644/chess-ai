use std::fs;
use std::path::Path;

// =========================
// list files by extension
// =========================
pub fn list_files_with_extension(

    dir: &str,

    ext: &str,
)
-> Vec<String>
{
    let mut files =
        Vec::new();

    for entry in fs::read_dir(dir)
        .expect("failed to read dir")
    {
        let entry =
            entry.expect("invalid entry");

        let path =
            entry.path();

        if path
            .extension()
            .map(|e| e == ext)
            .unwrap_or(false)
        {
            files.push(
                path
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }

    files.sort();

    files
}

// =========================
// ensure dir
// =========================
pub fn ensure_dir(
    path: &str,
) {
    fs::create_dir_all(path)
        .unwrap_or_else(|e| {

        panic!(
            "failed to create dir {} ({})",
            path,
            e,
        )
    });
}

// =========================
// filename only
// =========================
pub fn filename(
    path: &str,
)
-> String
{
    Path::new(path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
}