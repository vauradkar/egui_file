use std::io::{self, Error};
use std::path::{Path, PathBuf};

use dyn_clone::DynClone;
use poll_promise::Promise;

use crate::Filter;

pub trait PromiseResult {
  fn is_ready(&self) -> bool;
  fn take(&mut self) -> Result<Vec<Box<dyn VfsFile>>, Error>;
}
pub type ReadDirReturnType = Result<Vec<Box<dyn VfsFile>>, Error>;

pub struct ReadDirResult {
  pub promise: Option<Promise<ReadDirReturnType>>,
}

impl PromiseResult for ReadDirResult {
  fn is_ready(&self) -> bool {
    if let Some(promise) = &self.promise {
      promise.ready().is_some()
    } else {
      false
    }
  }

  fn take(&mut self) -> Result<Vec<Box<dyn VfsFile>>, Error> {
    let promise = self.promise.take().unwrap();
    promise
      .try_take()
      .map_err(|_| "promise needs to be read to take it")
      .unwrap()
  }
}

pub trait Vfs {
  fn create_dir(&self, path: &Path) -> io::Result<()>;

  fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;

  fn read_folder(
    &self,
    path: &Path,
    show_system_files: bool,
    show_files_filter: &Filter<PathBuf>,
    #[cfg(unix)] show_hidden: bool,
    #[cfg(windows)] show_drives: bool,
  ) -> Box<dyn PromiseResult>;
}

pub trait VfsFile: std::fmt::Debug + DynClone + Send + Sync {
  fn is_file(&self) -> bool;
  fn is_dir(&self) -> bool;
  fn path(&self) -> &Path;
  fn selected(&self) -> bool;
  fn set_selected(&mut self, selected: bool);
  fn get_file_name(&self) -> &str;
}

dyn_clone::clone_trait_object!(VfsFile);
