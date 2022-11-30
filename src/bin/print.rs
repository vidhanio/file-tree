use std::path::PathBuf;

use file_tree::FileTree;

#[fncli::cli]
fn main(path: PathBuf) {
    let tree = FileTree::try_from(&*path).unwrap();
    println!("{path:?}: {tree:#?}");
}
