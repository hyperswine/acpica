// -----------------
// MODULES
// -----------------

pub mod object;
pub mod local;

// -----------------
// BUILD TYPES & CONSTANTS
// -----------------

use core::ffi::{c_char, c_void};

/// 64-bit target for ACPI
pub const ACPI_MACHINE_WIDTH: u64 = 64;

// -----------------
// COMPONENT TYPES & CONSTANTS
// -----------------

pub const ACPI_UINT8_MAX: u8 = u8::MAX;
pub const ACPI_UINT16_MAX: u16 = u16::MAX;
pub const ACPI_UINT32_MAX: u32 = u32::MAX;
pub const ACPI_UINT64_MAX: u64 = u64::MAX;
pub const ACPI_ASCII_MAX: i32 = 0x7F;

// repr(C, packed) for singular types like Page or some u64 struct
// repr(C) for FFI on complex structs

pub type AcpiStatus = u32;
pub type AcpiName = u32;
pub type AcpiString = *mut c_char;
pub type AcpiHandle = *mut c_void;

pub type AcpiThreadId = u64;

// 64-bit only (we always consider 64-bit)
pub const ACPI_USE_NATIVE_DIVIDE: bool = true;
pub const ACPI_USE_NATIVE_MATH64: bool = true;
pub const ACPI_SIZE_MAX: u64 = ACPI_UINT64_MAX;
pub const ACPI_MAX_PTR: u64 = ACPI_UINT64_MAX;
pub type AcpiSize = u64;
pub type AcpiNativeInt = i64;
pub type AcpiIOAddress = u64;
pub type AcpiPhysicalAddress = u64;

// OS-Dependent
pub type AcpiCpuFlags = AcpiSize;
pub type AcpiSpinlock = *mut c_void;
pub type AcpiSemaphore = *mut c_void;
pub type AcpiMutex = *mut c_void;

// Compiler-Dependent
