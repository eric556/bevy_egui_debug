use std::fmt::{Display};
use bevy::math::{XY, XYZ, XYZW};
use bevy_egui::egui::{self, Ui};
pub use derive::*;

pub trait EguiDebug {
    fn debug(&self, ui: &mut Ui);
    fn debug_mut(&mut self, ui: &mut Ui);
}

#[macro_export]
macro_rules! default_debug {
    ($type:ty) => {
        fn debug(&self, ui: &mut Ui) {
            ui.label(format!("{:?}", self));
        }
    }
}

macro_rules! impl_egui_debug_nums {
    ($type:ty) => {
        impl EguiDebug for $type {
            default_debug!($type);

            fn debug_mut(&mut self, ui: &mut Ui) {
                ui.add(egui::DragValue::new(self));
            }
        }
    };
}

impl_egui_debug_nums!(f32);
impl_egui_debug_nums!(f64);
impl_egui_debug_nums!(i8);
impl_egui_debug_nums!(u8);
impl_egui_debug_nums!(i16);
impl_egui_debug_nums!(u16);
impl_egui_debug_nums!(i32);
impl_egui_debug_nums!(u32);
impl_egui_debug_nums!(i64);
impl_egui_debug_nums!(u64);
impl_egui_debug_nums!(isize);
impl_egui_debug_nums!(usize);

impl EguiDebug for bool {
    default_debug!(bool);

    fn debug_mut(&mut self, ui: &mut Ui) {
        ui.checkbox(self, "");
    }
}

impl<T> EguiDebug for XY<T> where T: Display + EguiDebug {
    fn debug(&self, ui: &mut Ui) {
        ui.label(format!("{}, {}", self.x, self.y));
    }

    fn debug_mut(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.x.debug_mut(ui);
            self.y.debug_mut(ui);
        });
    }
}

impl<T> EguiDebug for XYZ<T> where T: Display + EguiDebug {
    fn debug(&self, ui: &mut Ui) {
        ui.label(format!("{}, {}, {}", self.x, self.y, self.z));
    }

    fn debug_mut(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.x.debug_mut(ui);
            self.y.debug_mut(ui);
            self.z.debug_mut(ui);
        });
    }
}

impl<T> EguiDebug for XYZW<T> where T: Display + EguiDebug {
    fn debug(&self, ui: &mut Ui) {
        ui.label(format!("{}, {}, {}, {}", self.x, self.y, self.z, self.w));
    }

    fn debug_mut(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.x.debug_mut(ui);
            self.y.debug_mut(ui);
            self.z.debug_mut(ui);
            self.w.debug_mut(ui);
        });
    }
}

pub fn debug_with_label<T: EguiDebug>(ui: &mut Ui, label: &str, item: &T) {
    ui.horizontal(|ui|{
        ui.label(label);
        item.debug(ui);
    });
}

pub fn debug_with_label_mut<T: EguiDebug>(ui: &mut Ui, label: &str, item: &mut T) {
    ui.horizontal(|ui|{
        ui.label(label);
        item.debug_mut(ui);
    });
}