use tempdir::TempDir;
use git2::{Repository, Error};

pub fn create_blank_project(filepath: Option<&'static str>) -> Result<Repository, Error> {
    let repo = match filepath {
        Some(path) => try!(Repository::init(path)),
        None => {
            let tmp_dir = TempDir::new("mobius-desk-repo").expect("create temp dir");
            try!(Repository::init(tmp_dir.path()))
        }
    };
    Ok(repo)
}

