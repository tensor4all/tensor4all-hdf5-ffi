use hdf5_rt as hdf5;

#[test]
fn roundtrip_compound_type() {
    use hdf5::types::{CompoundField, CompoundType, TypeDescriptor};
    use hdf5::H5Type;

    #[repr(C)]
    struct Compound {
        a: u8,
        b: u8,
    }

    // Manual H5Type implementation since we don't have hdf5_derive
    unsafe impl H5Type for Compound {
        fn type_descriptor() -> TypeDescriptor {
            TypeDescriptor::Compound(CompoundType {
                fields: vec![
                    CompoundField {
                        name: "a".to_string(),
                        ty: u8::type_descriptor(),
                        offset: 0,
                        index: 0,
                    },
                    CompoundField {
                        name: "b".to_string(),
                        ty: u8::type_descriptor(),
                        offset: 1,
                        index: 1,
                    },
                ],
                size: std::mem::size_of::<Compound>(),
            })
        }
    }

    let dt = hdf5::Datatype::from_type::<Compound>().unwrap();
    let td = dt.to_descriptor().unwrap();
    assert_eq!(td, Compound::type_descriptor());
}
