use opensi_core::Package;

pub fn package_tab(package: &mut Package, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.label(&package.id);
        ui.label(package.name.clone().unwrap_or_default());
    });
}
