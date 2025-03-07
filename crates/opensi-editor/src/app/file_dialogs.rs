use std::path::Path;

use log::error;
use opensi_core::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use tokio;
#[cfg(target_arch = "wasm32")]
use tokio_with_wasm::alias as tokio;

use tokio::sync::oneshot;

/// Custom result for in-progress loading [`Package`] and its [`PackageError`]s.
pub type LoadingPackageResult = Result<Package, PackageError>;
/// Receiver of a singular [`LoadingPackageResult`].
pub type LoadingPackageReceiver = oneshot::Receiver<LoadingPackageResult>;

/// Error for loading [`Package`].
#[derive(thiserror::Error, Debug)]
pub enum PackageError {
    #[error("No file was selected")]
    NoFileSelected,
    #[error("Archive error: {0}")]
    ArchiveError(std::io::Error),
}

/// Show a dialog for loading a new [`Package`] asynchronously, and
/// return a receiver with the result.
pub fn import_dialog() -> LoadingPackageReceiver {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let _handle = tokio::spawn(async {
        let package = import_package().await;
        match sender.send(package) {
            Ok(_) => {},
            Err(_) => error!("Error sending imported package !"),
        };
    });
    receiver
}

async fn import_package() -> LoadingPackageResult {
    let file = rfd::AsyncFileDialog::new()
        .set_title("Выбрать файл с вопросами для импорта")
        .add_filter("SIGame Pack", &["siq"])
        .set_directory(get_directory())
        .set_can_create_directories(false)
        .pick_file()
        .await
        .ok_or(PackageError::NoFileSelected)?;

    let buffer = file.read().await;
    Package::from_zip_buffer(buffer).map_err(PackageError::ArchiveError)
}

/// Show a dialog for saving existing [`Package`] asynchronously.
pub fn export_dialog(package: &Package) {
    let Ok(bytes) = package.clone().to_bytes() else {
        return;
    };

    let _handle = tokio::spawn(async move {
        let file = rfd::AsyncFileDialog::new()
            .set_title("Сохранить выбранный пакет с вопросами")
            .set_directory(get_directory())
            .set_file_name("pack.siq")
            .save_file()
            .await?;

        file.write(&bytes).await.ok()
    });
}

fn get_directory() -> impl AsRef<Path> {
    #[cfg(not(target_arch = "wasm32"))]
    return dirs::home_dir().unwrap_or_default();
    #[cfg(target_arch = "wasm32")]
    return "/";
}
