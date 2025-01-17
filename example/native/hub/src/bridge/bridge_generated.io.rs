use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_prepare_rust_signal_stream(port_: i64) {
    wire_prepare_rust_signal_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_prepare_rust_response_stream(port_: i64) {
    wire_prepare_rust_response_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_prepare_channels() -> support::WireSyncReturn {
    wire_prepare_channels_impl()
}

#[no_mangle]
pub extern "C" fn wire_check_rust_streams(port_: i64) {
    wire_check_rust_streams_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_start_rust_logic(port_: i64) {
    wire_start_rust_logic_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_request_to_rust(
    request_unique: *mut wire_RustRequestUnique,
) -> support::WireSyncReturn {
    wire_request_to_rust_impl(request_unique)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_rust_request_unique_0() -> *mut wire_RustRequestUnique {
    support::new_leak_box_ptr(wire_RustRequestUnique::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<RustRequestUnique> for *mut wire_RustRequestUnique {
    fn wire2api(self) -> RustRequestUnique {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RustRequestUnique>::wire2api(*wrap).into()
    }
}

impl Wire2Api<RustRequest> for wire_RustRequest {
    fn wire2api(self) -> RustRequest {
        RustRequest {
            address: self.address.wire2api(),
            operation: self.operation.wire2api(),
            bytes: self.bytes.wire2api(),
        }
    }
}
impl Wire2Api<RustRequestUnique> for wire_RustRequestUnique {
    fn wire2api(self) -> RustRequestUnique {
        RustRequestUnique {
            id: self.id.wire2api(),
            request: self.request.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustRequest {
    address: *mut wire_uint_8_list,
    operation: i32,
    bytes: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustRequestUnique {
    id: i32,
    request: wire_RustRequest,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_RustRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            address: core::ptr::null_mut(),
            operation: Default::default(),
            bytes: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_RustRequest {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_RustRequestUnique {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            request: Default::default(),
        }
    }
}

impl Default for wire_RustRequestUnique {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
