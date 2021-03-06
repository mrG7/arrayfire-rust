extern crate libc;

use array::Array;
use defines::AfError;
use defines::InterpType;
use defines::ConvMode;
use defines::ConvDomain;
use self::libc::{uint8_t, c_int, c_float, c_double, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_approx1(out: MutAfArray, inp: AfArray, pos: AfArray,
                  method: c_int, off_grid: c_float) -> c_int;

    fn af_approx2(out: MutAfArray, inp: AfArray, pos0: AfArray, pos1: AfArray,
                  method: c_int, off_grid: c_float) -> c_int;

    fn af_fft(out: MutAfArray, arr: AfArray,
              nfac: c_double, odim0: c_longlong) -> c_int;

    fn af_fft2(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong) -> c_int;

    fn af_fft3(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong, odim2: c_longlong) -> c_int;

    fn af_ifft(out: MutAfArray, arr: AfArray,
               nfac: c_double, odim0: c_longlong) -> c_int;

    fn af_ifft2(out: MutAfArray, arr: AfArray, nfac: c_double,
                odim0: c_longlong, odim1: c_longlong) -> c_int;

    fn af_ifft3(out: MutAfArray, arr: AfArray, nfac: c_double,
                odim0: c_longlong, odim1: c_longlong, odim2: c_longlong) -> c_int;

    fn af_convolve1(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve2(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve3(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve2_sep(o: MutAfArray, c: AfArray, r: AfArray, s: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve1(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve2(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve3(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fir(out: MutAfArray, b: AfArray, x: AfArray) -> c_int;
    fn af_iir(out: MutAfArray, b: AfArray, a: AfArray, x: AfArray) -> c_int;
}

#[allow(unused_mut)]
pub fn approx1(input: &Array, pos: &Array,
               method: InterpType, off_grid: f32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_approx1(&mut temp as MutAfArray, input.get() as AfArray,
                                 pos.get() as AfArray, method as c_int, off_grid as c_float);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn approx2(input: &Array, pos0: &Array, pos1: &Array,
               method: InterpType, off_grid: f32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_approx2(&mut temp as MutAfArray, input.get() as AfArray,
                                 pos0.get() as AfArray, pos1.get() as AfArray,
                                 method as c_int, off_grid as c_float);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn fft(input: &Array, norm_factor: f64, odim0: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft(&mut temp as MutAfArray,
                             input.get() as AfArray, norm_factor as c_double,
                             odim0 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn fft2(input: &Array, norm_factor: f64, odim0: i64, odim1: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft2(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong, odim1 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn fft3(input: &Array, norm_factor: f64,
            odim0: i64, odim1: i64, odim2: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft3(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong, odim1 as c_longlong, odim2 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn ifft(input: &Array, norm_factor: f64, odim0: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_ifft(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn ifft2(input: &Array, norm_factor: f64, odim0: i64, odim1: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_ifft2(&mut temp as MutAfArray,
                               input.get() as AfArray, norm_factor as c_double,
                               odim0 as c_longlong, odim1 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn ifft3(input: &Array, norm_factor: f64,
             odim0: i64, odim1: i64, odim2: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_ifft3(&mut temp as MutAfArray,
                               input.get() as AfArray, norm_factor as c_double,
                               odim0 as c_longlong, odim1 as c_longlong, odim2 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! conv_func_def {
    ($fn_name:ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(signal: &Array, filter: &Array,
                        mode: ConvMode, domain: ConvDomain) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        signal.get() as AfArray, filter.get() as AfArray,
                                        mode as uint8_t, domain as uint8_t);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

conv_func_def!(convolve1, af_convolve1);
conv_func_def!(convolve2, af_convolve2);
conv_func_def!(convolve3, af_convolve3);

#[allow(unused_mut)]
pub fn convolve2_sep(cfilt: &Array, rfilt: &Array, signal: &Array,
                     mode: ConvMode) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_convolve2_sep(&mut temp as MutAfArray,
                                       cfilt.get() as AfArray, rfilt.get() as AfArray,
                                       signal.get() as AfArray, mode as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! fft_conv_func_def {
    ($fn_name:ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(signal: &Array, filter: &Array,
                        mode: ConvMode) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, signal.get() as AfArray,
                                        filter.get() as AfArray, mode as uint8_t);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

fft_conv_func_def!(fft_convolve1, af_fft_convolve1);
fft_conv_func_def!(fft_convolve2, af_fft_convolve2);
fft_conv_func_def!(fft_convolve3, af_fft_convolve3);

#[allow(unused_mut)]
pub fn fir(b: &Array, x: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fir(&mut temp as MutAfArray, b.get() as AfArray, x.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn iir(b: &Array, a: &Array, x: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_iir(&mut temp as MutAfArray,
                             b.get() as AfArray, a.get() as AfArray, x.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}
