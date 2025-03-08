
extern "C" {

    fn curve_fit_cubic_to_points_refit_fl(
        points: *const std::ffi::c_float,
        points_len: std::ffi::c_uint,
        dims: std::ffi::c_uint,
        error_threshold: std::ffi::c_float,
        calc_flag: std::ffi::c_uint,
        corners: *mut std::ffi::c_uint,
        corners_len: std::ffi::c_uint,
        corner_angle: std::ffi::c_float,
        
        r_cubic_array: *mut *mut std::ffi::c_float,
        r_cubic_array_len: *mut std::ffi::c_uint,
        r_cubic_orig_index: *mut *mut std::ffi::c_uint,
        r_corner_index_array: *mut *mut std::ffi::c_uint,
        r_corner_index_len: *mut std::ffi::c_uint
    ) -> std::ffi::c_int;

    fn free(ptr: *mut std::ffi::c_void);

}

pub fn fit_curve(dims: i32, points: &[f32], err: f32) -> Vec<f32> {
    unsafe {
        let mut r_cubic_array: *mut f32 = std::ptr::null_mut();
        let mut r_cubic_array_len: u32 = 0;
        curve_fit_cubic_to_points_refit_fl(
            points.as_ptr(),
            (points.len() / (dims as usize)) as u32,
            dims as u32,
            err, 
            0,
            std::ptr::null_mut(),
            0,
            std::f32::consts::PI * 5.0,
            &mut r_cubic_array as *mut *mut f32,
            &mut r_cubic_array_len as &mut u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut());
        
        let mut res = Vec::new();
        for i in 0..(r_cubic_array_len * 3) {
            for j in 0..dims {
                res.push(*r_cubic_array.add((i * (dims as u32) + (j as u32)) as usize));
            }
        }
        free(r_cubic_array as *mut std::ffi::c_void);
        res
    }
}
