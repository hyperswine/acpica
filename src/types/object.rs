use core::ffi::c_char;

use super::local::{AcpiOperandObject, AcpiNamespaceNode};

// ----------
// TYPES
// ----------

// Headers and Buffers

#[repr(C)]
pub struct AcpiObjectCommonHeader {
    next_object: *mut AcpiOperandObject,
    descriptor_type: u8,
    object_type: u8,
    ref_count: u16,
    flags: u8,
}

#[repr(C)]
pub struct AcpiCommonBufferInfo<T> {
    pointer: T,
    length: u32,
}

// MAIN OBJECTS
#[repr(C)]
pub struct AcpiObjectCommon(AcpiObjectCommonHeader);

#[repr(C)]
pub struct AcpiObjectInteger {
    header: AcpiObjectCommonHeader,
    fill: [u8; 3],
    value: u64,
}

#[repr(C)]
pub struct AcpiObjectString {
    header: AcpiObjectCommonHeader,
    buffer_info: AcpiCommonBufferInfo<c_char>,
}

#[repr(C)]
pub struct AcpiObjectBuffer {
    header: AcpiObjectCommonHeader,
    buffer_info: AcpiCommonBufferInfo<u8>,
    aml_length: u32,
    aml_start: *mut u8,
    node: *mut AcpiNamespaceNode
}

#[repr(C)]
pub struct AcpiObjectPackage {
    header: AcpiObjectCommonHeader,
    node: *mut AcpiNamespaceNode,
    elements: *mut *mut AcpiOperandObject
}

#[repr(C)]
pub struct AcpiObjectEvent {
    header: AcpiObjectCommonHeader,
    os_semaphore: AcpiSemaphore
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
