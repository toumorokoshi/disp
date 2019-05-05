#[repr(C)]
pub struct Array<T> {
    pub values: *mut T,
    pub size: i64,
}