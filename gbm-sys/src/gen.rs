/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_device([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_bo([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_surface([u8; 0]);
/**
 * Abstraction representing the handle to a buffer allocated by the
 * manager
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gbm_bo_handle {
    pub ptr: __BindgenUnionField<*mut libc::c_void>,
    pub s32: __BindgenUnionField<i32>,
    pub u32: __BindgenUnionField<u32>,
    pub s64: __BindgenUnionField<i64>,
    pub u64: __BindgenUnionField<u64>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_gbm_bo_handle() {
    assert_eq!(::std::mem::size_of::<gbm_bo_handle>() , 8usize , concat ! (
               "Size of: " , stringify ! ( gbm_bo_handle ) ));
    assert_eq! (::std::mem::align_of::<gbm_bo_handle>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gbm_bo_handle ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_bo_handle ) ) . ptr as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_bo_handle ) , "::"
                , stringify ! ( ptr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_bo_handle ) ) . s32 as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_bo_handle ) , "::"
                , stringify ! ( s32 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_bo_handle ) ) . u32 as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_bo_handle ) , "::"
                , stringify ! ( u32 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_bo_handle ) ) . s64 as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_bo_handle ) , "::"
                , stringify ! ( s64 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_bo_handle ) ) . u64 as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_bo_handle ) , "::"
                , stringify ! ( u64 ) ));
}
impl Clone for gbm_bo_handle {
    fn clone(&self) -> Self { *self }
}
#[repr(u32)]
/** Format of the allocated buffer */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gbm_bo_format {
    GBM_BO_FORMAT_XRGB8888 = 0,
    GBM_BO_FORMAT_ARGB8888 = 1,
}
pub const gbm_bo_flags_GBM_BO_USE_CURSOR_64X64: gbm_bo_flags =
    gbm_bo_flags::GBM_BO_USE_CURSOR;
#[repr(u32)]
/**
 * Flags to indicate the intended use for the buffer - these are passed into
 * gbm_bo_create(). The caller must set the union of all the flags that are
 * appropriate
 *
 * \sa Use gbm_device_is_format_supported() to check if the combination of format
 * and use flags are supported
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gbm_bo_flags {
    GBM_BO_USE_SCANOUT = 1,
    GBM_BO_USE_CURSOR = 2,
    GBM_BO_USE_RENDERING = 4,
    GBM_BO_USE_WRITE = 8,
    GBM_BO_USE_LINEAR = 16,
}
extern "C" {
    pub fn gbm_device_get_fd(gbm: *mut gbm_device) -> libc::c_int;
}
extern "C" {
    pub fn gbm_device_get_backend_name(gbm: *mut gbm_device)
     -> *const libc::c_char;
}
extern "C" {
    pub fn gbm_device_is_format_supported(gbm: *mut gbm_device, format: u32,
                                          usage: u32) -> libc::c_int;
}
extern "C" {
    pub fn gbm_device_destroy(gbm: *mut gbm_device);
}
extern "C" {
    pub fn gbm_create_device(fd: libc::c_int) -> *mut gbm_device;
}
extern "C" {
    pub fn gbm_bo_create(gbm: *mut gbm_device, width: u32, height: u32,
                         format: u32, flags: u32) -> *mut gbm_bo;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gbm_import_fd_data {
    pub fd: libc::c_int,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}
#[test]
fn bindgen_test_layout_gbm_import_fd_data() {
    assert_eq!(::std::mem::size_of::<gbm_import_fd_data>() , 20usize , concat
               ! ( "Size of: " , stringify ! ( gbm_import_fd_data ) ));
    assert_eq! (::std::mem::align_of::<gbm_import_fd_data>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( gbm_import_fd_data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_import_fd_data ) ) . fd as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_import_fd_data ) ,
                "::" , stringify ! ( fd ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_import_fd_data ) ) . width as * const
                _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_import_fd_data ) ,
                "::" , stringify ! ( width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_import_fd_data ) ) . height as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_import_fd_data ) ,
                "::" , stringify ! ( height ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_import_fd_data ) ) . stride as *
                const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_import_fd_data ) ,
                "::" , stringify ! ( stride ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gbm_import_fd_data ) ) . format as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gbm_import_fd_data ) ,
                "::" , stringify ! ( format ) ));
}
impl Clone for gbm_import_fd_data {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn gbm_bo_import(gbm: *mut gbm_device, type_: u32,
                         buffer: *mut libc::c_void, usage: u32)
     -> *mut gbm_bo;
}
#[repr(u32)]
/**
 * Flags to indicate the type of mapping for the buffer - these are
 * passed into gbm_bo_map(). The caller must set the union of all the
 * flags that are appropriate.
 *
 * These flags are independent of the GBM_BO_USE_* creation flags. However,
 * mapping the buffer may require copying to/from a staging buffer.
 *
 * See also: pipe_transfer_usage
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gbm_bo_transfer_flags {
    GBM_BO_TRANSFER_READ = 1,
    GBM_BO_TRANSFER_WRITE = 2,
    GBM_BO_TRANSFER_READ_WRITE = 3,
}
extern "C" {
    pub fn gbm_bo_map(bo: *mut gbm_bo, x: u32, y: u32, width: u32,
                      height: u32, flags: u32, stride: *mut u32,
                      map_data: *mut *mut libc::c_void) -> *mut libc::c_void;
}
extern "C" {
    pub fn gbm_bo_unmap(bo: *mut gbm_bo, map_data: *mut libc::c_void);
}
extern "C" {
    pub fn gbm_bo_get_width(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_height(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_stride(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_format(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_device(bo: *mut gbm_bo) -> *mut gbm_device;
}
extern "C" {
    pub fn gbm_bo_get_handle(bo: *mut gbm_bo) -> gbm_bo_handle;
}
extern "C" {
    pub fn gbm_bo_get_fd(bo: *mut gbm_bo) -> libc::c_int;
}
extern "C" {
    pub fn gbm_bo_write(bo: *mut gbm_bo, buf: *const libc::c_void,
                        count: usize) -> libc::c_int;
}
extern "C" {
    pub fn gbm_bo_set_user_data(bo: *mut gbm_bo, data: *mut libc::c_void,
                                destroy_user_data:
                                    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                   *mut gbm_bo,
                                                                               arg2:
                                                                                   *mut libc::c_void)>);
}
extern "C" {
    pub fn gbm_bo_get_user_data(bo: *mut gbm_bo) -> *mut libc::c_void;
}
extern "C" {
    pub fn gbm_bo_destroy(bo: *mut gbm_bo);
}
extern "C" {
    pub fn gbm_surface_create(gbm: *mut gbm_device, width: u32, height: u32,
                              format: u32, flags: u32) -> *mut gbm_surface;
}
extern "C" {
    pub fn gbm_surface_needs_lock_front_buffer(surface: *mut gbm_surface)
     -> libc::c_int;
}
extern "C" {
    pub fn gbm_surface_lock_front_buffer(surface: *mut gbm_surface)
     -> *mut gbm_bo;
}
extern "C" {
    pub fn gbm_surface_release_buffer(surface: *mut gbm_surface,
                                      bo: *mut gbm_bo);
}
extern "C" {
    pub fn gbm_surface_has_free_buffers(surface: *mut gbm_surface)
     -> libc::c_int;
}
extern "C" {
    pub fn gbm_surface_destroy(surface: *mut gbm_surface);
}
