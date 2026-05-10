use std::{rc::Rc, sync::Arc};

use crate::render::traits::*;

// impl<T: ?Sized + BufferPointers> BufferPointers for Arc<T> {
//     fn pointer(&self) -> *const u32 {
//         (**self).pointer()
//     }
//     //#[deprecated = "You cannot get a mutable pointer from Arc"]
//     fn mut_pointer(&mut self) -> *mut u32 {
//         panic!("Cannot get mutable pointer from Arc")
//     }
// }

impl<T: ?Sized + BufferMetrics> BufferMetrics for Arc<T> {
    fn width(&self) -> usize {
        (**self).width()
    }

    fn height(&self) -> usize {
        (**self).height()
    }
}
impl<T: ?Sized + BufferGetPixel> BufferGetPixel for Arc<T> {
    fn get_pixel(&self, xy: (usize, usize)) -> u32 {
        (**self).get_pixel(xy)
    }

    fn get_pixel_option(&self, xy: (usize, usize)) -> Option<u32> {
        (**self).get_pixel_option(xy)
    }

    unsafe fn get_pixel_unchecked(&self, xy: (usize, usize)) -> u32 {
        unsafe { (**self).get_pixel_unchecked(xy) }
    }
}

impl<T: ?Sized + BufferMetrics> BufferMetrics for Box<T> {
    fn width(&self) -> usize {
        (**self).width()
    }

    fn height(&self) -> usize {
        (**self).height()
    }
}

impl<T: ?Sized + BufferPointers> BufferPointers for Box<T> {
    fn pointer(&self) -> *const u32 {
        (**self).pointer()
    }

    fn mut_pointer(&mut self) -> *mut u32 {
        (**self).mut_pointer()
    }
}

impl<T: ?Sized + BufferGetPixel> BufferGetPixel for Box<T> {
    fn get_pixel(&self, xy: (usize, usize)) -> u32 {
        (**self).get_pixel(xy)
    }

    fn get_pixel_option(&self, xy: (usize, usize)) -> Option<u32> {
        (**self).get_pixel_option(xy)
    }

    unsafe fn get_pixel_unchecked(&self, xy: (usize, usize)) -> u32 {
        unsafe { (**self).get_pixel_unchecked(xy) }
    }
}

impl<T: ?Sized + BufferMetrics> BufferMetrics for Rc<T> {
    fn width(&self) -> usize {
        (**self).width()
    }

    fn height(&self) -> usize {
        (**self).height()
    }
}

// impl<T: ?Sized + BufferPointers> BufferPointers for Rc<T> {
//     fn pointer(&self) -> *const u32 {
//         (**self).pointer()
//     }

//     fn mut_pointer(&mut self) -> *mut u32 {
//         panic!("Cannot get mutable pointer from Arc")
//     }
// }

impl<T: ?Sized + BufferGetPixel> BufferGetPixel for Rc<T> {
    fn get_pixel(&self, xy: (usize, usize)) -> u32 {
        (**self).get_pixel(xy)
    }

    fn get_pixel_option(&self, xy: (usize, usize)) -> Option<u32> {
        (**self).get_pixel_option(xy)
    }

    unsafe fn get_pixel_unchecked(&self, xy: (usize, usize)) -> u32 {
        unsafe { (**self).get_pixel_unchecked(xy) }
    }
}
