use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use log::{error, warn};
#[cfg(not(target_arch = "wasm32"))]
use tokio;
#[cfg(target_arch = "wasm32")]
use tokio_with_wasm::alias as tokio;

use tokio::sync::oneshot;

use crate::EditorApp;

/// Result for in-progress loading.
pub type LoadingResult<T> = Result<T, FileError>;

/// Receiver of a singular [`LoadingResult`] with file.
pub type LoadingFileReceiver = oneshot::Receiver<LoadingResult<(Vec<u8>, PathBuf)>>;
/// Result for loading files.
pub type LoadingFileResult = Result<(Vec<u8>, PathBuf), FileError>;

/// Error for loading files.
#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("Files loader error: {0}")]
    LoaderError(Cow<'static, str>),
    #[error("No file was selected")]
    NoFileSelected,
    #[error("Archive error: {0}")]
    ArchiveError(std::io::Error),
}

/// Async file loader queue that can mutate [`EditorApp`] upon loading.
pub struct FilesQueue {
    receiver: LoadingFileReceiver,
    op: Option<Box<dyn FileLoader>>,
}

impl FilesQueue {
    pub fn update(&mut self, app: &mut EditorApp) -> bool {
        match self.receiver.try_recv() {
            Ok(Ok((data, file))) => {
                if let Some(op) = self.op.take() {
                    let _ = op.load(data, file.as_path(), app).inspect_err(|err| {
                        error!("Error running a loader: {err}");
                    });
                }
                return true;
            },
            Ok(Err(err)) => {
                error!("Error loading file: {err}");
            },
            Err(_) => {},
        }
        false
    }
}

pub trait FileLoader {
    fn load(&self, bytes: Vec<u8>, path: &Path, app: &mut EditorApp) -> LoadingResult<()>;
}

impl<F> FileLoader for F
where
    F: Fn(Vec<u8>, &Path, &mut EditorApp) -> LoadingResult<()>,
{
    fn load(&self, bytes: Vec<u8>, path: &Path, app: &mut EditorApp) -> LoadingResult<()> {
        self(bytes, path, app)
    }
}

/// Read a file directly from a file on systems that support
/// direct file systems and return a [`FileLoader`]: it will
/// run `op` once the file is loaded.
#[cfg(not(target_arch = "wasm32"))]
#[must_use = "Use loader to properly load a file"]
pub fn load_file(path: impl AsRef<Path>, loader: impl FileLoader + 'static) -> FilesQueue {
    fn read_file(file: impl AsRef<Path>) -> LoadingFileResult {
        let file = file.as_ref();
        let buffer = std::fs::read(file).map_err(FileError::ArchiveError)?;
        Ok((buffer, file.to_owned()))
    }

    let (sender, receiver) = tokio::sync::oneshot::channel();
    match sender.send(read_file(path)) {
        Ok(_) => {},
        Err(_) => error!("Error sending imported package !"),
    };

    FilesQueue { receiver, op: Some(Box::new(loader)) }
}

/// Show a file picker and return a [`FileLoader`] with this file:
/// it will run `op` once the file is loaded.
#[must_use = "Use loader to properly load a file"]
pub fn pick_file(
    title: impl ToString,
    file_filter: (impl ToString, impl IntoIterator<Item = &'static str>),
    loader: impl FileLoader + 'static,
) -> FilesQueue {
    async fn show_file_picker(
        title: &String,
        file_filter: &(String, Vec<&'static str>),
    ) -> LoadingFileResult {
        let file = rfd::AsyncFileDialog::new()
            .set_title(title)
            .add_filter(&file_filter.0, &file_filter.1)
            .set_directory(default_directory())
            .set_can_create_directories(false)
            .pick_file()
            .await
            .ok_or(FileError::NoFileSelected)?;

        let buffer = file.read().await;

        #[cfg(not(target_arch = "wasm32"))]
        let path = file.path().to_owned();
        #[cfg(target_arch = "wasm32")]
        let path = file.file_name().into();

        Ok((buffer, path))
    }

    let title = title.to_string();
    let file_filter = (file_filter.0.to_string(), file_filter.1.into_iter().collect::<Vec<_>>());

    let (sender, receiver) = tokio::sync::oneshot::channel();
    let _handle = tokio::spawn(async move {
        let result = show_file_picker(&title, &file_filter).await;
        match sender.send(result) {
            Ok(_) => {},
            Err(_) => error!("Error sending picked file"),
        };
    });
    FilesQueue { receiver, op: Some(Box::new(loader)) }
}

/// Show a dialog to save file.
pub fn save_to(
    title: impl ToString,
    file_name: impl ToString,
    generate_data: impl FnOnce() -> Option<Vec<u8>> + Send + Sync + 'static,
) {
    let title = title.to_string();
    let file_name = file_name.to_string();

    let _handle = tokio::spawn(async move {
        let file = rfd::AsyncFileDialog::new()
            .set_title(title)
            .set_directory(default_directory())
            .set_file_name(file_name)
            .save_file()
            .await?;

        if let Some(bytes) = generate_data() {
            file.write(&bytes).await.ok()
        } else {
            warn!("File saving interrupted: no bytes");
            Some(())
        }
    });
}

/// Get default directory for file pickers.
fn default_directory() -> impl AsRef<Path> {
    #[cfg(not(target_arch = "wasm32"))]
    return dirs::home_dir().unwrap_or_default();
    #[cfg(target_arch = "wasm32")]
    return "/";
}
