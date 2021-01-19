# Generic Filesystem Cache

A small rust library for caching slow-to-compute data based on hard drive contents. Given a set of starting paths and a 'processing function' supplied by you, this library will recursively scan the filesystem from those starting paths and apply the processing function to each file.

The cache will save cached data to disk at a path given by you whenever a set number of changes has occurred inside the cache.

When directed by you, the cache will update itself if the 'modification time' of any cached file is changed.


## Features
* Supports Parallel loading (through rayon)
* Will cache any serializable type
* Works on linux
* Not sure if this will work on windows
* I think the code is panic-free


## Example application.
There is an example appliation which will cache the lengths of all files in directories /bin and /usr/bin, and then print them. Execute it by running "cargo test -- --nocapture"


## Performance and behaviour notes
* When saving, this cache will rewrite the entire cache on disk. Depending on your IO performance and/or tolerance of pauses while this occurs, large may take unacceptably long to write.
  * However on my machine with an SSD, saving a cache of several-hundred-thousand small entries takes less than a second. (Note that if data is small, cache size is mostly determined by the length of the file paths)
* If there are any outstanding changes when the cache goes out of scope they will not be saved to disk automatically. Automatic saving can only be performed by implementing the Drop trait (which can not contain fallible code), but saving is an inherently error-prone operation (due to risk of filesystem errors). This means the user is responsible for calling ProcessingFsCache::save when the cache is no longer needed.
* Note that the cache does not contain any options for filtering or ignoring files, and thus will store an entry for every file it finds. If this is not desirable, you could maybe:
  * add filtering operations in the cache itself
  * change the datatype inside the cache to some sort of enum such as "enum CacheRecord{IrrelevantFile, RelevantFile(\<T\>)}"
  

## Todo
This crate is not yet mature enough to be hosted on crates.io due to the following reasons:
* Incomplete documentation.
* No unit tests (although the code 'seems to' work 'reliably' for me).
* Incomplete public exposure of types (you may have to export more types than are currently exported).
* Error types need to be neatened.
* Not yet fully compliant with rust API guidelines.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.