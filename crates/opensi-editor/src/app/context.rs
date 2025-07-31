#![allow(unused)]

use std::{path::Path, sync::Arc};

use derive_more::{Deref, DerefMut};
use opensi_core::prelude::*;

use crate::{
    EditorApp,
    app::{
        PackageState,
        files::{self, FileError, FileLoader, LoadingResult},
    },
};

/// [`AppContext`] subset for a certain [`Question`].
#[derive(Deref, DerefMut)]
pub struct QuestionContext<'a, 'ctx> {
    #[deref]
    #[deref_mut]
    ctx: &'ctx mut PackageContext<'a>,
    idx: QuestionIdx,
}

impl<'a, 'ctx> QuestionContext<'a, 'ctx> {
    pub fn try_new(ctx: &'ctx mut PackageContext<'a>, idx: QuestionIdx) -> Option<Self> {
        if !ctx.package().contains_question(idx) {
            return None;
        }
        Some(Self { ctx, idx })
    }

    pub fn question(&mut self) -> &mut Question {
        match self.ctx.package().get_question_mut(self.idx) {
            Some(question) => question,
            _ => unimplemented!("QuestionContext state is invalid"),
        }
    }

    pub fn idx(&self) -> QuestionIdx {
        self.idx
    }
}

/// [`AppContext`] subset for a certain [`Theme`].
#[derive(Deref, DerefMut)]
pub struct ThemeContext<'a, 'ctx> {
    #[deref]
    #[deref_mut]
    ctx: &'ctx mut PackageContext<'a>,
    idx: ThemeIdx,
}

impl<'a, 'ctx> ThemeContext<'a, 'ctx> {
    pub fn try_new(ctx: &'ctx mut PackageContext<'a>, idx: ThemeIdx) -> Option<Self> {
        if !ctx.package().contains_theme(idx) {
            return None;
        }
        Some(Self { ctx, idx })
    }

    pub fn theme(&mut self) -> &mut Theme {
        match self.ctx.package().get_theme_mut(self.idx) {
            Some(theme) => theme,
            _ => unimplemented!("ThemeContext state is invalid"),
        }
    }

    pub fn idx(&self) -> ThemeIdx {
        self.idx
    }
}

/// [`AppContext`] subset for a certain [`Round`].
#[derive(Deref, DerefMut)]
pub struct RoundContext<'a, 'ctx> {
    #[deref]
    #[deref_mut]
    ctx: &'ctx mut PackageContext<'a>,
    idx: RoundIdx,
}

impl<'a, 'ctx> RoundContext<'a, 'ctx> {
    pub fn try_new(ctx: &'ctx mut PackageContext<'a>, idx: RoundIdx) -> Option<Self> {
        if !ctx.package().contains_round(idx) {
            return None;
        }
        Some(Self { ctx, idx })
    }

    pub fn round(&mut self) -> &mut Round {
        match self.ctx.package().get_round_mut(self.idx) {
            Some(round) => round,
            _ => unimplemented!("RoundContext state is invalid"),
        }
    }

    pub fn idx(&self) -> RoundIdx {
        self.idx
    }
}

/// [`AppContext`] subset when there is an active package.
#[derive(Deref, DerefMut)]
pub struct PackageContext<'a> {
    #[deref]
    #[deref_mut]
    ctx: AppContext<'a>,
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

    pub fn pick_new_image_for(&mut self, idx: QuestionIdx) {
        let loader = files::pick_file(
            "Выберите изображение",
            ("Image", ["png", "jpg", "jpeg"]),
            image_loader(idx),
        );
        self.app.files_queue.push(loader);
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

/// Adapter for [`Atom`] images to use with [`FileLoader`].
fn image_loader(idx: QuestionIdx) -> impl FileLoader {
    move |bytes: Vec<u8>, path: &Path, app: &mut EditorApp| -> LoadingResult<()> {
        let PackageState::Active { ref mut package, .. } = app.package_state else {
            return Err(FileError::LoaderError("No active package to load an image".into()));
        };

        let id = ResourceId::image(
            path.file_name()
                .map(|file_name| file_name.to_string_lossy().to_string())
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        );
        let bytes = Arc::from(bytes.into_boxed_slice());
        package.resources.insert(id.clone(), bytes);

        let Some(body) =
            app.storage.insert(&id, package, package.resources.get(&id).unwrap().clone())
        else {
            return Err(FileError::LoaderError("Can't load image bytes into memory".into()));
        };

        let Some(question) = package.get_question_mut(idx) else {
            return Err(FileError::LoaderError(
                format!("Can't add image '{body}' to question with idx {idx}").into(),
            ));
        };
        question.scenario.push(Atom {
            kind: AtomKind::Image,
            body: body.to_string(),
            ..Atom::default()
        });

        Ok(())
    }
}
