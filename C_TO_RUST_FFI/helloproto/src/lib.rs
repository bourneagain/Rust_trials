#[no_mangle]
pub unsafe extern "C" fn add(left: usize, right: usize) -> usize {
   left + right
}
