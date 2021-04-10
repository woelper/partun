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
    let rename = matches.value_of("rename");
    let do_ignorepath = matches.is_present("ignorepath");
    let do_random = matches.is_present("random");


    let archive_path = std::path::Path::new(archive);
    let zipfile = fs::File::open(&archive_path).unwrap();

    let mut zip_archive = zip::ZipArchive::new(zipfile).unwrap();

    let mut indices = (0..zip_archive.len()).collect::<Vec<_>>().into_iter().filter(|x| {
        //if a filter flag is passed
        if let Some(f) = filter {
            let file = zip_archive.by_index(*x).unwrap();
            return file.name().contains(f)
        }
        else {
            return true
        }
    }).collect::<Vec<_>>();

    if do_random {
        indices = vec![indices.choose(&mut rand::thread_rng()).unwrap_or(&0).clone()];
    }
    
    // for i in 0..zip_archive.len() {
    for i in indices {
        let mut file = zip_archive.by_index(i).unwrap();
        let mut outpath = file.sanitized_name();

        // Make sure only filtered items pass
        if let Some(f) = filter {
            if !file.name().contains(f) {
                continue;
            }
        }

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
            // println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
                fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            
            if let Some(r) = rename {
                outpath = PathBuf::from(r);
            }

            println!("{}", outpath.as_path().display());


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