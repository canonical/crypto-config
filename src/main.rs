use clap::{Parser, Subcommand};
use serde_json;
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;
use topological_sort::TopologicalSort;

/*
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
}
*/

#[derive(Deserialize, Debug)]
struct ProfileMetadata {
    parent: String,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    GenerateRuntimeProfiles,
    GetCurrent,
    GetEnrolled,
    Status,
    Switch {
        profile: String,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, env)]
    destdir: Option<String>,
    #[command(subcommand)]
    cmd: Commands,
}

struct Paths<'a> {
    current: PathBuf,
    default: PathBuf,
    destdir: &'a Option<String>,
    state_dir: PathBuf,
    state_profiles_dir: PathBuf,
    system_profiles_dir: PathBuf,
}

fn paths<'a>(destdir: &'a Option<String>) -> Paths<'a> {
    let pathbuf = PathBuf::from(destdir.as_ref().map(String::as_str).unwrap_or("/"));

    let system_profiles_dir = pathbuf.join("usr").join("share").join("crypto-config").join("profiles");
    let state_dir = pathbuf.join("var").join("lib").join("crypto-config");

    let state_profiles_dir = state_dir.join("profiles");

    let default = state_profiles_dir.join("default");

    let current = state_profiles_dir.join("current");

    return Paths {
        current,
        default,
        destdir,
        state_dir,
        state_profiles_dir,
        system_profiles_dir,
    };
}

fn get_current(paths: Paths) -> Result<String, Error> {
    return Ok(
        paths
        .current
        .read_link()?
        .file_name()
        .ok_or(std::io::ErrorKind::InvalidInput)?
        .to_os_string()
        .to_str()
        .ok_or(std::io::ErrorKind::InvalidInput)?
        .to_string()
    );
}

fn get_profiles(paths: &Paths) -> Result<Vec<OsString>, Error> {
    let mut profiles = Vec::new();

    for entry in std::fs::read_dir(&paths.state_profiles_dir)? {
        let entry = entry?;
        if ! Path::new(&entry.path()).exists() {
            continue;
        }

        profiles.push(entry.file_name());
    }

    return Ok(profiles);
}

fn load_profile_metadata(paths: &Paths, profile: &OsString) -> Result<ProfileMetadata, Error> {
    let file = File::open(paths.state_profiles_dir.join(profile))?;
    let reader = BufReader::new(file);
    let metadata = serde_json::from_reader(reader)?;

    return Ok(metadata);
}

fn get_profiles_inheritance(paths: &Paths) -> Result<Vec<(OsString, OsString)>, Error> {
    let mut v = Vec::new();

    let profiles = get_profiles(&paths)?;

    for profile in profiles {
        let metadata = load_profile_metadata(paths, &profile)?;

        v.push((profile, OsString::from(metadata.parent)));
    }

    return Ok(v);
}

fn get_enrolled(paths: &Paths) -> Result<Vec<OsString>, Error> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(&(paths.default))? {
        let path = entry?.file_name();
        if path != "metadata.json" {
            entries.push(path);
        }
    }

    return Ok(entries);
}

fn generate_runtime_profiles(paths: &Paths) -> Result<(), Error> {
    let profiles = get_profiles_inheritance(&paths)?;
    let enrolled = get_enrolled(&paths)?;

    let mut tsort = TopologicalSort::<&OsString>::new();
    let mut parents = HashMap::new();

    for (profile, parent) in &profiles {
        tsort.add_dependency(parent, profile);
        parents.insert(profile, parent);
    }

    while tsort.len() != 0 {
        let profile = tsort.pop();
        assert!(profile.is_some(), "cyclic reference in profiles inheritance tree");
        let profile = profile.unwrap();

        /* XXX: this does not handle sub-directories */
        let profile_dir = paths.state_profiles_dir.join(&profile);

        if profile == "default" {
            std::os::unix::fs::symlink(paths.system_profiles_dir.join(profile), profile_dir)?;
            continue;
        }

        let parent_profile = parents.get(profile);
        assert!(parent_profile.is_some(), "lost parent of {}", profile.to_str().unwrap());
        let parent_profile = parent_profile.unwrap();

        let mut dir_builder = DirBuilder::new();
        dir_builder.recursive(true);

        dir_builder.create(&profile_dir)?;

        for e in &enrolled {
            let link = profile_dir.join(e);

            let target = paths.system_profiles_dir.join(profile).join(e);
            if Path::new(&target).exists() {
                std::os::unix::fs::symlink(link, target)?;
                continue;
            }

            let target = paths.state_profiles_dir.join(parent_profile).join(e);
            if Path::new(&target).exists() {
                std::os::unix::fs::symlink(link, target)?;
                continue;
            }

            let _ = fs::remove_file(link);
        }
    }

    if Path::new(&paths.current).exists() {
        std::os::unix::fs::symlink(&paths.current, "default")?;
    }

    return Ok(());
}

fn status(paths: Paths) -> Result<(), Error> {
    let current = get_current(paths)?;

    println!("Current profile: '{}'", current);

    return Ok(());
}

fn switch(paths: Paths, profile: &str) -> Result<(), Error> {
    let next = fs::canonicalize(paths.state_profiles_dir.join(profile))?;
    let metadata = next.metadata()?;
    let file_type = metadata.file_type();
    if ! (file_type.is_dir()) {
        return Err(std::io::ErrorKind::InvalidInput.into());
        /* TODO: use std::io::ErrorKind::NotADirectory; */
    }

    let tmp = paths.current.join(".tmp");

    std::os::unix::fs::symlink(profile, &tmp)?;
    std::fs::rename(&tmp, paths.current)?;

    return Ok(());
}

fn main() -> Result<(), Error> {
    /*
    let _command = Command::new("crypto-config")
        .version("0.7.0")
        .arg(
            Arg::new("status")
            .help("Print the current status")
        )
        .arg(
            Arg::new("switch")
            .help("Switch to the profile given")
        )
        .arg(
            Arg::new("get-current")
            .help("Print the current profile")
        )
        .arg(
            Arg::new("generate-runtime-profiles")
            .help("Generate the files that are used at runtime from the installed ones")
        )
        .after_help("some text after help")
        .get_matches ();
    */
    match Args::try_parse () {
        Ok(cli) => {
            let paths = paths(&cli.destdir);
            match cli.cmd {
                Commands::GenerateRuntimeProfiles => {
                    generate_runtime_profiles(&paths)?;
                },
                Commands::GetCurrent => {
                    let current = get_current(paths)?;
                    println!("{}", current);
                },
                Commands::GetEnrolled => {
                    get_enrolled(&paths)?;
                },
                Commands::Status => {
                    status(paths)?;
                },
                Commands::Switch { profile : s } => {
                    switch(paths, &s)?;
                },
            }
        }
        Err(e) => println!("error: {}", e.to_string()),
    };

    return Ok(());
}
