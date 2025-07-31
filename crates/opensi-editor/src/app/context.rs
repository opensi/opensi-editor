use std::{
    ops::{Deref, DerefMut},
    path::Path,
    sync::Arc,
};

use opensi_core::prelude::*;

use crate::{
    EditorApp,
    app::{
        PackageState,
        files::{self, FileError, FileLoader, LoadingResult},
    },
};

/// [`AppContext`] subset when there is an active package.
pub struct PackageContext<'a> {
    ctx: AppContext<'a>,
}

impl<'a> Deref for PackageContext<'a> {
    type Target = AppContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl<'a> DerefMut for PackageContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

impl<'a> PackageContext<'a> {
    pub fn try_new(app: &'a mut EditorApp) -> Option<Self> {
        if !app.has_active_package() {
            return None;
        }
        Some(Self { ctx: app.into() })
    }

    pub fn package(&mut self) -> &mut Package {
        match self.app.package_state {
            PackageState::Active { ref mut package, .. } => package,
            _ => unimplemented!("Package state mismatch for PackageContext"),
        }
    }

    pub fn selected(&self) -> Option<PackageNode> {
        match self.app.package_state {
            PackageState::Active { selected, .. } => selected,
            _ => unimplemented!("Package state mismatch for PackageContext"),
        }
    }

    pub fn select(&mut self, node: PackageNode) {
        match self.app.package_state {
            PackageState::Active { ref mut selected, .. } => *selected = Some(node),
            _ => unimplemented!("Package state mismatch for PackageContext"),
        }
    }

    pub fn deselect(&mut self) {
        match self.app.package_state {
            PackageState::Active { ref mut selected, .. } => *selected = None,
            _ => unimplemented!("Package state mismatch for PackageContext"),
        }
    }
}

/// Context for the whole app with comfortable API.
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
        self.app.files_queue.push(loader);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_new_package(&mut self, path: impl AsRef<Path>) {
        let loader = files::load_file(path, package_loader);
        self.app.files_queue.push(loader);
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
