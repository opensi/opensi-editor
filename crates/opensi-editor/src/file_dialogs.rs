use std::path::Path;

use opensi_core::Package;
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
        // TODO: handle channel errors ?
        let _ = sender.send(package).unwrap();
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
pub fn export_dialog(_package: &Package) {
    // TODO: actuall data from package
    let bytes = [0];

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
    return dirs::home_dir().unwrap();
    #[cfg(target_arch = "wasm32")]
    return "/";
}
