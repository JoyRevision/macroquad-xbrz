// zBRZ Texture Scaling for use with Macroquad
use libc::*;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum _ColorFormat {
    RGB,
    ARGB,
    ARGB_UNBUFFERED,
}

#[link(name = "./xbrz")]
#[allow(non_snake_case)]
extern "C" {
    pub fn XbrzScale(
        factor: usize,
        src: *mut u8,
        trg: *mut u8,
        srcWidth: c_int,
        srcHeight: c_int,
        colFmt: _ColorFormat,
    ) -> c_void;
}

pub fn scale(factor: usize, source: &mut Vec<u8>, width: u16, height: u16) -> Vec<u8> {
    let len: usize = factor * factor;
    let mut output_bytes: Vec<u8> = vec![0; source.len() * len];

    unsafe {
        let fmt = _ColorFormat::ARGB;
        XbrzScale(
            factor as usize,
            source.as_mut_ptr(),
            output_bytes.as_mut_ptr(),
            width as i32,
            height as i32,
            fmt,
        );
    }

    return output_bytes;
}
