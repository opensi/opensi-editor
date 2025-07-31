use std::path::Path;

use opensi_core::prelude::*;

use crate::{
    EditorApp,
    app::{
        PackageState,
        files::{self, FileError, LoadingResult},
    },
};

pub struct AppContext<'a> {
    app: &'a mut EditorApp,
}

impl<'a> From<&'a mut EditorApp> for AppContext<'a> {
    fn from(app: &'a mut EditorApp) -> Self {
        Self { app }
    }
}

impl AppContext<'_> {
    pub fn pick_new_package(&mut self) {
        let loader = files::pick_file(
            "Выбрать файл с вопросами для импорта",
            ("SIGame Pack", ["siq"]),
            package_loader,
        );
        self.app.loaders.push(loader);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_new_package(&mut self, path: impl AsRef<Path>) {
        let loader = files::load_file(path, package_loader);
        self.app.loaders.push(loader);
    }

    pub fn save_package(&mut self) {
        let PackageState::Active { ref package, .. } = self.app.package_state else {
            return;
        };

        let package = package.clone();
        files::save_to("Сохранить пакет с вопросами", "pack.siq", move || package.to_bytes().ok());
    }
}

/// Adapter for [`Package`] to use with [`FileLoader`].
fn package_loader(buffer: Vec<u8>, path: &Path, app: &mut EditorApp) -> LoadingResult<()> {
    let package = Package::from_zip_buffer(buffer).map_err(FileError::ArchiveError)?;

    // load all images into memory
    for (id, bytes) in &package.resources {
        app.storage.insert(id, &package, bytes.clone());
    }

    app.package_state = PackageState::Active { package, selected: None };

    // update recent files
    app.recent_files.remove(path);
    app.recent_files.insert(path.to_owned());
    app.recent_files = std::mem::take(&mut app.recent_files).into_iter().take(10).collect();

    Ok(())
}
