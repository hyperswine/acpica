use super::{local::AcpiNamespaceNode, AcpiMutex, AcpiSemaphore, AcpiThreadId};
use core::{ffi::c_char, ptr::null_mut};

// ----------------
// OPERAND OBJECT
// ----------------

#[repr(C)]
pub union AcpiOperandObject {
    Common: AcpiObjectCommon,
    Integer: AcpiObjectInteger,
    String: AcpiObjectString,
    Buffer: AcpiObjectBuffer,
    Package: AcpiObjectPackage,
    Event: AcpiObjectEvent,
    Method: AcpiObjectMethod,
    Mutex: AcpiObjectMutex,
    Region: AcpiObjectRegion,
    CommonNotify: AcpiObjectNotifyCommon,
    Device: AcpiObjectDevice,
    PowerResource: AcpiObjectPowerResource,
    Processor: AcpiObjectProcessor,
    ThermalZone: AcpiObjectThermalZone,
    CommonField: AcpiObjectCommonField,
    Field: AcpiObjectField,
    BufferField: AcpiObjectBufferField,
    BankField: AcpiObjectBankField,
    IndexField: AcpiObjectIndexField,
    Notify: AcpiObjectNotify,
    AddressSpace: AcpiObjectAddrHandler,
    Reference: AcpiObjectReference,
    Extra: AcpiObjectExtra,
    Data: AcpiObjectData,
    Cache: AcpiObjectCacheList,
    Node: AcpiNamespaceNode,
}

// ----------------
// Acpi Objects
// ----------------

// Headers & Buffers

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectCommonHeader {
    next_object: *mut AcpiOperandObject,
    descriptor_type: u8,
    object_type: u8,
    ref_count: u16,
    flags: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiCommonBufferInfo<T> {
    pointer: T,
    length: u32,
}

// Main Objects

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectCommon(AcpiObjectCommonHeader);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectInteger {
    header: AcpiObjectCommonHeader,
    fill: [u8; 3],
    value: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectString {
    header: AcpiObjectCommonHeader,
    buffer_info: AcpiCommonBufferInfo<c_char>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectBuffer {
    header: AcpiObjectCommonHeader,
    buffer_info: AcpiCommonBufferInfo<u8>,
    aml_length: u32,
    aml_start: *mut u8,
    node: *mut AcpiNamespaceNode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectPackage {
    header: AcpiObjectCommonHeader,
    node: *mut AcpiNamespaceNode,
    elements: *mut *mut AcpiOperandObject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectEvent {
    header: AcpiObjectCommonHeader,
    os_semaphore: AcpiSemaphore,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiObjectMutex {
    header: AcpiObjectCommonHeader,
    synclevel: u8,
    acquisition_depth: u16,
    os_mutex: AcpiMutex,
    thread_id: AcpiThreadId,
    owner_thread: *mut AcpiThreadState,
    prev: *mut AcpiOperandObject,
    next: *mut AcpiOperandObject,
    node: *mut AcpiNamespaceNode,
    original_sync_level: u8,
}

// ----------
// CONSTANTS
// ----------

pub const AOPOBJ_AML_CONSTANT: i32 = 0x01;
pub const AOPOBJ_STATIC_POINTER: i32 = 0x02;
pub const AOPOBJ_DATA_VALID: i32 = 0x04;
pub const AOPOBJ_OBJECT_INITIALIZED: i32 = 0x08;
pub const AOPOBJ_REG_CONNECTED: i32 = 0x10;
pub const AOPOBJ_SETUP_COMPLETE: i32 = 0x20;
pub const AOPOBJ_INVALID: i32 = 0x40;

// ----------
// TESTS
// ----------

#[test]
fn test_object_event() {
    let obj = AcpiObjectCommonHeader {
        next_object: null_mut(),
        descriptor_type: 0,
        object_type: 0,
        ref_count: 0,
        flags: 0,
    };
}
