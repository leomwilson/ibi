use git2::Repository;
use colored::*;
use std::env;
use std::path::Path;

// because of the complexity of version control, this is in its own file

// this returns a String since it has to format!() multiple ColoredStrings
pub fn git() -> String {
    let mut repo: Option<Repository> = None;
    for dir in Path::new(&format!("{}", env::current_dir().unwrap_or("".into()).display())).ancestors() { // check if in a subdirectory of a git repo
        if let Ok(r) = Repository::open(dir) {
            repo = Some(r);
            break;
        }
    }
    match repo {
        Some(r) => {
            let mut sym = "".white();
            let mut head = match match r.head() {
                    Ok(b) => b,
                    Err(_) => return "".to_string()
                }.shorthand() {
                Some(b) => b,
                _ => return "".to_string()
            }.green();
            if let Ok(statuses) = r.statuses(Some(git2::StatusOptions::new().include_untracked(true))) { // status code mostly copied from reujab/silver
                for status in statuses.iter() {
                    let status = status.status();
                    if status.is_wt_new() || status.is_wt_modified() || status.is_wt_renamed() || status.is_wt_typechange() { // untracked changes
                        head = head.red();
                    } else if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() || status.is_index_renamed() || status.is_index_typechange() { // tracked changes
                        head = head.bold();
                    }
                }
            }
            if let Some((a, b)) = get_ahead_behind(&r) {
                if (a > 0) | (b > 0) {
                    sym = sym.red();
                } else {
                    sym = sym.green();
                }
            }
            format!("{} {}", sym, head)
        },
        None => "".to_string()
    }
}

// shamelessly copied from NerdyPepper/pista
// TODO: replace with your own code
fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
    let head = (r.head().ok())?;
    if !head.is_branch() {
        return None
    }

    let head_name    = (head.shorthand())?;
    let head_branch  = (r.find_branch(head_name, git2::BranchType::Local).ok())?;
    let upstream     = (head_branch.upstream().ok())?;
    let head_oid     = (head.target())?;
    let upstream_oid = (upstream.get().target())?;

    r.graph_ahead_behind(head_oid, upstream_oid).ok()
}
