// Platform specific constants and types
// EFI and Neutron64
// Now IDK if arch really matters (e.g. arm/riscv/x86) as long as the AcpiOs* functions we implement work properly

use core::{ffi::c_void, ptr::null_mut};

// cfg feature neutron64
pub const ACPI_MACHINE_WIDTH: i32 = 64;

// Types (64-bit)
pub type UIntN = u64;
pub type IntN = i64;

macro_rules! acpi_efi_err {
    ($a:expr) => {
        (0x8000000000000000 | $a)
    };
}

// EDK II environment?
// cfg edk2_efi

// NOTE: for x86_64, need to enable USE_EFI_FUNCTION_WRAPPER
#[cfg(target_arch = "x86_64")]
mod efi_function_wrapper {}

// otherwise use the usual version
// uefi_call_wrapper(func, va_num, ...) func(__VA_ARGS__)
// not called anywhere?

// ACPI EFI INTERFACES
struct _ACPI_SIMPLE_TEXT_OUTPUT_INTERFACE;
struct _ACPI_SIMPLE_INPUT_INTERFACE;
struct _ACPI_EFI_FILE_IO_INTERFACE;
struct _ACPI_EFI_FILE_HANDLE;
struct _ACPI_EFI_BOOT_SERVICES;
struct _ACPI_EFI_RUNTIME_SERVICES;
#[derive(Debug)]
struct _ACPI_EFI_SYSTEM_TABLE {
    pointer: *mut c_void,
}
struct _ACPI_EFI_PCI_IO;

use lazy_static::lazy_static;
use spin::Mutex;
use uefi::table::{Runtime, SystemTable};

// I think we set _ACPI_EFI_SYSTEM_TABLE to point to deadbeef or something
// pub static ST: Mutex<*mut c_void> = 0xdeadbeef;
// lazy_static! {
//     pub static ref ST: Mutex<&'static mut _ACPI_EFI_SYSTEM_TABLE> = null_mut();
// }
