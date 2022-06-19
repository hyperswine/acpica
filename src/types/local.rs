use super::{AcpiMutex, AcpiSemaphore, ACPI_THREAD_ID};
use core::ffi::c_void;
use std::println;

// --------------
// CONSTANTS
// --------------

pub const ACPI_SERIALIZED: u32 = 0xFF;
/// Should be '-1' casted to a void pointer instead of usize::max, but idk lol
pub const ACPI_GLOBAL_LOCK: AcpiSemaphore = usize::MAX as *mut c_void;
pub const AML_NUM_OPCODES: u32 = 0x83;

// Predefined handles for acpi mutexes
pub const ACPI_MTX_INTERPRETER: u32 = 0;
pub const ACPI_MTX_NAMESPACE: u32 = 1;
pub const ACPI_MTX_TABLES: u32 = 2;
pub const ACPI_MTX_EVENTS: u32 = 3;
pub const ACPI_MTX_CACHES: u32 = 4;
pub const ACPI_MTX_MEMORY: u32 = 5;
pub const ACPI_MAX_MUTEX: u32 = 5;
pub const ACPI_NUM_MUTEX: u32 = ACPI_MAX_MUTEX + 1;

// Predefined handles for acpi spinlocks (Technically UTMutex)
pub const ACPI_LOCK_GPES: u32 = 0;
pub const ACPI_LOCK_HARDWARE: u32 = 1;
pub const ACPI_MAX_LOCK: u32 = 1;
pub const ACPI_NUM_LOCK: u32 = (ACPI_MAX_LOCK + 1);
pub const ACPI_MUTEX_NOT_ACQUIRED: ACPI_THREAD_ID = -1;
pub const ACPI_INVALID_THREAD_ID: ACPI_THREAD_ID = 0xFFFFFFFF;

// --------------
// TYPES
// --------------

type AcpiMutexHandle = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiNamespaceNode {
    object: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiRwLock {
    writer_mutex: AcpiMutex,
    reader_mutex: AcpiMutex,
    number_of_readers: u32,
}

// --------------
// TESTS
// --------------

#[test]
fn test_acpi_local_types() {
    // println!("x = {ACPI_MUTEX_NOT_ACQUIRED}");
}
