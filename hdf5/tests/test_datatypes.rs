#[macro_use]
mod common;

use hdf5::sys::h5::H5I_INVALID_HID;
use hdf5::types::{TypeDescriptor as TD, *};
use hdf5::{from_id, Datatype, H5Type};
use hdf5_rt as hdf5;
use pretty_assertions::{assert_eq, assert_str_eq};

macro_rules! check_roundtrip {
    ($ty:ty, $desc:expr) => {{
        let desc = <$ty as H5Type>::type_descriptor();
        assert_eq!(desc, $desc);
        let dt = Datatype::from_type::<$ty>().unwrap();
        assert_eq!(desc, dt.to_descriptor().unwrap());
        assert_eq!(dt.size(), desc.size());
    }};
}

#[test]
pub fn test_datatype_roundtrip_primitives() {
    check_roundtrip!(i8, TD::Integer(IntSize::U1));
    check_roundtrip!(i16, TD::Integer(IntSize::U2));
    check_roundtrip!(i32, TD::Integer(IntSize::U4));
    check_roundtrip!(i64, TD::Integer(IntSize::U8));
    check_roundtrip!(u8, TD::Unsigned(IntSize::U1));
    check_roundtrip!(u16, TD::Unsigned(IntSize::U2));
    check_roundtrip!(u32, TD::Unsigned(IntSize::U4));
    check_roundtrip!(u64, TD::Unsigned(IntSize::U8));
    #[cfg(feature = "f16")]
    check_roundtrip!(::half::f16, TD::Float(FloatSize::U2));
    check_roundtrip!(f32, TD::Float(FloatSize::U4));
    check_roundtrip!(f64, TD::Float(FloatSize::U8));
    check_roundtrip!(bool, TD::Boolean);
    check_roundtrip!([bool; 5], TD::FixedArray(Box::new(TD::Boolean), 5));
    check_roundtrip!(VarLenArray<bool>, TD::VarLenArray(Box::new(TD::Boolean)));
    check_roundtrip!(FixedAscii<5>, TD::FixedAscii(5));
    check_roundtrip!(FixedUnicode<5>, TD::FixedUnicode(5));
    check_roundtrip!(VarLenAscii, TD::VarLenAscii);
    check_roundtrip!(VarLenUnicode, TD::VarLenUnicode);
}

// Note: test_datatype_roundtrip for custom enums/structs removed - requires hdf5_derive

#[test]
pub fn test_invalid_datatype() {
    assert_err!(from_id::<Datatype>(H5I_INVALID_HID), "Invalid handle id");
}

#[test]
pub fn test_eq() {
    assert_eq!(Datatype::from_type::<u32>().unwrap(), Datatype::from_type::<u32>().unwrap());
    assert_ne!(Datatype::from_type::<u16>().unwrap(), Datatype::from_type::<u32>().unwrap());
}

#[test]
fn test_print_display_debug_datatype_bool() {
    let dt = Datatype::from_type::<bool>().unwrap();

    assert_str_eq!(format!("{dt}"), "bool");
    assert_str_eq!(format!("{dt:?}"), "<HDF5 datatype: bool>");
    assert_str_eq!(format!("{dt:#?}"), "<HDF5 datatype: bool>");
}

#[test]
fn test_print_display_debug_datatype_f64() {
    let dt = Datatype::from_type::<f64>().unwrap();

    assert_str_eq!(format!("{dt}"), "float64");
    assert_str_eq!(format!("{dt:?}"), "<HDF5 datatype: float64>");
    assert_str_eq!(format!("{dt:#?}"), "<HDF5 datatype: float64>");
}

// Note: test_print_display_debug_datatype_color_enum removed - requires hdf5_derive

#[test]
fn test_print_display_debug_datatype_var_len_unicode() {
    let dt = Datatype::from_type::<VarLenUnicode>().unwrap();
    assert!(dt.is::<VarLenUnicode>());

    assert_eq!(dt.to_descriptor().unwrap(), TD::VarLenUnicode);

    assert_str_eq!(format!("{dt}"), "unicode (var len)");
    assert_str_eq!(format!("{dt:?}"), "<HDF5 datatype: unicode (var len)>");
    assert_str_eq!(format!("{dt:#?}"), "<HDF5 datatype: unicode (var len)>");
}

#[test]
fn test_print_display_debug_datatype_fixed_len_unicode() {
    const SIZE: usize = 10;
    let dt = Datatype::from_type::<FixedUnicode<SIZE>>().unwrap();
    assert!(dt.is::<FixedUnicode<SIZE>>());

    assert_eq!(dt.to_descriptor().unwrap(), TD::FixedUnicode(SIZE));

    assert_str_eq!(format!("{dt}"), "unicode (len 10)");
    assert_str_eq!(format!("{dt:?}"), "<HDF5 datatype: unicode (len 10)>");
    assert_str_eq!(format!("{dt:#?}"), "<HDF5 datatype: unicode (len 10)>");
}
