extern crate git2;
extern crate tempdir;

mod repo;

fn main() {
    let tmp_repo = match repo::create_blank_project(None) {
        Ok(tmp_repo) => tmp_repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    println!("{:?}", tmp_repo.state());
}
