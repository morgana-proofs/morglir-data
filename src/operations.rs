#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
///
/// List of all basic operations
/// notation: `$Name__$($input)_*__$($output)_*`
/// `Pointer$(<$T:ident>)?` means pointer type to T
/// `Type$(<$T:ident>)?` means variable of type T
/// If type parameter is omitted it is same for all generics in signature
///
/// e.g.
/// `Add__Pointer_Field__Pointer` is an operation that has 2 inputs: pointer and a field and 1 output pointer
/// `Write__Pointer_Type__` has 2 inputs and no outputs
/// `Read__Pointer__Type` is the same as `Read__Pointer<T>__Type<T>`
///
pub enum OperationType {
    Add__Field_Field__Field,
    Neg__Field__Field,
    Mul__Field_Field__Field,
    Const____Field,

    Add__Pointer_Field__Pointer,

    Add__U32_U32__U32,
    Neg__U32__U32,
    Mul__U32_U32__U32,

    ToLimbs__Field__U32_U32,
    FromLimbs__U32_U32__Field,

    Alloc__Field__Pointer,
    Realloc__Pointer_Field__Pointer,
    Read__Pointer__Type,
    Write__Pointer_Type__,

    Next____Field,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Operation {  // todo: decide if op_data must itself be typed and if it can replace op itself...
    pub operation_type: OperationType,
    pub data_link: Option<usize>,
}

