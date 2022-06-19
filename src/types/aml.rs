// AML TYPES
// I think its a good idea to make the u32 into u32. Since they are pub const anyway

// ---------------
// PRIMARY OPCODES
// ---------------

pub const AML_ZERO_OP: u16 = 0x00;
pub const AML_ONE_OP: u16 = 0x01;
pub const AML_ALIAS_OP: u16 = 0x06;
pub const AML_NAME_OP: u16 = 0x08;
pub const AML_BYTE_OP: u16 = 0x0a;
pub const AML_WORD_OP: u16 = 0x0b;
pub const AML_DWORD_OP: u16 = 0x0c;
pub const AML_STRING_OP: u16 = 0x0d;
pub const AML_QWORD_OP: u16 = 0x0e;
pub const AML_SCOPE_OP: u16 = 0x10;
pub const AML_BUFFER_OP: u16 = 0x11;
pub const AML_PACKAGE_OP: u16 = 0x12;
pub const AML_VARIABLE_PACKAGE_OP: u16 = 0x13;
pub const AML_METHOD_OP: u16 = 0x14;
pub const AML_EXTERNAL_OP: u16 = 0x15;
pub const AML_DUAL_NAME_PREFIX: u16 = 0x2e;
pub const AML_MULTI_NAME_PREFIX: u16 = 0x2f;
pub const AML_EXTENDED_PREFIX: u16 = 0x5b;
pub const AML_ROOT_PREFIX: u16 = 0x5c;
pub const AML_PARENT_PREFIX: u16 = 0x5e;
pub const AML_FIRST_LOCAL_OP: u16 = 0x60;

// Local Operations
pub const AML_LOCAL0: u16 = 0x60;
pub const AML_LOCAL1: u16 = 0x61;
pub const AML_LOCAL2: u16 = 0x62;
pub const AML_LOCAL3: u16 = 0x63;
pub const AML_LOCAL4: u16 = 0x64;
pub const AML_LOCAL5: u16 = 0x65;
pub const AML_LOCAL6: u16 = 0x66;
pub const AML_LOCAL7: u16 = 0x67;
pub const AML_FIRST_ARG_OP: u16 = 0x68;

// Arg type Ops
pub const AML_ARG0: u16 = 0x68;
pub const AML_ARG1: u16 = 0x69;
pub const AML_ARG2: u16 = 0x6a;
pub const AML_ARG3: u16 = 0x6b;
pub const AML_ARG4: u16 = 0x6c;
pub const AML_ARG5: u16 = 0x6d;
pub const AML_ARG6: u16 = 0x6e;
pub const AML_STORE_OP: u16 = 0x70;
pub const AML_REF_OF_OP: u16 = 0x71;
pub const AML_ADD_OP: u16 = 0x72;
pub const AML_CONCATENATE_OP: u16 = 0x73;
pub const AML_SUBTRACT_OP: u16 = 0x74;
pub const AML_INCREMENT_OP: u16 = 0x75;
pub const AML_DECREMENT_OP: u16 = 0x76;
pub const AML_MULTIPLY_OP: u16 = 0x77;
pub const AML_DIVIDE_OP: u16 = 0x78;
pub const AML_SHIFT_LEFT_OP: u16 = 0x79;
pub const AML_SHIFT_RIGHT_OP: u16 = 0x7a;
pub const AML_BIT_AND_OP: u16 = 0x7b;
pub const AML_BIT_NAND_OP: u16 = 0x7c;
pub const AML_BIT_OR_OP: u16 = 0x7d;
pub const AML_BIT_NOR_OP: u16 = 0x7e;
pub const AML_BIT_XOR_OP: u16 = 0x7f;
pub const AML_BIT_NOT_OP: u16 = 0x80;
pub const AML_FIND_SET_LEFT_BIT_OP: u16 = 0x81;
pub const AML_FIND_SET_RIGHT_BIT_OP: u16 = 0x82;
pub const AML_DEREF_OF_OP: u16 = 0x83;
pub const AML_CONCATENATE_TEMPLATE_OP: u16 = 0x84;
pub const AML_MOD_OP: u16 = 0x85;
pub const AML_NOTIFY_OP: u16 = 0x86;
pub const AML_SIZE_OF_OP: u16 = 0x87;
pub const AML_INDEX_OP: u16 = 0x88;
pub const AML_MATCH_OP: u16 = 0x89;
pub const AML_CREATE_DWORD_FIELD_OP: u16 = 0x8a;
pub const AML_CREATE_WORD_FIELD_OP: u16 = 0x8b;
pub const AML_CREATE_BYTE_FIELD_OP: u16 = 0x8c;
pub const AML_CREATE_BIT_FIELD_OP: u16 = 0x8d;
pub const AML_OBJECT_TYPE_OP: u16 = 0x8e;
pub const AML_CREATE_QWORD_FIELD_OP: u16 = 0x8f;
pub const AML_LOGICAL_AND_OP: u16 = 0x90;
pub const AML_LOGICAL_OR_OP: u16 = 0x91;
pub const AML_LOGICAL_NOT_OP: u16 = 0x92;
pub const AML_LOGICAL_EQUAL_OP: u16 = 0x93;
pub const AML_LOGICAL_GREATER_OP: u16 = 0x94;
pub const AML_LOGICAL_LESS_OP: u16 = 0x95;
pub const AML_TO_BUFFER_OP: u16 = 0x96;
pub const AML_TO_DECIMAL_STRING_OP: u16 = 0x97;
pub const AML_TO_HEX_STRING_OP: u16 = 0x98;
pub const AML_TO_INTEGER_OP: u16 = 0x99;
pub const AML_TO_STRING_OP: u16 = 0x9c;
pub const AML_COPY_OBJECT_OP: u16 = 0x9d;
pub const AML_MID_OP: u16 = 0x9e;
pub const AML_CONTINUE_OP: u16 = 0x9f;
pub const AML_IF_OP: u16 = 0xa0;
pub const AML_ELSE_OP: u16 = 0xa1;
pub const AML_WHILE_OP: u16 = 0xa2;
pub const AML_NOOP_OP: u16 = 0xa3;
pub const AML_RETURN_OP: u16 = 0xa4;
pub const AML_BREAK_OP: u16 = 0xa5;
pub const AML_COMMENT_OP: u16 = 0xa9;
pub const AML_BREAKPOINT_OP: u16 = 0xcc;
pub const AML_ONES_OP: u16 = 0xff;

// ---------------
// COMBINATION OPS
// ---------------

// Mostly used by the disassembler and iASL compiler
// Still 2 Bytes, but each byte is used instead of just the LSBs

// Non prefixed
pub const AML_LOGICAL_GREATER_EQUAL_OP: u16 = 0x929;
pub const AML_LOGICAL_LESS_EQUAL_OP: u16 = 0x929;
pub const AML_LOGICAL_NOT_EQUAL_OP: u16 = 0x929;

// 0x5B prefixed
pub const AML_EXTENDED_OPCODE: u16 = 0x5b00;
pub const AML_MUTEX_OP: u16 = 0x5b01;
pub const AML_EVENT_OP: u16 = 0x5b02;
pub const AML_SHIFT_RIGHT_BIT_OP: u16 = 0x5b10;
pub const AML_SHIFT_LEFT_BIT_OP: u16 = 0x5b11;
pub const AML_CONDITIONAL_REF_OF_OP: u16 = 0x5b12;
pub const AML_CREATE_FIELD_OP: u16 = 0x5b13;
pub const AML_LOAD_TABLE_OP: u16 = 0x5b1f;
pub const AML_LOAD_OP: u16 = 0x5b20;
pub const AML_STALL_OP: u16 = 0x5b21;
pub const AML_SLEEP_OP: u16 = 0x5b22;
pub const AML_ACQUIRE_OP: u16 = 0x5b23;
pub const AML_SIGNAL_OP: u16 = 0x5b24;
pub const AML_WAIT_OP: u16 = 0x5b25;
pub const AML_RESET_OP: u16 = 0x5b26;
pub const AML_RELEASE_OP: u16 = 0x5b27;
pub const AML_FROM_BCD_OP: u16 = 0x5b28;
pub const AML_TO_BCD_OP: u16 = 0x5b29;
pub const AML_UNLOAD_OP: u16 = 0x5b2a;
pub const AML_REVISION_OP: u16 = 0x5b30;
pub const AML_DEBUG_OP: u16 = 0x5b31;
pub const AML_FATAL_OP: u16 = 0x5b32;
pub const AML_TIMER_OP: u16 = 0x5b33;
pub const AML_REGION_OP: u16 = 0x5b80;
pub const AML_FIELD_OP: u16 = 0x5b81;
pub const AML_DEVICE_OP: u16 = 0x5b82;
pub const AML_PROCESSOR_OP: u16 = 0x5b83;
pub const AML_POWER_RESOURCE_OP: u16 = 0x5b84;
pub const AML_THERMAL_ZONE_OP: u16 = 0x5b85;
pub const AML_INDEX_FIELD_OP: u16 = 0x5b86;
pub const AML_BANK_FIELD_OP: u16 = 0x5b87;
pub const AML_DATA_REGION_OP: u16 = 0x5b88;

// ---------------
// FIELD OPERATORS
// ---------------

pub const AML_FIELD_OFFSET_OP: u8 = 0x00;
pub const AML_FIELD_ACCESS_OP: u8 = 0x01;
pub const AML_FIELD_CONNECTION_OP: u8 = 0x02;
pub const AML_FIELD_EXT_ACCESS_OP: u8 = 0x03;

// ---------------
// INTERNAL OPCODE
// ---------------

pub const AML_INT_NAMEPATH_OP: u16 = 0x002d;
pub const AML_INT_NAMEDFIELD_OP: u16 = 0x0030;
pub const AML_INT_RESERVEDFIELD_OP: u16 = 0x0031;
pub const AML_INT_ACCESSFIELD_OP: u16 = 0x0032;
pub const AML_INT_BYTELIST_OP: u16 = 0x0033;
pub const AML_INT_METHODCALL_OP: u16 = 0x0035;
pub const AML_INT_RETURN_VALUE_OP: u16 = 0x0036;
pub const AML_INT_EVAL_SUBTREE_OP: u16 = 0x0037;
pub const AML_INT_CONNECTION_OP: u16 = 0x0038;
pub const AML_INT_EXTACCESSFIELD_OP: u16 = 0x0039;

// Special, doesnt matter if INT32 or UINT32, though maybe neg zero. I think that just gets casted to unsigned 0 anyway / and with beqz prob treats as the same
pub const ARG_NONE: u32 = 0x0;

// ---------------
// ARGUMENT TYPES
// ---------------

// For the parser

pub const ARGP_BYTEDATA: u32 = 0x01;
pub const ARGP_BYTELIST: u32 = 0x02;
pub const ARGP_CHARLIST: u32 = 0x03;
pub const ARGP_DATAOBJ: u32 = 0x04;
pub const ARGP_DATAOBJLIST: u32 = 0x05;
pub const ARGP_DWORDDATA: u32 = 0x06;
pub const ARGP_FIELDLIST: u32 = 0x07;
pub const ARGP_NAME: u32 = 0x08;
pub const ARGP_NAMESTRING: u32 = 0x09;
pub const ARGP_OBJLIST: u32 = 0x0A;
pub const ARGP_PKGLENGTH: u32 = 0x0B;
pub const ARGP_SUPERNAME: u32 = 0x0C;
pub const ARGP_TARGET: u32 = 0x0D;
pub const ARGP_TERMARG: u32 = 0x0E;
pub const ARGP_TERMLIST: u32 = 0x0F;
pub const ARGP_WORDDATA: u32 = 0x10;
pub const ARGP_QWORDDATA: u32 = 0x11;
pub const ARGP_SIMPLENAME: u32 = 0x12;
pub const ARGP_NAME_OR_REF: u32 = 0x13;
pub const ARGP_MAX: u32 = 0x13;
pub const ARGP_COMMENT: u32 = 0x14;

// ---------------
// RESOLVED ARGUMENT TYPES
// ---------------

pub const ARGI_ANYTYPE: u32 = 0x01;
pub const ARGI_PACKAGE: u32 = 0x02;
pub const ARGI_EVENT: u32 = 0x03;
pub const ARGI_MUTEX: u32 = 0x04;
pub const ARGI_DDBHANDLE: u32 = 0x05;
pub const ARGI_INTEGER: u32 = 0x06;
pub const ARGI_STRING: u32 = 0x07;
pub const ARGI_BUFFER: u32 = 0x08;
pub const ARGI_BUFFER_OR_STRING: u32 = 0x09;
pub const ARGI_COMPUTEDATA: u32 = 0x0A;
pub const ARGI_INTEGER_REF: u32 = 0x0B;
pub const ARGI_OBJECT_REF: u32 = 0x0C;
pub const ARGI_DEVICE_REF: u32 = 0x0D;
pub const ARGI_REFERENCE: u32 = 0x0E;
pub const ARGI_TARGETREF: u32 = 0x0F;
pub const ARGI_FIXED_TARGET: u32 = 0x10;
pub const ARGI_SIMPLE_TARGET: u32 = 0x11;
pub const ARGI_STORE_TARGET: u32 = 0x12;
pub const ARGI_DATAOBJECT: u32 = 0x13;
pub const ARGI_COMPLEXOBJ: u32 = 0x14;
pub const ARGI_REF_OR_STRING: u32 = 0x15;
pub const ARGI_REGION_OR_BUFFER: u32 = 0x16;
pub const ARGI_DATAREFOBJ: u32 = 0x17;
pub const ARGI_INVALID_OPCODE: u32 = 0xFFFFFFFF;

// ---------------
// OPCODE FLAGS
// ---------------

pub const AML_LOGICAL: u32 = 0x0001;
pub const AML_LOGICAL_NUMERIC: u32 = 0x0002;
pub const AML_MATH: u32 = 0x0004;
pub const AML_CREATE: u32 = 0x0008;
pub const AML_FIELD: u32 = 0x0010;
pub const AML_DEFER: u32 = 0x0020;
pub const AML_NAMED: u32 = 0x0040;
pub const AML_NSNODE: u32 = 0x0080;
pub const AML_NSOPCODE: u32 = 0x0100;
pub const AML_NSOBJECT: u32 = 0x0200;
pub const AML_HAS_RETVAL: u32 = 0x0400;
pub const AML_HAS_TARGET: u32 = 0x0800;
pub const AML_HAS_ARGS: u32 = 0x1000;
pub const AML_CONSTANT: u32 = 0x2000;
pub const AML_NO_OPERAND_RESOLVE: u32 = 0x4000;

// Convenient flag combos
pub const AML_FLAGS_EXEC_0A_0T_1R: u32 = AML_HAS_RETVAL;
pub const AML_FLAGS_EXEC_1A_0T_0R: u32 = AML_HAS_ARGS;
pub const AML_FLAGS_EXEC_1A_0T_1R: u32 = AML_HAS_ARGS | AML_HAS_RETVAL;
pub const AML_FLAGS_EXEC_1A_1T_0R: u32 = AML_HAS_ARGS | AML_HAS_TARGET;
pub const AML_FLAGS_EXEC_1A_1T_1R: u32 = AML_HAS_ARGS | AML_HAS_TARGET | AML_HAS_RETVAL;
pub const AML_FLAGS_EXEC_2A_0T_0R: u32 = AML_HAS_ARGS;
pub const AML_FLAGS_EXEC_2A_0T_1R: u32 = AML_HAS_ARGS | AML_HAS_RETVAL;
pub const AML_FLAGS_EXEC_2A_1T_1R: u32 = AML_HAS_ARGS | AML_HAS_TARGET | AML_HAS_RETVAL;
pub const AML_FLAGS_EXEC_2A_2T_1R: u32 = AML_HAS_ARGS | AML_HAS_TARGET | AML_HAS_RETVAL;
pub const AML_FLAGS_EXEC_3A_0T_0R: u32 = AML_HAS_ARGS;
pub const AML_FLAGS_EXEC_3A_1T_1R: u32 = AML_HAS_ARGS | AML_HAS_TARGET | AML_HAS_RETVAL;
pub const AML_FLAGS_EXEC_6A_0T_1R: u32 = AML_HAS_ARGS | AML_HAS_RETVAL;

// ---------------
// OPCODE TYPES
// ---------------

pub const AML_TYPE_EXEC_0A_0T_1R: u32 = 0x00;
pub const AML_TYPE_EXEC_1A_0T_0R: u32 = 0x01;
pub const AML_TYPE_EXEC_1A_0T_1R: u32 = 0x02;
pub const AML_TYPE_EXEC_1A_1T_0R: u32 = 0x03;
pub const AML_TYPE_EXEC_1A_1T_1R: u32 = 0x04;
pub const AML_TYPE_EXEC_2A_0T_0R: u32 = 0x05;
pub const AML_TYPE_EXEC_2A_0T_1R: u32 = 0x06;
pub const AML_TYPE_EXEC_2A_1T_1R: u32 = 0x07;
pub const AML_TYPE_EXEC_2A_2T_1R: u32 = 0x08;
pub const AML_TYPE_EXEC_3A_0T_0R: u32 = 0x09;
pub const AML_TYPE_EXEC_3A_1T_1R: u32 = 0x0A;
pub const AML_TYPE_EXEC_6A_0T_1R: u32 = 0x0B;
pub const AML_TYPE_LITERAL: u32 = 0x0C;
pub const AML_TYPE_CONSTANT: u32 = 0x0D;
pub const AML_TYPE_METHOD_ARGUMENT: u32 = 0x0E;
pub const AML_TYPE_LOCAL_VARIABLE: u32 = 0x0F;
pub const AML_TYPE_DATA_TERM: u32 = 0x10;
pub const AML_TYPE_METHOD_CALL: u32 = 0x11;
pub const AML_TYPE_CREATE_FIELD: u32 = 0x12;
pub const AML_TYPE_CREATE_OBJECT: u32 = 0x13;
pub const AML_TYPE_CONTROL: u32 = 0x14;
pub const AML_TYPE_NAMED_NO_OBJ: u32 = 0x15;
pub const AML_TYPE_NAMED_FIELD: u32 = 0x16;
pub const AML_TYPE_NAMED_SIMPLE: u32 = 0x17;
pub const AML_TYPE_NAMED_COMPLEX: u32 = 0x18;
pub const AML_TYPE_RETURN: u32 = 0x19;
pub const AML_TYPE_UNDEFINED: u32 = 0x1A;
pub const AML_TYPE_BOGUS: u32 = 0x1B;

pub const ACPI_AML_PACKAGE_TYPE1: u32 = 0x40;
pub const ACPI_AML_PACKAGE_TYPE2: u32 = 0x4000;
pub const ACPI_AML_PACKAGE_TYPE3: u32 = 0x400000;
pub const ACPI_AML_PACKAGE_TYPE4: u32 = 0x40000000;

// ---------------
// OPCODE CLASSES
// ---------------

pub const AML_CLASS_EXECUTE: u32 = 0x00;
pub const AML_CLASS_CREATE: u32 = 0x01;
pub const AML_CLASS_ARGUMENT: u32 = 0x02;
pub const AML_CLASS_NAMED_OBJECT: u32 = 0x03;
pub const AML_CLASS_CONTROL: u32 = 0x04;
pub const AML_CLASS_ASCII: u32 = 0x05;
pub const AML_CLASS_PREFIX: u32 = 0x06;
pub const AML_CLASS_INTERNAL: u32 = 0x07;
pub const AML_CLASS_RETURN_VALUE: u32 = 0x08;
pub const AML_CLASS_METHOD_CALL: u32 = 0x09;
pub const AML_CLASS_UNKNOWN: u32 = 0x0A;

/// Comparison operation codes for MatchOp operator
#[repr(C)]
pub enum AmlMatchOperator {
    MatchMtr = 0,
    MatchMeq = 1,
    MatchMle = 2,
    MatchMlt = 3,
    MatchMge = 4,
    MatchMgt = 5,
}

pub const MAX_MATCH_OPERATOR: u32 = 5;

// ---------------
// FIELD FLAGS
// ---------------

pub const AML_FIELD_ACCESS_TYPE_MASK: u32 = 0x0F;
pub const AML_FIELD_LOCK_RULE_MASK: u32 = 0x10;
pub const AML_FIELD_UPDATE_RULE_MASK: u32 = 0x60;

/// Field Access Types
#[repr(C)]
pub enum AmlAccessType {
    AmlFieldAccessAny0 = 0x00,
    AmlFieldAccessByte = 0x01,
    AmlFieldAccessWord = 0x02,
    AmlFieldAccessDword0x03 = 0x03,
    /// ACPI 2.0 only
    AmlFieldAccessQword0x04 = 0x04,
    /// ACPI 2.0 only
    AmlFieldAccessBuffer0x05 = 0x05,
}

/// Field Lock Rules
#[repr(C)]
pub enum AmlLockRule {
    AmlFieldLockNever = 0x00,
    AmlFieldLockAlways = 0x10,
}

/// Field Update Rules
#[repr(C)]
pub enum AmlUpdateRule {
    AmlFieldUpdatePreserve = 0x00,
    AmlFieldUpdateWriteAsOnes = 0x20,
    AmlFieldUpdateWriteAsZeros = 0x40,
}

// ---------------
// FIELD ACCESS ATTRIBUTES
// ---------------

#[repr(C)]
enum AmlAccessAttribute {
    AmlFieldAttribQuick = 0x02,
    AmlFieldAttribSendReceive = 0x04,
    AmlFieldAttribByte = 0x06,
    AmlFieldAttribWord = 0x08,
    AmlFieldAttribBlock = 0x0A,
    AmlFieldAttribBytes = 0x0B,
    AmlFieldAttribProcessCall = 0x0C,
    AmlFieldAttribBlockProcessCall = 0x0D,
    AmlFieldAttribRawBytes = 0x0E,
    AmlFieldAttribRawProcessBytes = 0x0F,
}

/* Bit fields in the AML MethodFlags byte */

pub const AML_METHOD_ARG_COUNT: u32 = 0x07;
pub const AML_METHOD_SERIALIZED: u32 = 0x08;
pub const AML_METHOD_SYNC_LEVEL: u32 = 0xF0;
