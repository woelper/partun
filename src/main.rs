use std::io;
use std::fs;
use std::path::PathBuf;
use clap::{Arg, App};
use rand::seq::SliceRandom;

fn main() {

    let matches = App::new("Partun")
    .about("Extracts zip files partially")
    .arg(Arg::with_name("filter")
         .short("f")
         .long("filter")
         .help("Only extract file containing this string")
         .takes_value(true)
        )
    .arg(Arg::with_name("exclude")
         .short("e")
         .long("exclude")
         .help("Do not extract file containing this string")
         .takes_value(true)
        )
    .arg(Arg::with_name("rename")
        //  .short("rn")
         .long("rename")
         .help("Rename EVERY file to this string. Useful in scripts with the random option")
         .takes_value(true)
        )
    .arg(Arg::with_name("ignorepath")
         .short("i")
         .long("ignorepath")
         .help("Extract all files to current dir, ignoring all paths")
        )
    .arg(Arg::with_name("random")
         .short("r")
         .long("random")
         .help("Extract only a random file. This can be combined with the filter flag.")
        )
    .arg(Arg::with_name("ZIP")
         .help("Sets the input file to use")
         .required(true)
         .index(1)
        )
    .get_matches();


    let archive = matches.value_of("ZIP").unwrap();
    let filter = matches.value_of("filter");
    let exclude = matches.value_of("exclude");
    let rename = matches.value_of("rename");
    let do_ignorepath = matches.is_present("ignorepath");
    let do_random = matches.is_present("random");

    let archive_path = std::path::Path::new(archive);
    let zipfile = fs::File::open(&archive_path).unwrap();

    let mut zip_archive = zip::ZipArchive::new(zipfile).unwrap();

    let mut indices = (0..zip_archive.len())
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|i| {
            let zipfile = &zip_archive.by_index(*i).unwrap();
            zipfile.name().contains(filter.unwrap_or_default()) && 
            {if exclude.is_none() {true} else {
                !zipfile.name().contains(exclude.unwrap_or_default())
            }}
        })


    .collect::<Vec<_>>();

    if do_random {
        
        // Make sure we don't include directories for selecting a random file
        // For that reason, filter indices to exclude directories.
        indices = indices.into_iter().filter(|i| {
            let zipfile = &zip_archive.by_index(*i).unwrap();
            !zipfile.name().ends_with("/")
        })
        .collect();
        // select one of the indices
        indices = vec![indices.choose(&mut rand::thread_rng()).unwrap_or(&0).clone()];
    }
    
    for i in indices {
        let mut file = zip_archive.by_index(i).unwrap();
        let mut outpath = file.sanitized_name();

        // If ignorepath is set, turn the filename into the path
        if do_ignorepath {
            if let Some(p) = outpath.file_name() {
                outpath = PathBuf::from(p);
            }
            if outpath.is_dir() {
                continue;
            }
            if (&*file.name()).ends_with('/') {
                continue;
            }
        }

        if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            
            println!("{}", outpath.as_path().display());
            if let Some(r) = rename {
                outpath = PathBuf::from(r);
            }


            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}