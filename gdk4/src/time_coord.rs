// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AxisFlags;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkTimeCoord")]
    pub struct TimeCoord(BoxedInline<ffi::GdkTimeCoord>);
}

impl TimeCoord {
    pub fn new(time: u32, axes: [f64; 12], flags: AxisFlags) -> Self {
        assert_initialized_main_thread!();
        Self(ffi::GdkTimeCoord {
            time,
            axes,
            flags: flags.into_glib(),
        })
    }

    pub fn time(&self) -> u32 {
        self.0.time
    }

    pub fn axes(&self) -> &[f64; 12] {
        &self.0.axes
    }

    pub fn flags(&self) -> AxisFlags {
        unsafe { from_glib(self.0.flags) }
    }
}

impl fmt::Debug for TimeCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TimeCoord")
            .field("time", &self.time())
            .field("axes", &self.axes())
            .field("flags", &self.flags())
            .finish()
    }
}
