#[macro_use]
extern crate log;

mod base_fs_cache;
pub mod errors;
mod file_set;
pub mod processing_fs_cache;

pub use file_set::*;
pub use processing_fs_cache::*;

#[cfg(test)]
//to demonstrate operation... Given some arbitrary directories (/bin, /usr), cache the length of all files inside,
//and then print them to the screen.
#[test]
fn example_application() {
    use std::path::PathBuf;

    //configure the logger to print debug information.
    {
        use simplelog::{ConfigBuilder, LevelFilter::*, TermLogger, TerminalMode::Stdout};
        let config = ConfigBuilder::new().build();
        TermLogger::init(Trace, config, Stdout).unwrap()
    }

    //path to save the cache. The hardcoded random string is to (lazily) avoid clobbering other items in /tmp.
    let cache_path = PathBuf::from("/tmp/hoxvrqdbfmoqodwkuooz/cache.bin");

    //Here are two arbitrary dirs that we will cache. There is a feature to ignore some paths.
    //To demonstrate this we will choose to ignore /usr/sbin.
    let dirs_to_process = [PathBuf::from("/bin"), PathBuf::from("/usr")];
    let excl_dirs = [PathBuf::from("/usr/bin")];

    //our cache deserves its own named type.
    type FileLenCache = ProcessingFsCache<u64>;

    //the cache will write itself to disk once this many changes have been made in memory.
    let save_threshold = 1000;

    //To do the actual processing, we pass a 'processing function' to the cache upon creation.
    //You will need to write your own processing code in here, to cache whatever you want to cache.
    //
    //Here we are caching u64s, but but arbitrary type can be stored in the cache.
    let file_len_fn = |src_path: PathBuf| -> u64 {
        match std::fs::metadata(src_path) {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        }
    };
    let file_len_fn = Box::new(file_len_fn);

    //create the cache...
    //note we are silently ignoring errors here in this example code.
    let cache = FileLenCache::new(save_threshold, cache_path, file_len_fn).unwrap();

    //file_set enumerates the paths in dirs_to_process that are not also in excl_dirs
    //implementation note: The behaviour embodied by FileSet could have been placed inside ProcessingFsCache,
    //and thus hidden. But for now this has not been done.
    let mut file_set = FileSet::new(&dirs_to_process, &excl_dirs);

    //now perform the processing. The program will obtain all file lengths from the given path.
    cache.update_from_fs(&mut file_set).unwrap();

    //Now processing is done, we can query the cache for file lengths without having to visit those files.
    //for now, print them all to screen.
    for src_path in cache.keys() {
        println!(
            "len: {:12}, path: {}",
            cache.get(src_path.clone()).unwrap(),
            src_path.display()
        );
    }
}
