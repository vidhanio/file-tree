use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum FileTree {
    File,
    Directory(BTreeMap<OsString, FileTree>),
}

impl FileTree {
    pub fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();

        if path.exists() {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("`{}` already exists", path.display()),
            ))
        } else {
            match self {
                FileTree::File => {
                    File::create(path)?;
                }
                FileTree::Directory(children) => {
                    fs::create_dir(path)?;
                    for (name, child) in children {
                        child.create(&path.join(name))?;
                    }
                }
            };

            Ok(())
        }
    }
}

impl TryFrom<PathBuf> for FileTree {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        FileTree::try_from(&*path)
    }
}

impl TryFrom<&PathBuf> for FileTree {
    type Error = io::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        FileTree::try_from(&**path)
    }
}

impl TryFrom<&Path> for FileTree {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_file() {
            Ok(FileTree::File)
        } else if path.is_dir() {
            path.read_dir()?
                .map(|entry| {
                    let entry = entry?;

                    let path = entry.path();
                    let name = path.file_name().unwrap_or(path.as_os_str());

                    Ok((name.to_owned(), FileTree::try_from(&*path)?))
                })
                .collect::<Result<BTreeMap<_, _>, _>>()
                .map(FileTree::Directory)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("`{}` is not a file or directory", path.display()),
            ))
        }
    }
}
