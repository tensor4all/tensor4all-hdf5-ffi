# Runtime-Loading Feature Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the `runtime-loading` feature by adding ~230 missing symbols so the crate compiles with `--no-default-features --features "runtime-loading,complex"`.

**Architecture:** Add missing H5T type constants, H5E error constants, H5P property list constants, and H5T functions to `runtime.rs`, update exports in `sys/mod.rs`, and modify `globals.rs` for dual-mode support.

**Tech Stack:** Rust, HDF5 FFI, libloading, paste macro

---

## Task 1: Add H5T Type Constants to runtime.rs

**Files:**
- Modify: `hdf5/src/sys/runtime.rs` (after line 1836)

**Step 1: Add bitfield type constants**

Add after the existing `H5T_STD_U64LE` definition (around line 1836):

```rust
// Bitfield types
define_native_type!(H5T_STD_B8BE, "H5T_STD_B8BE_g");
define_native_type!(H5T_STD_B8LE, "H5T_STD_B8LE_g");
define_native_type!(H5T_STD_B16BE, "H5T_STD_B16BE_g");
define_native_type!(H5T_STD_B16LE, "H5T_STD_B16LE_g");
define_native_type!(H5T_STD_B32BE, "H5T_STD_B32BE_g");
define_native_type!(H5T_STD_B32LE, "H5T_STD_B32LE_g");
define_native_type!(H5T_STD_B64BE, "H5T_STD_B64BE_g");
define_native_type!(H5T_STD_B64LE, "H5T_STD_B64LE_g");

// Reference type
define_native_type!(H5T_STD_REF_DSETREG, "H5T_STD_REF_DSETREG_g");

// Time types
define_native_type!(H5T_UNIX_D32BE, "H5T_UNIX_D32BE_g");
define_native_type!(H5T_UNIX_D32LE, "H5T_UNIX_D32LE_g");
define_native_type!(H5T_UNIX_D64BE, "H5T_UNIX_D64BE_g");
define_native_type!(H5T_UNIX_D64LE, "H5T_UNIX_D64LE_g");

// String/VAX types
define_native_type!(H5T_FORTRAN_S1, "H5T_FORTRAN_S1_g");
define_native_type!(H5T_VAX_F32, "H5T_VAX_F32_g");
define_native_type!(H5T_VAX_F64, "H5T_VAX_F64_g");

// Additional native types
define_native_type!(H5T_NATIVE_SCHAR, "H5T_NATIVE_SCHAR_g");
define_native_type!(H5T_NATIVE_UCHAR, "H5T_NATIVE_UCHAR_g");
define_native_type!(H5T_NATIVE_SHORT, "H5T_NATIVE_SHORT_g");
define_native_type!(H5T_NATIVE_USHORT, "H5T_NATIVE_USHORT_g");
define_native_type!(H5T_NATIVE_INT, "H5T_NATIVE_INT_g");
define_native_type!(H5T_NATIVE_UINT, "H5T_NATIVE_UINT_g");
define_native_type!(H5T_NATIVE_LONG, "H5T_NATIVE_LONG_g");
define_native_type!(H5T_NATIVE_ULONG, "H5T_NATIVE_ULONG_g");
define_native_type!(H5T_NATIVE_LLONG, "H5T_NATIVE_LLONG_g");
define_native_type!(H5T_NATIVE_ULLONG, "H5T_NATIVE_ULLONG_g");
define_native_type!(H5T_NATIVE_LDOUBLE, "H5T_NATIVE_LDOUBLE_g");
define_native_type!(H5T_NATIVE_B8, "H5T_NATIVE_B8_g");
define_native_type!(H5T_NATIVE_B16, "H5T_NATIVE_B16_g");
define_native_type!(H5T_NATIVE_B32, "H5T_NATIVE_B32_g");
define_native_type!(H5T_NATIVE_B64, "H5T_NATIVE_B64_g");
define_native_type!(H5T_NATIVE_OPAQUE, "H5T_NATIVE_OPAQUE_g");
define_native_type!(H5T_NATIVE_HADDR, "H5T_NATIVE_HADDR_g");
define_native_type!(H5T_NATIVE_HSIZE, "H5T_NATIVE_HSIZE_g");
define_native_type!(H5T_NATIVE_HSSIZE, "H5T_NATIVE_HSSIZE_g");
define_native_type!(H5T_NATIVE_HERR, "H5T_NATIVE_HERR_g");
define_native_type!(H5T_NATIVE_HBOOL, "H5T_NATIVE_HBOOL_g");
define_native_type!(H5T_NATIVE_INT_LEAST8, "H5T_NATIVE_INT_LEAST8_g");
define_native_type!(H5T_NATIVE_UINT_LEAST8, "H5T_NATIVE_UINT_LEAST8_g");
define_native_type!(H5T_NATIVE_INT_FAST8, "H5T_NATIVE_INT_FAST8_g");
define_native_type!(H5T_NATIVE_UINT_FAST8, "H5T_NATIVE_UINT_FAST8_g");
define_native_type!(H5T_NATIVE_INT_LEAST16, "H5T_NATIVE_INT_LEAST16_g");
define_native_type!(H5T_NATIVE_UINT_LEAST16, "H5T_NATIVE_UINT_LEAST16_g");
define_native_type!(H5T_NATIVE_INT_FAST16, "H5T_NATIVE_INT_FAST16_g");
define_native_type!(H5T_NATIVE_UINT_FAST16, "H5T_NATIVE_UINT_FAST16_g");
define_native_type!(H5T_NATIVE_INT_LEAST32, "H5T_NATIVE_INT_LEAST32_g");
define_native_type!(H5T_NATIVE_UINT_LEAST32, "H5T_NATIVE_UINT_LEAST32_g");
define_native_type!(H5T_NATIVE_INT_FAST32, "H5T_NATIVE_INT_FAST32_g");
define_native_type!(H5T_NATIVE_UINT_FAST32, "H5T_NATIVE_UINT_FAST32_g");
define_native_type!(H5T_NATIVE_INT_LEAST64, "H5T_NATIVE_INT_LEAST64_g");
define_native_type!(H5T_NATIVE_UINT_LEAST64, "H5T_NATIVE_UINT_LEAST64_g");
define_native_type!(H5T_NATIVE_INT_FAST64, "H5T_NATIVE_INT_FAST64_g");
define_native_type!(H5T_NATIVE_UINT_FAST64, "H5T_NATIVE_UINT_FAST64_g");
```

**Step 2: Verify no syntax errors**

Run: `cargo check -p tensor4all-hdf5-ffi --no-default-features --features "runtime-loading"`
Expected: Compilation proceeds (may still have other errors, but no errors in this section)

---

## Task 2: Add H5T Functions and Types to runtime.rs

**Files:**
- Modify: `hdf5/src/sys/runtime.rs` (in enums section ~line 450 and after type constants)

**Step 1: Add H5T_VARIABLE constant**

Add in the constants section (around line 50, after H5P_DEFAULT):

```rust
/// Variable-length type marker
pub const H5T_VARIABLE: size_t = !0usize;
```

**Step 2: Add H5T_cmd_t and H5T_bkg_t enums**

Add after the existing enums (around line 450):

```rust
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum H5T_cmd_t {
    H5T_CONV_INIT = 0,
    H5T_CONV_CONV = 1,
    H5T_CONV_FREE = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum H5T_bkg_t {
    H5T_BKG_NO = 0,
    H5T_BKG_TEMP = 1,
    H5T_BKG_YES = 2,
}
```

**Step 3: Add H5T_cdata_t struct**

Add after the enums:

```rust
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H5T_cdata_t {
    pub command: H5T_cmd_t,
    pub need_bkg: H5T_bkg_t,
    pub recalc: hbool_t,
    pub priv_: *mut c_void,
}
```

**Step 4: Add H5T_conv_t type and functions**

Add after H5T_cdata_t:

```rust
pub type H5T_conv_t = Option<
    unsafe extern "C" fn(
        src_id: hid_t,
        dst_id: hid_t,
        cdata: *mut H5T_cdata_t,
        nelmts: size_t,
        buf_stride: size_t,
        bkg_stride: size_t,
        buf: *mut c_void,
        bkg: *mut c_void,
        dxpl: hid_t,
    ) -> herr_t,
>;
```

**Step 5: Add H5Tfind and H5Tcompiler_conv functions**

Add in the H5T functions section (find existing `hdf5_function!` calls for H5T):

```rust
hdf5_function!(H5Tfind, fn(src_id: hid_t, dst_id: hid_t, pcdata: *mut *mut H5T_cdata_t) -> H5T_conv_t);
hdf5_function!(H5Tcompiler_conv, fn(src_id: hid_t, dst_id: hid_t) -> htri_t);
```

**Step 6: Verify no syntax errors**

Run: `cargo check -p tensor4all-hdf5-ffi --no-default-features --features "runtime-loading"`
Expected: Compilation proceeds

---

## Task 3: Add H5E Error Constants to runtime.rs

**Files:**
- Modify: `hdf5/src/sys/runtime.rs` (add new section after type constants)

**Step 1: Add all H5E error constants**

Add a new section after the type constants:

```rust
// =============================================================================
// Error class and message IDs (loaded at runtime)
// =============================================================================

// Major error classes
define_native_type!(H5E_ERR_CLS, "H5E_ERR_CLS_g");
define_native_type!(H5E_NONE_MAJOR, "H5E_NONE_MAJOR_g");
define_native_type!(H5E_ARGS, "H5E_ARGS_g");
define_native_type!(H5E_RESOURCE, "H5E_RESOURCE_g");
define_native_type!(H5E_INTERNAL, "H5E_INTERNAL_g");
define_native_type!(H5E_FILE, "H5E_FILE_g");
define_native_type!(H5E_IO, "H5E_IO_g");
define_native_type!(H5E_FUNC, "H5E_FUNC_g");
define_native_type!(H5E_ATOM, "H5E_ATOM_g");
define_native_type!(H5E_CACHE, "H5E_CACHE_g");
define_native_type!(H5E_LINK, "H5E_LINK_g");
define_native_type!(H5E_BTREE, "H5E_BTREE_g");
define_native_type!(H5E_SYM, "H5E_SYM_g");
define_native_type!(H5E_HEAP, "H5E_HEAP_g");
define_native_type!(H5E_OHDR, "H5E_OHDR_g");
define_native_type!(H5E_DATATYPE, "H5E_DATATYPE_g");
define_native_type!(H5E_DATASPACE, "H5E_DATASPACE_g");
define_native_type!(H5E_DATASET, "H5E_DATASET_g");
define_native_type!(H5E_STORAGE, "H5E_STORAGE_g");
define_native_type!(H5E_PLIST, "H5E_PLIST_g");
define_native_type!(H5E_ATTR, "H5E_ATTR_g");
define_native_type!(H5E_PLINE, "H5E_PLINE_g");
define_native_type!(H5E_EFL, "H5E_EFL_g");
define_native_type!(H5E_REFERENCE, "H5E_REFERENCE_g");
define_native_type!(H5E_VFL, "H5E_VFL_g");
define_native_type!(H5E_TST, "H5E_TST_g");
define_native_type!(H5E_RS, "H5E_RS_g");
define_native_type!(H5E_PLUGIN, "H5E_PLUGIN_g");
define_native_type!(H5E_SLIST, "H5E_SLIST_g");
define_native_type!(H5E_FSPACE, "H5E_FSPACE_g");
define_native_type!(H5E_SOHM, "H5E_SOHM_g");
define_native_type!(H5E_ERROR, "H5E_ERROR_g");
define_native_type!(H5E_PATH, "H5E_PATH_g");

// Minor error codes
define_native_type!(H5E_NONE_MINOR, "H5E_NONE_MINOR_g");
define_native_type!(H5E_ALIGNMENT, "H5E_ALIGNMENT_g");
define_native_type!(H5E_ALREADYEXISTS, "H5E_ALREADYEXISTS_g");
define_native_type!(H5E_ALREADYINIT, "H5E_ALREADYINIT_g");
define_native_type!(H5E_BADATOM, "H5E_BADATOM_g");
define_native_type!(H5E_BADFILE, "H5E_BADFILE_g");
define_native_type!(H5E_BADGROUP, "H5E_BADGROUP_g");
define_native_type!(H5E_BADITER, "H5E_BADITER_g");
define_native_type!(H5E_BADMESG, "H5E_BADMESG_g");
define_native_type!(H5E_BADRANGE, "H5E_BADRANGE_g");
define_native_type!(H5E_BADSELECT, "H5E_BADSELECT_g");
define_native_type!(H5E_BADSIZE, "H5E_BADSIZE_g");
define_native_type!(H5E_BADTYPE, "H5E_BADTYPE_g");
define_native_type!(H5E_BADVALUE, "H5E_BADVALUE_g");
define_native_type!(H5E_CALLBACK, "H5E_CALLBACK_g");
define_native_type!(H5E_CANAPPLY, "H5E_CANAPPLY_g");
define_native_type!(H5E_CANTALLOC, "H5E_CANTALLOC_g");
define_native_type!(H5E_CANTATTACH, "H5E_CANTATTACH_g");
define_native_type!(H5E_CANTCLIP, "H5E_CANTCLIP_g");
define_native_type!(H5E_CANTCLOSEFILE, "H5E_CANTCLOSEFILE_g");
define_native_type!(H5E_CANTCLOSEOBJ, "H5E_CANTCLOSEOBJ_g");
define_native_type!(H5E_CANTCOMPARE, "H5E_CANTCOMPARE_g");
define_native_type!(H5E_CANTCOMPUTE, "H5E_CANTCOMPUTE_g");
define_native_type!(H5E_CANTCONVERT, "H5E_CANTCONVERT_g");
define_native_type!(H5E_CANTCOPY, "H5E_CANTCOPY_g");
define_native_type!(H5E_CANTCOUNT, "H5E_CANTCOUNT_g");
define_native_type!(H5E_CANTCREATE, "H5E_CANTCREATE_g");
define_native_type!(H5E_CANTDEC, "H5E_CANTDEC_g");
define_native_type!(H5E_CANTDECODE, "H5E_CANTDECODE_g");
define_native_type!(H5E_CANTDELETE, "H5E_CANTDELETE_g");
define_native_type!(H5E_CANTDIRTY, "H5E_CANTDIRTY_g");
define_native_type!(H5E_CANTENCODE, "H5E_CANTENCODE_g");
define_native_type!(H5E_CANTEXPUNGE, "H5E_CANTEXPUNGE_g");
define_native_type!(H5E_CANTEXTEND, "H5E_CANTEXTEND_g");
define_native_type!(H5E_CANTFILTER, "H5E_CANTFILTER_g");
define_native_type!(H5E_CANTFLUSH, "H5E_CANTFLUSH_g");
define_native_type!(H5E_CANTFREE, "H5E_CANTFREE_g");
define_native_type!(H5E_CANTGC, "H5E_CANTGC_g");
define_native_type!(H5E_CANTGET, "H5E_CANTGET_g");
define_native_type!(H5E_CANTGETSIZE, "H5E_CANTGETSIZE_g");
define_native_type!(H5E_CANTINC, "H5E_CANTINC_g");
define_native_type!(H5E_CANTINIT, "H5E_CANTINIT_g");
define_native_type!(H5E_CANTINS, "H5E_CANTINS_g");
define_native_type!(H5E_CANTINSERT, "H5E_CANTINSERT_g");
define_native_type!(H5E_CANTLIST, "H5E_CANTLIST_g");
define_native_type!(H5E_CANTLOAD, "H5E_CANTLOAD_g");
define_native_type!(H5E_CANTLOCK, "H5E_CANTLOCK_g");
define_native_type!(H5E_CANTMARKDIRTY, "H5E_CANTMARKDIRTY_g");
define_native_type!(H5E_CANTMERGE, "H5E_CANTMERGE_g");
define_native_type!(H5E_CANTMODIFY, "H5E_CANTMODIFY_g");
define_native_type!(H5E_CANTMOVE, "H5E_CANTMOVE_g");
define_native_type!(H5E_CANTNEXT, "H5E_CANTNEXT_g");
define_native_type!(H5E_CANTOPENFILE, "H5E_CANTOPENFILE_g");
define_native_type!(H5E_CANTOPENOBJ, "H5E_CANTOPENOBJ_g");
define_native_type!(H5E_CANTOPERATE, "H5E_CANTOPERATE_g");
define_native_type!(H5E_CANTPACK, "H5E_CANTPACK_g");
define_native_type!(H5E_CANTPIN, "H5E_CANTPIN_g");
define_native_type!(H5E_CANTPROTECT, "H5E_CANTPROTECT_g");
define_native_type!(H5E_CANTRECV, "H5E_CANTRECV_g");
define_native_type!(H5E_CANTREDISTRIBUTE, "H5E_CANTREDISTRIBUTE_g");
define_native_type!(H5E_CANTREGISTER, "H5E_CANTREGISTER_g");
define_native_type!(H5E_CANTRELEASE, "H5E_CANTRELEASE_g");
define_native_type!(H5E_CANTREMOVE, "H5E_CANTREMOVE_g");
define_native_type!(H5E_CANTRENAME, "H5E_CANTRENAME_g");
define_native_type!(H5E_CANTRESET, "H5E_CANTRESET_g");
define_native_type!(H5E_CANTRESIZE, "H5E_CANTRESIZE_g");
define_native_type!(H5E_CANTRESTORE, "H5E_CANTRESTORE_g");
define_native_type!(H5E_CANTREVIVE, "H5E_CANTREVIVE_g");
define_native_type!(H5E_CANTSELECT, "H5E_CANTSELECT_g");
define_native_type!(H5E_CANTSERIALIZE, "H5E_CANTSERIALIZE_g");
define_native_type!(H5E_CANTSET, "H5E_CANTSET_g");
define_native_type!(H5E_CANTSHRINK, "H5E_CANTSHRINK_g");
define_native_type!(H5E_CANTSORT, "H5E_CANTSORT_g");
define_native_type!(H5E_CANTSPLIT, "H5E_CANTSPLIT_g");
define_native_type!(H5E_CANTSWAP, "H5E_CANTSWAP_g");
define_native_type!(H5E_CANTUNLOCK, "H5E_CANTUNLOCK_g");
define_native_type!(H5E_CANTUNPIN, "H5E_CANTUNPIN_g");
define_native_type!(H5E_CANTUNPROTECT, "H5E_CANTUNPROTECT_g");
define_native_type!(H5E_CANTUPDATE, "H5E_CANTUPDATE_g");
define_native_type!(H5E_CLOSEERROR, "H5E_CLOSEERROR_g");
define_native_type!(H5E_COMPLEN, "H5E_COMPLEN_g");
define_native_type!(H5E_DUPCLASS, "H5E_DUPCLASS_g");
define_native_type!(H5E_EXISTS, "H5E_EXISTS_g");
define_native_type!(H5E_FCNTL, "H5E_FCNTL_g");
define_native_type!(H5E_FILEEXISTS, "H5E_FILEEXISTS_g");
define_native_type!(H5E_FILEOPEN, "H5E_FILEOPEN_g");
define_native_type!(H5E_LINKCOUNT, "H5E_LINKCOUNT_g");
define_native_type!(H5E_MOUNT, "H5E_MOUNT_g");
define_native_type!(H5E_MPI, "H5E_MPI_g");
define_native_type!(H5E_MPIERRSTR, "H5E_MPIERRSTR_g");
define_native_type!(H5E_NLINKS, "H5E_NLINKS_g");
define_native_type!(H5E_NOENCODER, "H5E_NOENCODER_g");
define_native_type!(H5E_NOFILTER, "H5E_NOFILTER_g");
define_native_type!(H5E_NOIDS, "H5E_NOIDS_g");
define_native_type!(H5E_NOSPACE, "H5E_NOSPACE_g");
define_native_type!(H5E_NOTCACHED, "H5E_NOTCACHED_g");
define_native_type!(H5E_NOTFOUND, "H5E_NOTFOUND_g");
define_native_type!(H5E_NOTHDF5, "H5E_NOTHDF5_g");
define_native_type!(H5E_NOTREGISTERED, "H5E_NOTREGISTERED_g");
define_native_type!(H5E_OBJOPEN, "H5E_OBJOPEN_g");
define_native_type!(H5E_OPENERROR, "H5E_OPENERROR_g");
define_native_type!(H5E_OVERFLOW, "H5E_OVERFLOW_g");
define_native_type!(H5E_PROTECT, "H5E_PROTECT_g");
define_native_type!(H5E_READERROR, "H5E_READERROR_g");
define_native_type!(H5E_SEEKERROR, "H5E_SEEKERROR_g");
define_native_type!(H5E_SETDISALLOWED, "H5E_SETDISALLOWED_g");
define_native_type!(H5E_SETLOCAL, "H5E_SETLOCAL_g");
define_native_type!(H5E_SYSERRSTR, "H5E_SYSERRSTR_g");
define_native_type!(H5E_SYSTEM, "H5E_SYSTEM_g");
define_native_type!(H5E_TRAVERSE, "H5E_TRAVERSE_g");
define_native_type!(H5E_TRUNCATED, "H5E_TRUNCATED_g");
define_native_type!(H5E_UNINITIALIZED, "H5E_UNINITIALIZED_g");
define_native_type!(H5E_UNSUPPORTED, "H5E_UNSUPPORTED_g");
define_native_type!(H5E_VERSION, "H5E_VERSION_g");
define_native_type!(H5E_WRITEERROR, "H5E_WRITEERROR_g");
```

**Step 2: Verify no syntax errors**

Run: `cargo check -p tensor4all-hdf5-ffi --no-default-features --features "runtime-loading"`
Expected: Compilation proceeds

---

## Task 4: Add H5P Property List Constants to runtime.rs

**Files:**
- Modify: `hdf5/src/sys/runtime.rs` (add after error constants)

**Step 1: Add all H5P constants**

Add a new section:

```rust
// =============================================================================
// Property list class and default IDs (loaded at runtime)
// =============================================================================

// Property list classes
define_native_type!(H5P_CLS_ROOT, "H5P_CLS_ROOT_ID_g");
define_native_type!(H5P_CLS_OBJECT_CREATE, "H5P_CLS_OBJECT_CREATE_ID_g");
define_native_type!(H5P_CLS_FILE_CREATE, "H5P_CLS_FILE_CREATE_ID_g");
define_native_type!(H5P_CLS_FILE_ACCESS, "H5P_CLS_FILE_ACCESS_ID_g");
define_native_type!(H5P_CLS_DATASET_CREATE, "H5P_CLS_DATASET_CREATE_ID_g");
define_native_type!(H5P_CLS_DATASET_ACCESS, "H5P_CLS_DATASET_ACCESS_ID_g");
define_native_type!(H5P_CLS_DATASET_XFER, "H5P_CLS_DATASET_XFER_ID_g");
define_native_type!(H5P_CLS_FILE_MOUNT, "H5P_CLS_FILE_MOUNT_ID_g");
define_native_type!(H5P_CLS_GROUP_CREATE, "H5P_CLS_GROUP_CREATE_ID_g");
define_native_type!(H5P_CLS_GROUP_ACCESS, "H5P_CLS_GROUP_ACCESS_ID_g");
define_native_type!(H5P_CLS_DATATYPE_CREATE, "H5P_CLS_DATATYPE_CREATE_ID_g");
define_native_type!(H5P_CLS_DATATYPE_ACCESS, "H5P_CLS_DATATYPE_ACCESS_ID_g");
define_native_type!(H5P_CLS_STRING_CREATE, "H5P_CLS_STRING_CREATE_ID_g");
define_native_type!(H5P_CLS_ATTRIBUTE_CREATE, "H5P_CLS_ATTRIBUTE_CREATE_ID_g");
define_native_type!(H5P_CLS_OBJECT_COPY, "H5P_CLS_OBJECT_COPY_ID_g");
define_native_type!(H5P_CLS_LINK_CREATE, "H5P_CLS_LINK_CREATE_ID_g");
define_native_type!(H5P_CLS_LINK_ACCESS, "H5P_CLS_LINK_ACCESS_ID_g");

// Default property lists
define_native_type!(H5P_LST_FILE_CREATE, "H5P_LST_FILE_CREATE_ID_g");
define_native_type!(H5P_LST_FILE_ACCESS, "H5P_LST_FILE_ACCESS_ID_g");
define_native_type!(H5P_LST_DATASET_CREATE, "H5P_LST_DATASET_CREATE_ID_g");
define_native_type!(H5P_LST_DATASET_ACCESS, "H5P_LST_DATASET_ACCESS_ID_g");
define_native_type!(H5P_LST_DATASET_XFER, "H5P_LST_DATASET_XFER_ID_g");
define_native_type!(H5P_LST_FILE_MOUNT, "H5P_LST_FILE_MOUNT_ID_g");
define_native_type!(H5P_LST_GROUP_CREATE, "H5P_LST_GROUP_CREATE_ID_g");
define_native_type!(H5P_LST_GROUP_ACCESS, "H5P_LST_GROUP_ACCESS_ID_g");
define_native_type!(H5P_LST_DATATYPE_CREATE, "H5P_LST_DATATYPE_CREATE_ID_g");
define_native_type!(H5P_LST_DATATYPE_ACCESS, "H5P_LST_DATATYPE_ACCESS_ID_g");
define_native_type!(H5P_LST_ATTRIBUTE_CREATE, "H5P_LST_ATTRIBUTE_CREATE_ID_g");
define_native_type!(H5P_LST_OBJECT_COPY, "H5P_LST_OBJECT_COPY_ID_g");
define_native_type!(H5P_LST_LINK_CREATE, "H5P_LST_LINK_CREATE_ID_g");
define_native_type!(H5P_LST_LINK_ACCESS, "H5P_LST_LINK_ACCESS_ID_g");
```

**Step 2: Verify no syntax errors**

Run: `cargo check -p tensor4all-hdf5-ffi --no-default-features --features "runtime-loading"`
Expected: Compilation proceeds

---

## Task 5: Update sys/mod.rs Exports

**Files:**
- Modify: `hdf5/src/sys/mod.rs`

**Step 1: Update h5t module exports**

Find the `pub mod h5t` block (around line 200) and add the new exports:

```rust
#[cfg(all(feature = "runtime-loading", not(feature = "link")))]
pub mod h5t {
    pub use super::runtime::{
        // Existing exports...
        H5T_class_t, H5T_cset_t, H5T_order_t, H5T_sign_t, H5T_str_t, H5Tarray_create2, H5Tclose,
        H5Tcommit2, H5Tcommitted, H5Tcopy, H5Tcreate, H5Tenum_create, H5Tenum_insert, H5Tequal,
        H5Tget_array_dims2, H5Tget_array_ndims, H5Tget_class, H5Tget_cset, H5Tget_member_name,
        H5Tget_member_offset, H5Tget_member_type, H5Tget_member_value, H5Tget_native_type,
        H5Tget_nmembers, H5Tget_offset, H5Tget_order, H5Tget_precision, H5Tget_sign, H5Tget_size,
        H5Tget_strpad, H5Tget_super, H5Tinsert, H5Tis_variable_str, H5Tset_cset, H5Tset_ebias,
        H5Tset_fields, H5Tset_offset, H5Tset_order, H5Tset_precision, H5Tset_size, H5Tset_strpad,
        H5Tvlen_create, H5T_CSET_ASCII, H5T_CSET_UTF8, H5T_C_S1, H5T_IEEE_F32BE, H5T_IEEE_F32LE,
        H5T_IEEE_F64BE, H5T_IEEE_F64LE, H5T_NATIVE_DOUBLE, H5T_NATIVE_FLOAT, H5T_NATIVE_INT16,
        H5T_NATIVE_INT32, H5T_NATIVE_INT64, H5T_NATIVE_INT8, H5T_NATIVE_UINT16, H5T_NATIVE_UINT32,
        H5T_NATIVE_UINT64, H5T_NATIVE_UINT8, H5T_STD_I16BE, H5T_STD_I16LE, H5T_STD_I32BE,
        H5T_STD_I32LE, H5T_STD_I64BE, H5T_STD_I64LE, H5T_STD_I8BE, H5T_STD_I8LE, H5T_STD_REF,
        H5T_STD_REF_OBJ, H5T_STD_U16BE, H5T_STD_U16LE, H5T_STD_U32BE, H5T_STD_U32LE, H5T_STD_U64BE,
        H5T_STD_U64LE, H5T_STD_U8BE, H5T_STD_U8LE,
        // NEW: Type conversion types and functions
        H5T_VARIABLE, H5T_cdata_t, H5T_cmd_t, H5T_bkg_t, H5T_conv_t, H5Tfind, H5Tcompiler_conv,
        // NEW: Bitfield types
        H5T_STD_B8BE, H5T_STD_B8LE, H5T_STD_B16BE, H5T_STD_B16LE,
        H5T_STD_B32BE, H5T_STD_B32LE, H5T_STD_B64BE, H5T_STD_B64LE,
        // NEW: Reference type
        H5T_STD_REF_DSETREG,
        // NEW: Time types
        H5T_UNIX_D32BE, H5T_UNIX_D32LE, H5T_UNIX_D64BE, H5T_UNIX_D64LE,
        // NEW: String/VAX types
        H5T_FORTRAN_S1, H5T_VAX_F32, H5T_VAX_F64,
        // NEW: Additional native types
        H5T_NATIVE_SCHAR, H5T_NATIVE_UCHAR, H5T_NATIVE_SHORT, H5T_NATIVE_USHORT,
        H5T_NATIVE_INT, H5T_NATIVE_UINT, H5T_NATIVE_LONG, H5T_NATIVE_ULONG,
        H5T_NATIVE_LLONG, H5T_NATIVE_ULLONG, H5T_NATIVE_LDOUBLE,
        H5T_NATIVE_B8, H5T_NATIVE_B16, H5T_NATIVE_B32, H5T_NATIVE_B64,
        H5T_NATIVE_OPAQUE, H5T_NATIVE_HADDR, H5T_NATIVE_HSIZE,
        H5T_NATIVE_HSSIZE, H5T_NATIVE_HERR, H5T_NATIVE_HBOOL,
        H5T_NATIVE_INT_LEAST8, H5T_NATIVE_UINT_LEAST8, H5T_NATIVE_INT_FAST8, H5T_NATIVE_UINT_FAST8,
        H5T_NATIVE_INT_LEAST16, H5T_NATIVE_UINT_LEAST16, H5T_NATIVE_INT_FAST16, H5T_NATIVE_UINT_FAST16,
        H5T_NATIVE_INT_LEAST32, H5T_NATIVE_UINT_LEAST32, H5T_NATIVE_INT_FAST32, H5T_NATIVE_UINT_FAST32,
        H5T_NATIVE_INT_LEAST64, H5T_NATIVE_UINT_LEAST64, H5T_NATIVE_INT_FAST64, H5T_NATIVE_UINT_FAST64,
    };
    pub use super::runtime::H5T_class_t::*;
}
```

**Step 2: Update h5e module exports**

Find the `pub mod h5e` block and add all error constants:

```rust
#[cfg(all(feature = "runtime-loading", not(feature = "link")))]
pub mod h5e {
    pub use super::runtime::{
        H5E_auto2_t, H5E_error2_t, H5Eclear2, H5Eget_current_stack, H5Eget_msg, H5Eprint2,
        H5Eset_auto2, H5Ewalk2, H5E_DEFAULT, H5E_WALK_DOWNWARD, H5E_WALK_UPWARD,
        // NEW: All error constants
        H5E_ERR_CLS, H5E_NONE_MAJOR, H5E_NONE_MINOR,
        H5E_ARGS, H5E_RESOURCE, H5E_INTERNAL, H5E_FILE, H5E_IO, H5E_FUNC, H5E_ATOM,
        H5E_CACHE, H5E_LINK, H5E_BTREE, H5E_SYM, H5E_HEAP, H5E_OHDR, H5E_DATATYPE,
        H5E_DATASPACE, H5E_DATASET, H5E_STORAGE, H5E_PLIST, H5E_ATTR, H5E_PLINE,
        H5E_EFL, H5E_REFERENCE, H5E_VFL, H5E_TST, H5E_RS, H5E_PLUGIN, H5E_SLIST,
        H5E_FSPACE, H5E_SOHM, H5E_ERROR, H5E_PATH,
        H5E_ALIGNMENT, H5E_ALREADYEXISTS, H5E_ALREADYINIT, H5E_BADATOM, H5E_BADFILE,
        H5E_BADGROUP, H5E_BADITER, H5E_BADMESG, H5E_BADRANGE, H5E_BADSELECT, H5E_BADSIZE,
        H5E_BADTYPE, H5E_BADVALUE, H5E_CALLBACK, H5E_CANAPPLY, H5E_CANTALLOC, H5E_CANTATTACH,
        H5E_CANTCLIP, H5E_CANTCLOSEFILE, H5E_CANTCLOSEOBJ, H5E_CANTCOMPARE, H5E_CANTCOMPUTE,
        H5E_CANTCONVERT, H5E_CANTCOPY, H5E_CANTCOUNT, H5E_CANTCREATE, H5E_CANTDEC,
        H5E_CANTDECODE, H5E_CANTDELETE, H5E_CANTDIRTY, H5E_CANTENCODE, H5E_CANTEXPUNGE,
        H5E_CANTEXTEND, H5E_CANTFILTER, H5E_CANTFLUSH, H5E_CANTFREE, H5E_CANTGC,
        H5E_CANTGET, H5E_CANTGETSIZE, H5E_CANTINC, H5E_CANTINIT, H5E_CANTINS, H5E_CANTINSERT,
        H5E_CANTLIST, H5E_CANTLOAD, H5E_CANTLOCK, H5E_CANTMARKDIRTY, H5E_CANTMERGE,
        H5E_CANTMODIFY, H5E_CANTMOVE, H5E_CANTNEXT, H5E_CANTOPENFILE, H5E_CANTOPENOBJ,
        H5E_CANTOPERATE, H5E_CANTPACK, H5E_CANTPIN, H5E_CANTPROTECT, H5E_CANTRECV,
        H5E_CANTREDISTRIBUTE, H5E_CANTREGISTER, H5E_CANTRELEASE, H5E_CANTREMOVE, H5E_CANTRENAME,
        H5E_CANTRESET, H5E_CANTRESIZE, H5E_CANTRESTORE, H5E_CANTREVIVE, H5E_CANTSELECT,
        H5E_CANTSERIALIZE, H5E_CANTSET, H5E_CANTSHRINK, H5E_CANTSORT, H5E_CANTSPLIT,
        H5E_CANTSWAP, H5E_CANTUNLOCK, H5E_CANTUNPIN, H5E_CANTUNPROTECT, H5E_CANTUPDATE,
        H5E_CLOSEERROR, H5E_COMPLEN, H5E_DUPCLASS, H5E_EXISTS, H5E_FCNTL, H5E_FILEEXISTS,
        H5E_FILEOPEN, H5E_LINKCOUNT, H5E_MOUNT, H5E_MPI, H5E_MPIERRSTR, H5E_NLINKS,
        H5E_NOENCODER, H5E_NOFILTER, H5E_NOIDS, H5E_NOSPACE, H5E_NOTCACHED, H5E_NOTFOUND,
        H5E_NOTHDF5, H5E_NOTREGISTERED, H5E_OBJOPEN, H5E_OPENERROR, H5E_OVERFLOW, H5E_PROTECT,
        H5E_READERROR, H5E_SEEKERROR, H5E_SETDISALLOWED, H5E_SETLOCAL, H5E_SYSERRSTR,
        H5E_SYSTEM, H5E_TRAVERSE, H5E_TRUNCATED, H5E_UNINITIALIZED, H5E_UNSUPPORTED,
        H5E_VERSION, H5E_WRITEERROR,
    };
}
```

**Step 3: Update h5p module exports**

Find the `pub mod h5p` block and add property list constants:

```rust
#[cfg(all(feature = "runtime-loading", not(feature = "link")))]
pub mod h5p {
    pub use super::runtime::{
        // Existing exports...
        H5Pall_filters_avail, H5Pclose, H5Pcopy, H5Pcreate, H5Pequal, H5Pexist,
        // ... (keep all existing)
        H5P_ATTRIBUTE_ACCESS, H5P_ATTRIBUTE_CREATE, H5P_CRT_ORDER_INDEXED, H5P_CRT_ORDER_TRACKED,
        H5P_DATASET_ACCESS, H5P_DATASET_CREATE, H5P_DATASET_XFER, H5P_DATATYPE_ACCESS,
        H5P_DATATYPE_CREATE, H5P_DEFAULT, H5P_FILE_ACCESS, H5P_FILE_CREATE, H5P_GROUP_ACCESS,
        H5P_GROUP_CREATE, H5P_LINK_ACCESS, H5P_LINK_CREATE, H5P_OBJECT_COPY,
        // NEW: Property list classes
        H5P_CLS_ROOT, H5P_CLS_OBJECT_CREATE, H5P_CLS_FILE_CREATE, H5P_CLS_FILE_ACCESS,
        H5P_CLS_DATASET_CREATE, H5P_CLS_DATASET_ACCESS, H5P_CLS_DATASET_XFER, H5P_CLS_FILE_MOUNT,
        H5P_CLS_GROUP_CREATE, H5P_CLS_GROUP_ACCESS, H5P_CLS_DATATYPE_CREATE, H5P_CLS_DATATYPE_ACCESS,
        H5P_CLS_STRING_CREATE, H5P_CLS_ATTRIBUTE_CREATE, H5P_CLS_OBJECT_COPY,
        H5P_CLS_LINK_CREATE, H5P_CLS_LINK_ACCESS,
        // NEW: Default property lists
        H5P_LST_FILE_CREATE, H5P_LST_FILE_ACCESS, H5P_LST_DATASET_CREATE, H5P_LST_DATASET_ACCESS,
        H5P_LST_DATASET_XFER, H5P_LST_FILE_MOUNT, H5P_LST_GROUP_CREATE, H5P_LST_GROUP_ACCESS,
        H5P_LST_DATATYPE_CREATE, H5P_LST_DATATYPE_ACCESS, H5P_LST_ATTRIBUTE_CREATE,
        H5P_LST_OBJECT_COPY, H5P_LST_LINK_CREATE, H5P_LST_LINK_ACCESS,
    };
}
```

**Step 4: Verify exports**

Run: `cargo check -p tensor4all-hdf5-ffi --no-default-features --features "runtime-loading"`
Expected: Fewer errors (globals.rs errors may remain)

---

## Task 6: Update globals.rs for Runtime-Loading

**Files:**
- Modify: `hdf5/src/globals.rs`

**Step 1: Fix hdf5_sys references and add conditional compilation**

Replace the beginning of the file (lines 1-43) with:

```rust
#![allow(dead_code)]

use std::mem;
use std::sync::LazyLock;

#[cfg(feature = "have-direct")]
use crate::sys::h5p::H5Pset_fapl_direct;
use crate::sys::h5p::{
    H5Pclose, H5Pcreate, H5Pget_driver, H5Pset_fapl_core, H5Pset_fapl_family, H5Pset_fapl_log,
    H5Pset_fapl_multi, H5Pset_fapl_sec2, H5Pset_fapl_stdio,
};
use crate::sys::{h5e, h5p, h5t};

use crate::internal_prelude::*;

// Link mode: use H5GlobalConstant wrapper
#[cfg(feature = "link")]
pub struct H5GlobalConstant(
    #[cfg(msvc_dll_indirection)] &'static usize,
    #[cfg(not(msvc_dll_indirection))] &'static hdf5_sys::h5i::hid_t,
);

#[cfg(feature = "link")]
impl std::ops::Deref for H5GlobalConstant {
    type Target = hdf5_sys::h5i::hid_t;
    fn deref(&self) -> &Self::Target {
        LazyLock::force(&crate::sync::LIBRARY_INIT);
        cfg_if::cfg_if! {
            if #[cfg(msvc_dll_indirection)] {
                let dll_ptr = self.0 as *const usize;
                let ptr: *const *const hdf5_sys::h5i::hid_t = dll_ptr.cast();
                unsafe {
                    &**ptr
                }
            } else {
                self.0
            }
        }
    }
}

#[cfg(feature = "link")]
macro_rules! link_hid {
    ($rust_name:ident, $c_name:path) => {
        pub static $rust_name: H5GlobalConstant = H5GlobalConstant($c_name);
    };
}

// Runtime-loading mode: use LazyLock with function calls
#[cfg(all(feature = "runtime-loading", not(feature = "link")))]
macro_rules! link_hid {
    ($rust_name:ident, $c_name:path) => {
        pub static $rust_name: LazyLock<hid_t> = LazyLock::new(|| $c_name());
    };
}
```

**Step 2: Fix get_driver macro for runtime-loading**

Update the `get_driver!` macro to work with both modes. After the `link_hid!` macro, update `get_driver!`:

```rust
/// Fetches the driver ID using the workaround from https://github.com/HDFGroup/hdf5/issues/1809
/// as the _init functions seem to be removed in HDF5 2.0.0
macro_rules! get_driver {
    ($set_driver:expr) => {{
        let fapl = h5call!(H5Pcreate(*H5P_FILE_ACCESS)).expect("should always create FAPL");
        h5call!($set_driver(fapl)).expect("should always be able to set the driver");
        let id = h5call!(H5Pget_driver(fapl)).expect("should always be able to extract the driver");
        h5call!(H5Pclose(fapl)).expect("should always be able to close the FAPL");
        id
    }};
}
```

**Step 3: Fix H5FD_MPIO for runtime-loading**

Update the H5FD_MPIO definitions (around line 362-371) to handle runtime-loading:

```rust
// MPI-IO file driver
#[cfg(all(feature = "link", feature = "2.0.0", all(feature = "have-parallel", feature = "mpio")))]
pub static H5FD_MPIO: LazyLock<hid_t> = LazyLock::new(|| *hdf5_sys::h5p::H5FD_MPIO_id);
#[cfg(all(feature = "link", feature = "2.0.0", not(all(feature = "have-parallel", feature = "mpio"))))]
pub static H5FD_MPIO: LazyLock<hid_t> = LazyLock::new(|| H5I_INVALID_HID);

#[cfg(all(feature = "link", not(feature = "2.0.0"), all(feature = "have-parallel", feature = "mpio")))]
pub static H5FD_MPIO: LazyLock<hid_t> = LazyLock::new(|| h5lock!(hdf5_sys::h5fd::H5FD_mpio_init()));
#[cfg(all(feature = "link", not(feature = "2.0.0"), not(all(feature = "have-parallel", feature = "mpio"))))]
pub static H5FD_MPIO: LazyLock<hid_t> = LazyLock::new(|| H5I_INVALID_HID);

// Runtime-loading mode: MPI not supported
#[cfg(all(feature = "runtime-loading", not(feature = "link")))]
pub static H5FD_MPIO: LazyLock<hid_t> = LazyLock::new(|| H5I_INVALID_HID);
```

**Step 4: Verify compilation**

Run: `cargo check -p tensor4all-hdf5-ffi --no-default-features --features "runtime-loading,complex"`
Expected: No errors

---

## Task 7: Verify Link Mode Still Works

**Step 1: Check link mode compilation**

Run: `cargo check -p tensor4all-hdf5-ffi`
Expected: No errors

**Step 2: Run tests in link mode**

Run: `cargo test --workspace`
Expected: All tests pass

---

## Task 8: Final Verification and Commit

**Step 1: Format code**

Run: `cargo fmt --all`

**Step 2: Run clippy**

Run: `cargo clippy --workspace`
Expected: No errors

**Step 3: Final runtime-loading check**

Run: `cargo check -p tensor4all-hdf5-ffi --no-default-features --features "runtime-loading,complex"`
Expected: No errors

**Step 4: Commit changes**

```bash
git add hdf5/src/sys/runtime.rs hdf5/src/sys/mod.rs hdf5/src/globals.rs
git commit -m "feat: complete runtime-loading feature implementation

Add ~230 missing symbols to runtime-loading mode:
- H5T type constants (bitfield, time, native types)
- H5T functions (H5Tfind, H5Tcompiler_conv) and types (H5T_cdata_t)
- H5E error constants (~120 symbols)
- H5P property list constants (~30 symbols)

Update globals.rs with conditional compilation to support both
link mode and runtime-loading mode with unified access pattern.

Closes #5

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```
