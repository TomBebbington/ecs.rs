use std::intrinsics::TyDesc;
use std::intrinsics;
use std::mem;

#[inline(always)]
pub fn get_type_name<T>() -> &'static str {
    let val: &TyDesc = unsafe { mem::transmute(intrinsics::get_tydesc::<T>()) };
    val.name
}