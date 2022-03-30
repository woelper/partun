use clap::{Arg, Command};
use log::debug;
use rand::seq::SliceRandom;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    let _ = env_logger::try_init();

    let matches = Command::new("Partun")
    .about("Extracts zip files partially")
    .arg(Arg::new("filter")
         .short('f')
         .long("filter")
         .help("Only extract file containing this string")
         .takes_value(true)
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
        //  .short("rn")
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
         .help("List files instead of extracting, one per line.")
        )
    .arg(Arg::new("ZIP")
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
                            .split(',')
                            .any(|e| name.to_lowercase().contains(&e.to_lowercase()))
                    }
                }
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

    let mut stdout = io::stdout();
    for name in names.iter() {
        // if list option is given, do not extract
        if matches.is_present("list") {
            if let Err(err) = writeln!(&mut stdout, "{}", name) {
                return if err.kind() == io::ErrorKind::BrokenPipe {
                    Ok(())
                } else {
                    Err(err)
                };
            }
            continue;
        }

        let mut file = zip_archive.by_name(name).unwrap();

        let mut inflated_file = out_path.join(file.mangled_name());

        debug!("Base outpath: {}", inflated_file.display());

        // If ignorepath is set, turn the filename into the path
        if do_ignorepath {
            if let Some(p) = inflated_file.file_name() {
                inflated_file = out_path.join(p);
            }
            if inflated_file.is_dir() {
                continue;
            }
            if (&*file.name()).ends_with('/') {
                continue;
            }
        }

        if (&*file.name()).ends_with('/') {
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
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&inflated_file, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}

#[test]
fn extract() {
    use std::process::Command;
    #[cfg(unix)]
    {
        // create some folders
        Command::new("cargo").arg("build").status().unwrap();
        Command::new("mkdir")
            .arg("ziptest_extract")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest_extract/foo")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest_extract/bar")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest_extract/baz")
            .status()
            .unwrap();
        Command::new("zip")
            .args(&["-r", "ziptest_extract.zip", "ziptest_extract/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest_extract/"])
            .status()
            .unwrap();

        Command::new("target/debug/partun")
            .args(&["-r", "ziptest_extract.zip"])
            .status()
            .unwrap();

        Command::new("rm")
            .args(&["-rf", "ziptest_extract/"])
            .status()
            .unwrap();

        Command::new("target/debug/partun")
            .args(&["-r", "-e", "foo,bar,baz", "ziptest_extract.zip"])
            .status()
            .unwrap();

        Command::new("rm")
            .args(&["-rf", "ziptest_extract/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest_extract.zip"])
            .status()
            .unwrap();
    }
}

#[test]
fn t_output() {
    use std::process::Command;
    #[cfg(unix)]
    {
        // create some folders
        Command::new("cargo").arg("build").status().unwrap();
        Command::new("mkdir").arg("ziptest").status().unwrap();
        Command::new("touch").arg("ziptest/foo").status().unwrap();
        Command::new("touch").arg("ziptest/bar").status().unwrap();
        Command::new("touch").arg("ziptest/baz").status().unwrap();
        Command::new("zip")
            .args(&["-r", "ziptest.zip", "ziptest/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest/"])
            .status()
            .unwrap();

        Command::new("target/debug/partun")
            .args(&["ziptest.zip", "-i", "-r", "--output", "/tmp/"])
            .status()
            .unwrap();

        Command::new("rm")
            .args(&["-rf", "ziptest/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest.zip"])
            .status()
            .unwrap();

        // Command::new("rm").args(&["-rf", "ziptest/"]).status().unwrap();
    }
}

#[test]
fn t_list() {
    use std::process::Command;
    #[cfg(unix)]
    {
        // create some folders
        Command::new("cargo").arg("build").status().unwrap();
        Command::new("mkdir").arg("ziptest_list").status().unwrap();
        Command::new("touch")
            .arg("ziptest_list/foo")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest_list/bar")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest_list/baz")
            .status()
            .unwrap();
        Command::new("zip")
            .args(&["-r", "ziptest_list.zip", "ziptest_list/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest_list/"])
            .status()
            .unwrap();

        Command::new("target/debug/partun")
            .args(&["ziptest_list.zip", "--list"])
            .status()
            .unwrap();

        Command::new("target/debug/partun")
            .args(&["ziptest_list.zip", "--list", "-r"])
            .status()
            .unwrap();

        Command::new("rm")
            .args(&["-rf", "ziptest_list/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest_list.zip"])
            .status()
            .unwrap();
    }
}
