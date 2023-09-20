use clap::{Arg, App};
use log::debug;
use log::info;
use sevenz_rust;
// use log::info;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
mod tests;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("Partun")
    .about("Extracts zip or 7z files partially")
    .arg(Arg::new("filter")
         .short('f')
         .long("filter")
         .help("Only extract file containing this string")
         .takes_value(true)
        )
    .arg(Arg::new("ext")
        .long("ext")
        .help("Only extract files with this extension (e.g. gif). Use commas for multiple exclusions.")
        .takes_value(true)
       )
    .arg(Arg::new("skip-duplicate-filenames")
       .long("skip-duplicate-filenames")
       .help("Do not extract duplicate file names")
      )
    .arg(Arg::new("exclude")
         .short('e')
         .long("exclude")
         .help("Do not extract file containing this string. Use commas for multiple exclusions.")
         .takes_value(true)
        )
    .arg(Arg::new("output")
         .short('o')
         .long("output")
         .help("extract files to this location")
         .takes_value(true)
        )
    .arg(Arg::new("rename")
         .long("rename")
         .help("Rename EVERY file to this string. Useful in scripts with the random option")
         .takes_value(true)
        )
    .arg(Arg::new("ignorepath")
         .short('i')
         .long("ignorepath")
         .help("Extract all files to current dir, ignoring all paths")
        )
    .arg(Arg::new("random")
         .short('r')
         .long("random")
         .help("Extract only a random file. This can be combined with the filter flag.")
        )
    .arg(Arg::new("list")
         .short('l')
         .long("list")
         .help("List files instead of extracting, one per line. Filtes apply.")
        )
    .arg(Arg::new("include-archive-name")
        .long("include-archive-name")
        .help("When listing, include the archive name in path")
       )
    .arg(Arg::new("debug")
        .long("debug")
        .help("Toggle debug output")
       )
    .arg(Arg::new("ZIP")
         .help("Sets the input archive")
         .required(true)
         .index(1)
        )
    .arg(Arg::new("type")
         .short('t')
         .long("type")
         .help("Specify the archive type: zip or 7z")
         .takes_value(true)
         .default_value("zip")
        )
    .get_matches();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    if matches.is_present("debug") {
        std::env::set_var("RUST_LOG", "debug");
    }
    let _ = env_logger::try_init();

    let archive = matches.value_of("ZIP").expect("must supply archive");
    let filter = matches.value_of("filter");
    let ext = matches.value_of("ext");
    let exclude = matches.value_of("exclude");
    let rename = matches.value_of("rename");
    let do_ignorepath = matches.is_present("ignorepath");
    let do_random = matches.is_present("random");
    let do_include_archive_name = matches.is_present("include-archive-name");
    let skip_dupe_filenames = matches.is_present("skip-duplicate-filenames");
    let out_path = PathBuf::from(matches.value_of("output").unwrap_or("."));

    let archive_type = matches.value_of("type").unwrap();
    match archive_type {
        "zip" => {
                    
            let archive_path = std::path::Path::new(archive);
            let zipfile = fs::File::open(&archive_path).unwrap();
        
            let mut zip_archive = zip::ZipArchive::new(zipfile).unwrap();
        
            let mut names = zip_archive
                .file_names()
                .filter(|name| {
                    name.to_lowercase()
                        .contains(&filter.unwrap_or_default().to_lowercase())
                        && {
                            if exclude.is_none() {
                                true
                            } else {
                                !exclude
                                    .unwrap_or_default()
                                    .replace(" ", "")
                                    .split(',')
                                    .any(|e| name.to_lowercase().contains(&e.to_lowercase()))
                            }
                        }
                })
                .filter(|name| match ext {
                    // Some(ext) => name.to_lowercase().ends_with(ext),
                    Some(ext) => ext
                        .replace(" ", "")
                        .split(',')
                        .any(|e| name.to_lowercase().ends_with(&e.to_lowercase())),
                    None => true,
                })
                .map(|n| n.into())
                .collect::<Vec<String>>();
        
            if do_random {
                // Make sure we don't include directories for selecting a random file
                // For that reason, filter indices to exclude directories.
                names = names
                    .into_iter()
                    .filter(|name| !name.ends_with('/'))
                    .collect();
                // select one of the indices - if there is any
                if let Some(n) = names.choose(&mut rand::thread_rng()) {
                    names = vec![n.clone()];
                }
            }
        
            // let mut crcmap: HashSet<u32> = HashSet::default();
            let mut name_map: HashSet<String> = HashSet::default();
        
            let mut stdout = io::stdout();
            for name in names.iter() {
                if skip_dupe_filenames {
                    if let Some(filename) = Path::new(name).file_name() {
                        let string_name = filename.to_string_lossy().to_string();
                        // info!("{string_name}");
                        if name_map.contains(&string_name) {
                            debug!("Skipping {name}");
                            continue;
                        } else {
                            name_map.insert(string_name);
                        }
                    }
                }
        
                // if list option is given, do not extract
                if matches.is_present("list") {
                    // if archive name should be included, do nothing, else use parent
                    let archive_path = match do_include_archive_name {
                        true => archive_path,
                        false => archive_path.parent().unwrap_or(Path::new(".")),
                    };
                    if let Err(err) = writeln!(&mut stdout, "{}", archive_path.join(name).display()) {
                        return if err.kind() == io::ErrorKind::BrokenPipe {
                            Ok(())
                        } else {
                            Err(err)
                        };
                    }
                    continue;
                }
        
                let mut zipfile = zip_archive
                    .by_name(name)
                    .expect(&format!("Can't get zipfile index by name {name}"));
                // let crc = zipfile.crc32();
                // if crcmap.contains(&crc) && do_skip_dupes {
                //     info!("Skipping {name}");
                // } else {
                //     crcmap.insert(crc);
                // }
        
                let mut inflated_file = out_path.join(zipfile.mangled_name());
        
                debug!("Base outpath: {}", inflated_file.display());
        
                // If ignorepath is set, turn the filename into the path
                if do_ignorepath {
                    if let Some(p) = inflated_file.file_name() {
                        inflated_file = out_path.join(p);
                    }
                    if inflated_file.is_dir() {
                        continue;
                    }
                    if (&*zipfile.name()).ends_with('/') {
                        continue;
                    }
                }
        
                if (&*zipfile.name()).ends_with('/') {
                    fs::create_dir_all(&inflated_file).unwrap();
                } else {
                    if let Some(p) = inflated_file.parent() {
                        if !p.exists() {
                            fs::create_dir_all(&p).unwrap();
                        }
                    }
        
                    println!("{}", inflated_file.as_path().display());
                    if let Some(r) = rename {
                        inflated_file = PathBuf::from(r);
                    }
        
                    let mut outfile = fs::File::create(&inflated_file).unwrap();
                    io::copy(&mut zipfile, &mut outfile).unwrap();
                }
        
                // Get and Set permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = zipfile.unix_mode() {
                        fs::set_permissions(&inflated_file, fs::Permissions::from_mode(mode)).unwrap();
                    }
                }
            }
        },
        "7z" => {
            // New 7z logic using sevenz-rust
            sevenz_rust::decompress_file(archive, &out_path.display().to_string()).expect("Decompression failed");
        },
        _ => {
            eprintln!("Unsupported archive type: {}", archive_type);
        }
    }
}


