/* automatically generated by rust-bindgen */

pub const GSS_C_DELEG_FLAG: u32 = 1;
pub const GSS_C_MUTUAL_FLAG: u32 = 2;
pub const GSS_C_REPLAY_FLAG: u32 = 4;
pub const GSS_C_SEQUENCE_FLAG: u32 = 8;
pub const GSS_C_CONF_FLAG: u32 = 16;
pub const GSS_C_INTEG_FLAG: u32 = 32;
pub const GSS_C_ANON_FLAG: u32 = 64;
pub const GSS_C_PROT_READY_FLAG: u32 = 128;
pub const GSS_C_TRANS_FLAG: u32 = 256;
pub const GSS_C_DELEG_POLICY_FLAG: u32 = 32768;
pub const GSS_C_BOTH: u32 = 0;
pub const GSS_C_INITIATE: u32 = 1;
pub const GSS_C_ACCEPT: u32 = 2;
pub const GSS_C_GSS_CODE: u32 = 1;
pub const GSS_C_MECH_CODE: u32 = 2;
pub const GSS_C_AF_UNSPEC: u32 = 0;
pub const GSS_C_AF_LOCAL: u32 = 1;
pub const GSS_C_AF_INET: u32 = 2;
pub const GSS_C_AF_IMPLINK: u32 = 3;
pub const GSS_C_AF_PUP: u32 = 4;
pub const GSS_C_AF_CHAOS: u32 = 5;
pub const GSS_C_AF_NS: u32 = 6;
pub const GSS_C_AF_NBS: u32 = 7;
pub const GSS_C_AF_ECMA: u32 = 8;
pub const GSS_C_AF_DATAKIT: u32 = 9;
pub const GSS_C_AF_CCITT: u32 = 10;
pub const GSS_C_AF_SNA: u32 = 11;
pub const GSS_C_AF_DECnet: u32 = 12;
pub const GSS_C_AF_DLI: u32 = 13;
pub const GSS_C_AF_LAT: u32 = 14;
pub const GSS_C_AF_HYLINK: u32 = 15;
pub const GSS_C_AF_APPLETALK: u32 = 16;
pub const GSS_C_AF_BSC: u32 = 17;
pub const GSS_C_AF_DSS: u32 = 18;
pub const GSS_C_AF_OSI: u32 = 19;
pub const GSS_C_AF_NETBIOS: u32 = 20;
pub const GSS_C_AF_X25: u32 = 21;
pub const GSS_C_AF_NULLADDR: u32 = 255;
pub const GSS_C_QOP_DEFAULT: u32 = 0;
pub const GSS_S_COMPLETE: u32 = 0;
pub const GSS_C_CALLING_ERROR_OFFSET: u32 = 24;
pub const GSS_C_ROUTINE_ERROR_OFFSET: u32 = 16;
pub const GSS_C_SUPPLEMENTARY_OFFSET: u32 = 0;
pub const GSS_S_CONTINUE_NEEDED: u32 = 1;
pub const GSS_S_DUPLICATE_TOKEN: u32 = 2;
pub const GSS_S_OLD_TOKEN: u32 = 4;
pub const GSS_S_UNSEQ_TOKEN: u32 = 8;
pub const GSS_S_GAP_TOKEN: u32 = 16;
pub const GSS_C_PRF_KEY_FULL: u32 = 0;
pub const GSS_C_PRF_KEY_PARTIAL: u32 = 1;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __ssize_t = ::std::os::raw::c_long;
pub type size_t = ::std::os::raw::c_ulong;
pub type ssize_t = __ssize_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_name_struct {
    _unused: [u8; 0],
}
pub type gss_name_t = *mut gss_name_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_cred_id_struct {
    _unused: [u8; 0],
}
pub type gss_cred_id_t = *mut gss_cred_id_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_ctx_id_struct {
    _unused: [u8; 0],
}
pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
pub type gss_uint32 = u32;
pub type gss_int32 = i32;
pub type OM_uint32 = gss_uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_OID_desc_struct {
    pub length: OM_uint32,
    pub elements: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_gss_OID_desc_struct() {
    assert_eq!(
        ::std::mem::size_of::<gss_OID_desc_struct>(),
        16usize,
        concat!("Size of: ", stringify!(gss_OID_desc_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<gss_OID_desc_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(gss_OID_desc_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gss_OID_desc_struct>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_OID_desc_struct),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gss_OID_desc_struct>())).elements as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_OID_desc_struct),
            "::",
            stringify!(elements)
        )
    );
}
pub type gss_OID_desc = gss_OID_desc_struct;
pub type gss_OID = *mut gss_OID_desc_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_OID_set_desc_struct {
    pub count: size_t,
    pub elements: gss_OID,
}
#[test]
fn bindgen_test_layout_gss_OID_set_desc_struct() {
    assert_eq!(
        ::std::mem::size_of::<gss_OID_set_desc_struct>(),
        16usize,
        concat!("Size of: ", stringify!(gss_OID_set_desc_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<gss_OID_set_desc_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(gss_OID_set_desc_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gss_OID_set_desc_struct>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_OID_set_desc_struct),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gss_OID_set_desc_struct>())).elements as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_OID_set_desc_struct),
            "::",
            stringify!(elements)
        )
    );
}
pub type gss_OID_set_desc = gss_OID_set_desc_struct;
pub type gss_OID_set = *mut gss_OID_set_desc_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_buffer_desc_struct {
    pub length: size_t,
    pub value: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_gss_buffer_desc_struct() {
    assert_eq!(
        ::std::mem::size_of::<gss_buffer_desc_struct>(),
        16usize,
        concat!("Size of: ", stringify!(gss_buffer_desc_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<gss_buffer_desc_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(gss_buffer_desc_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gss_buffer_desc_struct>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_buffer_desc_struct),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gss_buffer_desc_struct>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_buffer_desc_struct),
            "::",
            stringify!(value)
        )
    );
}
pub type gss_buffer_desc = gss_buffer_desc_struct;
pub type gss_buffer_t = *mut gss_buffer_desc_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_channel_bindings_struct {
    pub initiator_addrtype: OM_uint32,
    pub initiator_address: gss_buffer_desc,
    pub acceptor_addrtype: OM_uint32,
    pub acceptor_address: gss_buffer_desc,
    pub application_data: gss_buffer_desc,
}
#[test]
fn bindgen_test_layout_gss_channel_bindings_struct() {
    assert_eq!(
        ::std::mem::size_of::<gss_channel_bindings_struct>(),
        64usize,
        concat!("Size of: ", stringify!(gss_channel_bindings_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<gss_channel_bindings_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(gss_channel_bindings_struct))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gss_channel_bindings_struct>())).initiator_addrtype as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_channel_bindings_struct),
            "::",
            stringify!(initiator_addrtype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gss_channel_bindings_struct>())).initiator_address as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_channel_bindings_struct),
            "::",
            stringify!(initiator_address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gss_channel_bindings_struct>())).acceptor_addrtype as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_channel_bindings_struct),
            "::",
            stringify!(acceptor_addrtype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gss_channel_bindings_struct>())).acceptor_address as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_channel_bindings_struct),
            "::",
            stringify!(acceptor_address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gss_channel_bindings_struct>())).application_data as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(gss_channel_bindings_struct),
            "::",
            stringify!(application_data)
        )
    );
}
pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
pub type gss_qop_t = OM_uint32;
pub type gss_cred_usage_t = ::std::os::raw::c_int;
extern "C" {
    pub static mut GSS_C_NT_USER_NAME: gss_OID;
}
extern "C" {
    pub static mut GSS_C_NT_MACHINE_UID_NAME: gss_OID;
}
extern "C" {
    pub static mut GSS_C_NT_STRING_UID_NAME: gss_OID;
}
extern "C" {
    pub static mut GSS_C_NT_HOSTBASED_SERVICE_X: gss_OID;
}
extern "C" {
    pub static mut GSS_C_NT_HOSTBASED_SERVICE: gss_OID;
}
extern "C" {
    pub static mut GSS_C_NT_ANONYMOUS: gss_OID;
}
extern "C" {
    pub static mut GSS_C_NT_EXPORT_NAME: gss_OID;
}
extern "C" {
    pub fn gss_acquire_cred(
        arg1: *mut OM_uint32,
        arg2: gss_name_t,
        arg3: OM_uint32,
        arg4: gss_OID_set,
        arg5: gss_cred_usage_t,
        arg6: *mut gss_cred_id_t,
        arg7: *mut gss_OID_set,
        arg8: *mut OM_uint32,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_release_cred(arg1: *mut OM_uint32, arg2: *mut gss_cred_id_t) -> OM_uint32;
}
extern "C" {
    pub fn gss_init_sec_context(
        arg1: *mut OM_uint32,
        arg2: gss_cred_id_t,
        arg3: *mut gss_ctx_id_t,
        arg4: gss_name_t,
        arg5: gss_OID,
        arg6: OM_uint32,
        arg7: OM_uint32,
        arg8: gss_channel_bindings_t,
        arg9: gss_buffer_t,
        arg10: *mut gss_OID,
        arg11: gss_buffer_t,
        arg12: *mut OM_uint32,
        arg13: *mut OM_uint32,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_accept_sec_context(
        arg1: *mut OM_uint32,
        arg2: *mut gss_ctx_id_t,
        arg3: gss_cred_id_t,
        arg4: gss_buffer_t,
        arg5: gss_channel_bindings_t,
        arg6: *mut gss_name_t,
        arg7: *mut gss_OID,
        arg8: gss_buffer_t,
        arg9: *mut OM_uint32,
        arg10: *mut OM_uint32,
        arg11: *mut gss_cred_id_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_process_context_token(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_delete_sec_context(
        arg1: *mut OM_uint32,
        arg2: *mut gss_ctx_id_t,
        arg3: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_context_time(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: *mut OM_uint32,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_get_mic(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: gss_qop_t,
        arg4: gss_buffer_t,
        arg5: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_verify_mic(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: gss_buffer_t,
        arg4: gss_buffer_t,
        arg5: *mut gss_qop_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_wrap(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: ::std::os::raw::c_int,
        arg4: gss_qop_t,
        arg5: gss_buffer_t,
        arg6: *mut ::std::os::raw::c_int,
        arg7: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_unwrap(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: gss_buffer_t,
        arg4: gss_buffer_t,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut gss_qop_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_display_status(
        arg1: *mut OM_uint32,
        arg2: OM_uint32,
        arg3: ::std::os::raw::c_int,
        arg4: gss_OID,
        arg5: *mut OM_uint32,
        arg6: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_indicate_mechs(arg1: *mut OM_uint32, arg2: *mut gss_OID_set) -> OM_uint32;
}
extern "C" {
    pub fn gss_compare_name(
        arg1: *mut OM_uint32,
        arg2: gss_name_t,
        arg3: gss_name_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_display_name(
        arg1: *mut OM_uint32,
        arg2: gss_name_t,
        arg3: gss_buffer_t,
        arg4: *mut gss_OID,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_import_name(
        arg1: *mut OM_uint32,
        arg2: gss_buffer_t,
        arg3: gss_OID,
        arg4: *mut gss_name_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_release_name(arg1: *mut OM_uint32, arg2: *mut gss_name_t) -> OM_uint32;
}
extern "C" {
    pub fn gss_release_buffer(arg1: *mut OM_uint32, arg2: gss_buffer_t) -> OM_uint32;
}
extern "C" {
    pub fn gss_release_oid_set(arg1: *mut OM_uint32, arg2: *mut gss_OID_set) -> OM_uint32;
}
extern "C" {
    pub fn gss_inquire_cred(
        arg1: *mut OM_uint32,
        arg2: gss_cred_id_t,
        arg3: *mut gss_name_t,
        arg4: *mut OM_uint32,
        arg5: *mut gss_cred_usage_t,
        arg6: *mut gss_OID_set,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_inquire_context(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: *mut gss_name_t,
        arg4: *mut gss_name_t,
        arg5: *mut OM_uint32,
        arg6: *mut gss_OID,
        arg7: *mut OM_uint32,
        arg8: *mut ::std::os::raw::c_int,
        arg9: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_wrap_size_limit(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: ::std::os::raw::c_int,
        arg4: gss_qop_t,
        arg5: OM_uint32,
        arg6: *mut OM_uint32,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_import_name_object(
        arg1: *mut OM_uint32,
        arg2: *mut ::std::os::raw::c_void,
        arg3: gss_OID,
        arg4: *mut gss_name_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_export_name_object(
        arg1: *mut OM_uint32,
        arg2: gss_name_t,
        arg3: gss_OID,
        arg4: *mut *mut ::std::os::raw::c_void,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_add_cred(
        arg1: *mut OM_uint32,
        arg2: gss_cred_id_t,
        arg3: gss_name_t,
        arg4: gss_OID,
        arg5: gss_cred_usage_t,
        arg6: OM_uint32,
        arg7: OM_uint32,
        arg8: *mut gss_cred_id_t,
        arg9: *mut gss_OID_set,
        arg10: *mut OM_uint32,
        arg11: *mut OM_uint32,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_inquire_cred_by_mech(
        arg1: *mut OM_uint32,
        arg2: gss_cred_id_t,
        arg3: gss_OID,
        arg4: *mut gss_name_t,
        arg5: *mut OM_uint32,
        arg6: *mut OM_uint32,
        arg7: *mut gss_cred_usage_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_export_sec_context(
        arg1: *mut OM_uint32,
        arg2: *mut gss_ctx_id_t,
        arg3: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_import_sec_context(
        arg1: *mut OM_uint32,
        arg2: gss_buffer_t,
        arg3: *mut gss_ctx_id_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_release_oid(arg1: *mut OM_uint32, arg2: *mut gss_OID) -> OM_uint32;
}
extern "C" {
    pub fn gss_create_empty_oid_set(arg1: *mut OM_uint32, arg2: *mut gss_OID_set) -> OM_uint32;
}
extern "C" {
    pub fn gss_add_oid_set_member(
        arg1: *mut OM_uint32,
        arg2: gss_OID,
        arg3: *mut gss_OID_set,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_test_oid_set_member(
        arg1: *mut OM_uint32,
        arg2: gss_OID,
        arg3: gss_OID_set,
        arg4: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_str_to_oid(
        arg1: *mut OM_uint32,
        arg2: gss_buffer_t,
        arg3: *mut gss_OID,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_oid_to_str(arg1: *mut OM_uint32, arg2: gss_OID, arg3: gss_buffer_t) -> OM_uint32;
}
extern "C" {
    pub fn gss_inquire_names_for_mech(
        arg1: *mut OM_uint32,
        arg2: gss_OID,
        arg3: *mut gss_OID_set,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_inquire_mechs_for_name(
        arg1: *mut OM_uint32,
        arg2: gss_name_t,
        arg3: *mut gss_OID_set,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_sign(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: ::std::os::raw::c_int,
        arg4: gss_buffer_t,
        arg5: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_verify(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: gss_buffer_t,
        arg4: gss_buffer_t,
        arg5: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_seal(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: gss_buffer_t,
        arg6: *mut ::std::os::raw::c_int,
        arg7: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_unseal(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: gss_buffer_t,
        arg4: gss_buffer_t,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_export_name(arg1: *mut OM_uint32, arg2: gss_name_t, arg3: gss_buffer_t)
        -> OM_uint32;
}
extern "C" {
    pub fn gss_duplicate_name(
        arg1: *mut OM_uint32,
        arg2: gss_name_t,
        arg3: *mut gss_name_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_canonicalize_name(
        arg1: *mut OM_uint32,
        arg2: gss_name_t,
        arg3: gss_OID,
        arg4: *mut gss_name_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_pseudo_random(
        arg1: *mut OM_uint32,
        arg2: gss_ctx_id_t,
        arg3: ::std::os::raw::c_int,
        arg4: gss_buffer_t,
        arg5: ssize_t,
        arg6: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_store_cred(
        arg1: *mut OM_uint32,
        arg2: gss_cred_id_t,
        arg3: gss_cred_usage_t,
        arg4: gss_OID,
        arg5: OM_uint32,
        arg6: OM_uint32,
        arg7: *mut gss_OID_set,
        arg8: *mut gss_cred_usage_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_set_neg_mechs(
        arg1: *mut OM_uint32,
        arg2: gss_cred_id_t,
        arg3: gss_OID_set,
    ) -> OM_uint32;
}
pub type gss_const_buffer_t = *const gss_buffer_desc;
pub type gss_const_channel_bindings_t = *const gss_channel_bindings_struct;
pub type gss_const_ctx_id_t = *const gss_ctx_id_struct;
pub type gss_const_cred_id_t = *const gss_cred_id_struct;
pub type gss_const_name_t = *const gss_name_struct;
pub type gss_const_OID = *const gss_OID_desc;
pub type gss_const_OID_set = *const gss_OID_set_desc;
extern "C" {
    pub fn gss_indicate_mechs_by_attrs(
        arg1: *mut OM_uint32,
        arg2: gss_const_OID_set,
        arg3: gss_const_OID_set,
        arg4: gss_const_OID_set,
        arg5: *mut gss_OID_set,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_inquire_attrs_for_mech(
        arg1: *mut OM_uint32,
        arg2: gss_const_OID,
        arg3: *mut gss_OID_set,
        arg4: *mut gss_OID_set,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_display_mech_attr(
        arg1: *mut OM_uint32,
        arg2: gss_const_OID,
        arg3: gss_buffer_t,
        arg4: gss_buffer_t,
        arg5: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub static mut GSS_C_MA_MECH_CONCRETE: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_MECH_PSEUDO: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_MECH_COMPOSITE: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_MECH_NEGO: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_MECH_GLUE: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_NOT_MECH: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_DEPRECATED: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_NOT_DFLT_MECH: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_ITOK_FRAMED: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_AUTH_INIT: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_AUTH_TARG: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_AUTH_INIT_INIT: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_AUTH_TARG_INIT: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_AUTH_INIT_ANON: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_AUTH_TARG_ANON: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_DELEG_CRED: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_INTEG_PROT: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_CONF_PROT: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_MIC: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_WRAP: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_PROT_READY: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_REPLAY_DET: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_OOS_DET: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_CBINDINGS: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_PFS: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_COMPRESS: gss_const_OID;
}
extern "C" {
    pub static mut GSS_C_MA_CTX_TRANS: gss_const_OID;
}
extern "C" {
    pub fn gss_inquire_saslname_for_mech(
        arg1: *mut OM_uint32,
        arg2: gss_OID,
        arg3: gss_buffer_t,
        arg4: gss_buffer_t,
        arg5: gss_buffer_t,
    ) -> OM_uint32;
}
extern "C" {
    pub fn gss_inquire_mech_for_saslname(
        arg1: *mut OM_uint32,
        arg2: gss_buffer_t,
        arg3: *mut gss_OID,
    ) -> OM_uint32;
}
pub const _GSS_S_DEFECTIVE_TOKEN: OM_uint32 = 589824;
pub const _GSS_S_NO_CONTEXT: OM_uint32 = 524288;
pub const _GSS_C_INDEFINITE: OM_uint32 = 4294967295;
pub const _GSS_S_CONTINUE_NEEDED: OM_uint32 = 1;
pub const _GSS_C_CALLING_ERROR_MASK: OM_uint32 = 255;
pub const _GSS_C_ROUTINE_ERROR_MASK: OM_uint32 = 255;
