use super::object::*;

#[repr(C)]
pub struct AcpiNamespaceNode {
    object: i32
}

#[repr(C)]
pub union AcpiOperandObject {
    Common: AcpiObjectCommon,
    Integer: AcpiObjectInteger,
    String: AcpiObjectString,
    Buffer: AcpiObjectBuffer,
}
