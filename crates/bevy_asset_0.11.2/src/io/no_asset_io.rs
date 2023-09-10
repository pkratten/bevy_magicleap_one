use crate::{AssetIo, AssetIoError, ChangeWatcher, Metadata};
use anyhow::Result;
use bevy_utils::BoxedFuture;
use std::path::{Path, PathBuf};

pub struct NoAssetIO {}

impl NoAssetIO {
    /// Creates a new `NoAssetIO`.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        NoAssetIO {}
    }
}

impl AssetIo for NoAssetIO {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        bevy_log::error!("Loading assets from filesystem is not currently supproted!");
        Box::pin(async move {
            //implement here.
            Err(AssetIoError::NotFound(PathBuf::default()))
        })
    }

    fn read_directory(
        &self,
        _path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        bevy_log::warn!("Loading folders from filesystem is not currently supproted!");
        //implement here.
        Ok(Box::new(std::iter::empty::<PathBuf>()))
    }

    fn watch_path_for_changes(
        &self,
        _to_watch: &Path,
        _to_reload: Option<PathBuf>,
    ) -> Result<(), AssetIoError> {
        //implement here.
        Ok(())
    }

    fn watch_for_changes(&self, _configuration: &ChangeWatcher) -> Result<(), AssetIoError> {
        //implement here.
        bevy_log::warn!("Watching for changes is not currently supported!");
        Ok(())
    }

    fn get_metadata(&self, path: &Path) -> Result<Metadata, AssetIoError> {
        bevy_log::warn!("Getting metadata from filesystem is not currently supported!");
        //implement here.
        Err(AssetIoError::NotFound(PathBuf::default()))
    }
}
