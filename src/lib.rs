use std::{fmt, path::PathBuf};

pub struct Listing {
    root: PathBuf,
    child_paths: Vec<PathBuf>,
}

impl Listing {
    pub fn new(root: PathBuf) -> anyhow::Result<Self> {
        let paths: Result<Vec<_>, _> = std::fs::read_dir(&root)?
            .map(|result| result.map(|dir_entry| dir_entry.path()))
            .collect();

        let paths = paths?;

        Ok(Self {
            root,
            child_paths: paths,
        })
    }
}

impl fmt::Display for Listing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for path in &self.child_paths {
            // This strip_prefix call cannot fail because all the child paths start from root.
            writeln!(f, "{}", path.strip_prefix(&self.root).unwrap().display())?;
        }

        Ok(())
    }
}
