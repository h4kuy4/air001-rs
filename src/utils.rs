#[macro_export]
macro_rules! set_bit {
    ($val:ident, $offset:expr ,$mask: expr, $new_val: expr) => {
        ($val & !($mask << $offset)) | (($new_val & $mask) << $offset)
    };
}
