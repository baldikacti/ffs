//use dashmap::mapref::entry;
use extendr_api::prelude::*;
use std::{os::unix::fs::MetadataExt, path::PathBuf};
//use std::sync::atomic::{AtomicU64, Ordering};
use walkdir::WalkDir;
use users::get_user_by_uid;
use rayon::prelude::*;
//use dashmap::DashMap;

/// Returns a dataframe of file information for all files in the selected directory recursively
/// @export
#[extendr]
fn dir_info(path: String) -> Robj {
    // Start directory for the scan
    //let start_path = std::env::current_dir().expect("Could not retrieve current directory");
    let start_path = PathBuf::from(path);

    // Collect all file metadata into DIRINFO structs
    let dir_info: Vec<DIRINFO> = WalkDir::new(&start_path)
        .into_iter()
        .map(|entry| {
            // Return the DirEntry if it is Ok and return if not
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();
            // Get the UID and find the user name
            let uid = metadata.uid();
            // Retrive the filetype and call it "file" or "dir"
            let filetype = if metadata.is_file() { "file" } else { "dir" };

            let user_name = get_user_by_uid(uid)
                .map(|user| user.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| uid.to_string());
            DIRINFO {
                filename: entry.path().to_string_lossy().to_string(),
                size: metadata.size() as f64,
                user_name,
                filetype: filetype.to_string(),
            }
        })
        .collect();

    dir_info.into_dataframe().unwrap().as_robj().clone()
  
}

/// Returns a dataframe of file information for all files in the selected directory recursively
/// @export
#[extendr]
fn dir_info_par(path: String) -> Robj {
    // Start directory for the scan
    let start_path = PathBuf::from(path);

    // Collect all file metadata into DIRINFO structs
    let dir_info: Vec<DIRINFO> = WalkDir::new(&start_path)
        .into_iter()
        .par_bridge()
        .map(|entry| {
            // Return the DirEntry if it is Ok and return if not
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();
            // Get the UID and find the user name
            let uid = metadata.uid();
            // Retrive the filetype and call it "file" or "dir"
            let filetype = if metadata.is_file() { "file" } else { "dir" };

            let user_name = get_user_by_uid(uid)
                .map(|user| user.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| uid.to_string());
            DIRINFO {
                filename: entry.path().to_string_lossy().to_string(),
                size: metadata.size() as f64,
                user_name,
                filetype: filetype.to_string(),
            }
        }).collect();

    dir_info.into_dataframe().unwrap().as_robj().clone()
  
}

#[derive(Debug, Default, Clone, IntoDataFrameRow)]
struct DIRINFO {
    filename: String,
    size: f64,
    user_name: String,
    filetype: String,
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod ffs;
    fn dir_info;
    fn dir_info_par;
}
