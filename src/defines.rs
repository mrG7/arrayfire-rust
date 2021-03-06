use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AfError {
    ///
    /// The function returned successfully
    ///
    SUCCESS            =   0,
    // 100-199 Errors in environment
    ///
    /// The system or device ran out of memory
    ///
    ERR_NO_MEM         = 101,
    ///
    /// There was an error in the device driver
    ///
    ERR_DRIVER         = 102,
    ///
    /// There was an error with the runtime environment
    ///
    ERR_RUNTIME        = 103,
    // 200-299 Errors in input parameters
    ///
    /// The input array is not a valid af_array object
    ///
    ERR_INVALID_ARRAY  = 201,
    ///
    /// One of the function arguments is incorrect
    ///
    ERR_ARG            = 202,
    ///
    /// The size is incorrect
    ///
    ERR_SIZE           = 203,
    ///
    /// The type is not suppported by this function
    ///
    ERR_TYPE           = 204,
    ///
    /// The type of the input arrays are not compatible
    ///
    ERR_DIFF_TYPE      = 205,
    ///
    /// Function does not support GFOR / batch mode
    ///
    ERR_BATCH          = 207,
    // 300-399 Errors for missing software features
    ///
    /// The option is not supported
    ///
    ERR_NOT_SUPPORTED  = 301,
    ///
    /// This build of ArrayFire does not support this feature
    ///
    ERR_NOT_CONFIGURED = 302,
    // 400-499 Errors for missing hardware features
    ///
    /// This device does not support double
    ///
    ERR_NO_DBL         = 401,
    ///
    /// This build of ArrayFire was not built with graphics or this device does
    /// not support graphics
    ///
    ERR_NO_GFX         = 402,
    // 900-999 Errors from upstream libraries and runtimes
    ///
    /// There was an internal error either in ArrayFire or in a project
    /// upstream
    ///
    ERR_INTERNAL       = 998,
    ///
    /// Unknown Error
    ///
    ERR_UNKNOWN        = 999
}

impl Display for AfError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.description())
    }
}

impl Error for AfError {
    fn description(&self) -> &str {
        match *self {
            AfError::SUCCESS => "Function returned successfully",
            AfError::ERR_NO_MEM => "The system or device ran out of memory",
            AfError::ERR_DRIVER => "Device driver error",
            AfError::ERR_RUNTIME => "Error in runtime environment",
            AfError::ERR_INVALID_ARRAY => "Input is not a valid Array Object",
            AfError::ERR_ARG => "One of the function arguments is incorrect",
            AfError::ERR_SIZE => "The size is incorrect",
            AfError::ERR_TYPE => "The type is not supported by this function",
            AfError::ERR_DIFF_TYPE => "The type of input arrays are not compatible",
            AfError::ERR_BATCH => "Function does not support GFOR / batch mode",
            AfError::ERR_NOT_SUPPORTED => "The option is not supported",
            AfError::ERR_NOT_CONFIGURED => "This build of ArrayFire does not support this feature",
            AfError::ERR_NO_DBL => "This device does not support double",
            AfError::ERR_NO_GFX => "This build of ArrayFire was not built with graphics or this device does not support graphics",
            AfError::ERR_INTERNAL => "There was an internal error in either ArrayFire or upstream project",
            AfError::ERR_UNKNOWN => "Unkown Error",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Aftype {
    F32 = 0,
    C32 = 1,
    F64 = 2,
    C64 = 3,
    B8  = 4,
    S32 = 5,
    U32 = 6,
    U8  = 7,
    S64 = 8,
    U64 = 9,
}

#[derive(Copy, Clone)]
pub enum InterpType {
    NEAREST = 0,
    LINEAR  = 1,
    BILINEAR= 2,
    CUBIC   = 3,
}

#[derive(Copy, Clone)]
pub enum BorderType {
    ZERO = 0,
    SYMMETRIC = 1,
}

#[derive(Copy, Clone)]
pub enum Connectivity {
    FOUR = 4,
    EIGHT = 8
}

#[derive(Copy, Clone)]
pub enum ConvMode {
    DEFAULT = 0,
    EXPAND  = 1,
}

#[derive(Copy, Clone)]
pub enum ConvDomain {
    AUTO     = 0,
    SPATIAL  = 1,
    FREQUENCY= 2,
}

#[derive(Copy, Clone)]
pub enum MatchType {
    SAD = 0,
    ZSAD= 1,
    LSAD= 2,
    SSD = 3,
    ZSSD= 4,
    LSSD= 5,
    NCC = 6,
    ZNCC= 7,
    SHD = 8,
}

#[derive(Copy, Clone)]
pub enum ColorSpace {
    GRAY = 0,
    RGB  = 1,
    HSV  = 2,
}

#[derive(Copy, Clone)]
pub enum MatProp {
    NONE,
    TRANS,
    CTRANS,
    UPPER,
    LOWER,
    DIAGUNIT,
    SYM,
    POSDEF,
    ORTHOG,
    TRIDIAG,
    BLOCKDIAG,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum NormType {
    VECTOR_1    = 0,
    VECTOR_INF  = 1,
    VECTOR_2    = 2,
    VECTOR_P    = 3,
    MATRIX_1    = 4,
    MATRIX_INF  = 5,
    MATRIX_2    = 6,
    MATRIX_L_PQ = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ColorMap {
    DEFAULT = 0,
    SPECTRUM= 1,
    COLORS  = 2,
    RED     = 3,
    MOOD    = 4,
    HEAT    = 5,
    BLUE    = 6,
}
