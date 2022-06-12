// -----------------
// MODULES
// -----------------

// NOTE: does not include any debug statistics (ACPI_DBG_TRACK_ALLOCATIONS)

pub mod local;
pub mod object;

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
// assume no defined cache, otherwise use ACPI_MEMORY_LIST
pub type AcpiCache = *mut c_void;

// Compiler-Dependent

// MEMORY
pub struct AcpiMemoryList {
    list_name: *const c_char,
    list_head: *const c_void,
    object_size: u16,
    max_depth: u16,
    curr_depth: u16,
}

// BitRegister IDs

/* PM1 Status register */
pub const ACPI_BITREG_TIMER_STATUS: i32 = 0x00;
pub const ACPI_BITREG_BUS_MASTER_STATUS: i32 = 0x01;
pub const ACPI_BITREG_GLOBAL_LOCK_STATUS: i32 = 0x02;
pub const ACPI_BITREG_POWER_BUTTON_STATUS: i32 = 0x03;
pub const ACPI_BITREG_SLEEP_BUTTON_STATUS: i32 = 0x04;
pub const ACPI_BITREG_RT_CLOCK_STATUS: i32 = 0x05;
pub const ACPI_BITREG_WAKE_STATUS: i32 = 0x06;
pub const ACPI_BITREG_PCIEXP_WAKE_STATUS: i32 = 0x07;

/* PM1 Enable register */
pub const ACPI_BITREG_TIMER_ENABLE: i32 = 0x08;
pub const ACPI_BITREG_GLOBAL_LOCK_ENABLE: i32 = 0x09;
pub const ACPI_BITREG_POWER_BUTTON_ENABLE: i32 = 0x0A;
pub const ACPI_BITREG_SLEEP_BUTTON_ENABLE: i32 = 0x0B;
pub const ACPI_BITREG_RT_CLOCK_ENABLE: i32 = 0x0C;
pub const ACPI_BITREG_PCIEXP_WAKE_DISABLE: i32 = 0x0D;

/* PM1 Control register */
pub const ACPI_BITREG_SCI_ENABLE: i32 = 0x0E;
pub const ACPI_BITREG_BUS_MASTER_RLD: i32 = 0x0F;
pub const ACPI_BITREG_GLOBAL_LOCK_RELEASE: i32 = 0x10;
pub const ACPI_BITREG_SLEEP_TYPE: i32 = 0x11;
pub const ACPI_BITREG_SLEEP_ENABLE: i32 = 0x12;

/* PM2 Control register */
pub const ACPI_BITREG_ARB_DISABLE: i32 = 0x13;
pub const ACPI_BITREG_MAX: i32 = 0x13;
pub const ACPI_NUM_BITREG: i32 = ACPI_BITREG_MAX + 1;

/* Status register values. A 1 clears a status bit. 0 = no effect */
pub const ACPI_CLEAR_STATUS: i32 = 1;

/* Enable and Control register values */
pub const ACPI_ENABLE_EVENT: i32 = 1;
pub const ACPI_DISABLE_EVENT: i32 = 0;
