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
fn t_extension() {
    use std::path::Path;
    use std::process::Command;
    #[cfg(unix)]
    {
        // create some folders
        Command::new("cargo").arg("build").status().unwrap();
        Command::new("mkdir").arg("ziptest").status().unwrap();
        Command::new("touch")
            .arg("ziptest/foo.zip")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest/bar.jpg")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest/bar.bmp")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest/baz.bar")
            .status()
            .unwrap();
        Command::new("zip")
            .args(&["-r", "ziptest.zip", "ziptest/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest/"])
            .status()
            .unwrap();
        Command::new("target/debug/partun")
            .args(&["ziptest.zip", "--ext", "jpg"])
            .status()
            .unwrap();
        assert!(Path::new("ziptest/bar.jpg").exists());
        println!("Multiple extensions");
        Command::new("target/debug/partun")
            .args(&["ziptest.zip", "--ext", "jpg,bmp"])
            .status()
            .unwrap();
        assert!(Path::new("ziptest/bar.jpg").exists());
        assert!(Path::new("ziptest/bar.bmp").exists());
        Command::new("rm")
            .args(&["-rf", "ziptest/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest.zip"])
            .status()
            .unwrap();
    }
}

#[test]
fn t_abs() {
    use std::process::Command;
    #[cfg(unix)]
    {
        // create some folders
        Command::new("cargo").arg("build").status().unwrap();
        Command::new("mkdir").arg("ziptest").status().unwrap();
        Command::new("touch")
            .arg("ziptest/foo.zip")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest/bar.jpg")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest/baz.bar")
            .status()
            .unwrap();
        Command::new("zip")
            .args(&["-r", "ziptest.zip", "ziptest/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest/"])
            .status()
            .unwrap();
        Command::new("target/debug/partun")
            .args(&["/home/woelper/repos/partun/ziptest.zip", "-l"])
            .status()
            .unwrap();
        Command::new("target/debug/partun")
            .args(&[
                "/home/woelper/repos/partun/ziptest.zip",
                "-l",
                "--include-archive-name",
            ])
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
    }
}

#[test]
fn t_dupe() {
    use std::process::Command;
    #[cfg(unix)]
    {
        // create some folders
        Command::new("cargo").arg("build").status().unwrap();
        Command::new("mkdir").arg("ziptest").status().unwrap();
        Command::new("mkdir").arg("ziptest/foo").status().unwrap();
        Command::new("touch")
            .arg("ziptest/foo/bar.jpg")
            .status()
            .unwrap();
        Command::new("touch")
            .arg("ziptest/bar.jpg")
            .status()
            .unwrap();
        Command::new("zip")
            .args(&["-r", "ziptest.zip", "ziptest/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest/"])
            .status()
            .unwrap();
        Command::new("target/debug/partun")
            .args(&[
                "/home/woelper/repos/partun/ziptest.zip",
                "-l",
                "--skip-duplicate-filenames",
            ])
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

#[test]
fn t_exts() {
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
            .args(&["-r", "ziptest_list.zzz", "ziptest_list/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest_list/"])
            .status()
            .unwrap();
        Command::new("target/debug/partun")
            .args(&["ziptest_list.zzz", "--list"])
            .status()
            .unwrap();

        Command::new("rm")
            .args(&["-rf", "ziptest_list/"])
            .status()
            .unwrap();
        Command::new("rm")
            .args(&["-rf", "ziptest_list.zzz"])
            .status()
            .unwrap();
    }
}
