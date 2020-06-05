use std::{
    fmt,
    path::{Path, PathBuf},
};

pub struct Listing {
    paths: Vec<PathBuf>,
}

impl Listing {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let paths: Result<Vec<_>, _> = std::fs::read_dir(path)?
            .map(|result| result.map(|dir_entry| dir_entry.path()))
            .collect();

        let paths = paths?;

        Ok(Self { paths })
    }
}

impl fmt::Display for Listing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for path in &self.paths {
            writeln!(f, "{}", path.display())?;
        }

        Ok(())
    }
}
