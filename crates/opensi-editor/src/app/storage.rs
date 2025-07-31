use std::sync::Arc;

use opensi_core::prelude::*;

/// Storage adapter for egui to be able to use [`Package`] resources.
#[derive(Clone, Default, Debug)]
pub struct SharedPackageBytesStorage {
    cache: Arc<dashmap::DashMap<String, egui::load::Bytes>>,
}

impl SharedPackageBytesStorage {
    pub fn get(&self, path: impl AsRef<str>) -> Option<egui::load::Bytes> {
        let path = path.as_ref();
        self.cache.as_ref().get(path).map(|r| r.value().clone())
    }

    pub fn insert<'id>(
        &self,
        id: &'id ResourceId,
        package: &Package,
        bytes: Arc<[u8]>,
    ) -> Option<&'id str> {
        if !matches!(id, ResourceId::Image(..)) {
            return None;
        }

        let path = format!("{}/{}", package.id, id.path());
        self.cache.as_ref().insert(path.clone(), egui::load::Bytes::Shared(bytes));

        log::info!("Cached new resource: {path}");

        Some(id.name())
    }
}

/// [`egui::load::BytesLoader`] implementation for [`SharedPackageBytesStorage`].
pub struct EguiPackageBytesLoader(SharedPackageBytesStorage);

impl EguiPackageBytesLoader {
    pub fn new(storage: &SharedPackageBytesStorage) -> Self {
        Self(storage.clone())
    }
}

impl egui::load::BytesLoader for EguiPackageBytesLoader {
    fn id(&self) -> &str {
        egui::load::generate_loader_id!(PackageBytesLoader)
    }

    fn load(&self, _ctx: &egui::Context, uri: &str) -> egui::load::BytesLoadResult {
        let Some(path) = uri.strip_prefix("package://") else {
            return Err(egui::load::LoadError::NotSupported);
        };

        let Some(bytes) = self.0.get(path) else {
            return Err(egui::load::LoadError::Loading(format!(
                "Package image for '{path}' isn't loaded into app's cache!"
            )));
        };

        Ok(egui::load::BytesPoll::Ready { size: None, bytes, mime: None })
    }

    fn forget(&self, _uri: &str) {}

    fn forget_all(&self) {}

    fn byte_size(&self) -> usize {
        0
    }
}
