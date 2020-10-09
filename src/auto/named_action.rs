// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use gtk_sys;
use std::fmt;
use ShortcutAction;

glib_wrapper! {
    pub struct NamedAction(Object<gtk_sys::GtkNamedAction, gtk_sys::GtkNamedActionClass, NamedActionClass>) @extends ShortcutAction;

    match fn {
        get_type => || gtk_sys::gtk_named_action_get_type(),
    }
}

impl NamedAction {
    pub fn new(name: &str) -> NamedAction {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_named_action_new(name.to_glib_none().0)) }
    }

    pub fn get_action_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_named_action_get_action_name(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for NamedAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NamedAction")
    }
}
