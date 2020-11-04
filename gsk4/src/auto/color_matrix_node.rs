// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use graphene;
use gsk_sys;
use std::fmt;
use RenderNode;

glib_wrapper! {
    pub struct ColorMatrixNode(Object<gsk_sys::GskColorMatrixNode, ColorMatrixNodeClass>) @extends RenderNode;

    match fn {
        get_type => || gsk_sys::gsk_color_matrix_node_get_type(),
    }
}

impl ColorMatrixNode {
    pub fn new<P: IsA<RenderNode>>(
        child: &P,
        color_matrix: &graphene::Matrix,
        color_offset: &graphene::Vec4,
    ) -> ColorMatrixNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gsk_sys::gsk_color_matrix_node_new(
                child.as_ref().to_glib_none().0,
                color_matrix.to_glib_none().0,
                color_offset.to_glib_none().0,
            ))
        }
    }

    pub fn get_child(&self) -> Option<RenderNode> {
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_get_child(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn peek_color_matrix(&self) -> Option<graphene::Matrix> {
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_peek_color_matrix(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn peek_color_offset(&self) -> Option<graphene::Vec4> {
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_peek_color_offset(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ColorMatrixNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorMatrixNode")
    }
}
