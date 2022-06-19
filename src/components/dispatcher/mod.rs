// -----------------
// DYNAMIC ARGUMENTS
// -----------------

// execute dynamic arguments for static objs

use crate::types::{local::AcpiNamespaceNode, AcpiStatus};

fn acpi_ds_execute_arguments(
    node: *mut AcpiNamespaceNode,
    scope_node: *mut AcpiNamespaceNode,
    aml_length: u32,
    aml_start: u8,
) -> AcpiStatus {
    let mut status = 0;

    let parse_object = acpi_ds_alloc_op()

    status
}
