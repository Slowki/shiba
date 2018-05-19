use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn create_proxies(hooks_dir: &std::path::Path) {
    static GET_HOOK_DIR: &'static str = "HOOK=$(readlink -f $0)\nHOOK_DIR=$(dirname $HOOK)";

    let mut hooks_buf = std::path::PathBuf::from(hooks_dir);
    for &hook in &[
        "applypatch-msg",
        "post-update",
        "prepare-commit-msg",
        "pre-receive",
        "commit-msg",
        "pre-commit",
        "pre-rebase",
    ] {
        hooks_buf.push(hook);
        if !hooks_buf.exists() {
            let mut file = std::fs::File::create(&hooks_buf).expect("Couldn't create hook file");
            let mut file_perm = file.metadata().unwrap().permissions();
            file_perm.set_mode(0o755);
            write!(&mut file, "#!/bin/sh\n{}\nif [ -e $HOOK_DIR/../../.shiba/{} ]; then\n    exec $HOOK_DIR/../../.shiba/{} $@\nfi", GET_HOOK_DIR, hook, hook).unwrap();
            file.set_permissions(file_perm).unwrap();
        }
        hooks_buf.pop();
    }
}

fn main() {
    let mut output_dir =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR undefined"));
    let mut project_dir = None;

    while output_dir.pop() {
        output_dir.push("Cargo.toml");
        if output_dir.exists() {
            output_dir.pop();
            project_dir = Some(output_dir);
            break;
        } else {
            output_dir.pop();
        }
    }

    if let Some(project) = project_dir {
        let mut git_dir = project.clone();
        let mut hook_dir = project.clone();
        git_dir.push(".git");
        hook_dir.push(".shiba");
        if git_dir.is_dir() && hook_dir.is_dir() {
            git_dir.push("hooks");
            if !git_dir.is_dir() {
                std::fs::create_dir(&git_dir).expect("failed to create .git/hooks directory");
            }
            create_proxies(git_dir.as_path());
        } else if !git_dir.is_dir() {
            eprintln!("{} isn't a git repo", git_dir.to_str().unwrap());
        } else if !hook_dir.is_dir() {
            eprintln!("{} doesn't exist", hook_dir.to_str().unwrap());
        }
    } else {
        eprintln!("Couldn't find the project directory");
    }
}
