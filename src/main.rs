use clap::{App, Arg};
use rand::seq::SliceRandom;
use std::fs;
use std::io;
use std::path::PathBuf;

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
         .help("Do not extract file containing this string. Use commas for multiple exclusions.")
         .takes_value(true)
        )
    .arg(Arg::with_name("output")
         .short("o")
         .long("output")
         .help("extract files to this location")
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
    let out_path = PathBuf::from(matches.value_of("output").unwrap_or("."));

    let archive_path = std::path::Path::new(archive);
    let zipfile = fs::File::open(&archive_path).unwrap();

    let mut zip_archive = zip::ZipArchive::new(zipfile).unwrap();

    let mut indices = (0..zip_archive.len())
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|i| {
            let zipfile = &zip_archive.by_index(*i).unwrap();
            zipfile.name().contains(filter.unwrap_or_default()) && {
                if exclude.is_none() {
                    true
                } else {
                    // !zipfile.name().contains(exclude.unwrap_or_default())
                    !exclude.unwrap_or_default().split(",").any(|e| zipfile.name().contains(e))
                }
            }
        })
        .collect::<Vec<_>>();

    if do_random {
        // Make sure we don't include directories for selecting a random file
        // For that reason, filter indices to exclude directories.
        indices = indices
            .into_iter()
            .filter(|i| {
                let zipfile = &zip_archive.by_index(*i).unwrap();
                !zipfile.name().ends_with("/")
            })
            .collect();
        // select one of the indices - if there is any
        if let Some(index) = indices.choose(&mut rand::thread_rng()) {
            indices = vec![*index];
        }
    }

    for i in indices {
        let mut file = zip_archive.by_index(i).unwrap();
        let mut outpath = out_path.join(file.sanitized_name());

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


#[test]
fn extract() {
    use std::process::Command;
    #[cfg(unix)]
    {
        // create some folders
        Command::new("cargo").arg("build").status().unwrap();
        Command::new("mkdir").arg("ziptest").status().unwrap();
        Command::new("touch").arg("ziptest/foo").status().unwrap();
        Command::new("touch").arg("ziptest/bar").status().unwrap();
        Command::new("touch").arg("ziptest/baz").status().unwrap();
        Command::new("zip").args(&["-r", "ziptest.zip", "ziptest/"]).status().unwrap();
        Command::new("rm").args(&["-rf", "ziptest/"]).status().unwrap();

        Command::new("target/debug/partun").args(&["-r", "ziptest.zip"]).status().unwrap();
        
        Command::new("rm").args(&["-rf", "ziptest/"]).status().unwrap();

        Command::new("target/debug/partun").args(&["-r", "-e", "foo,bar,baz", "ziptest.zip"]).status().unwrap();


        Command::new("rm").args(&["-rf", "ziptest/"]).status().unwrap();
        Command::new("rm").args(&["-rf", "ziptest.zip"]).status().unwrap();


    }
}