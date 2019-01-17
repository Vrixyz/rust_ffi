use std::os::raw::c_char;
use std::ffi::CString;

static mut STRING_POINTER: *mut c_char = 0 as *mut c_char;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct SampleStruct {    
    pub field_one: i32,
    pub field_two: i32,
    pub string_field: *mut c_char,
}

#[no_mangle]
pub extern fn do_stuff(number1: i32, number2: i32) -> SampleStruct {
    let test_string: &'static str = "Hi, I'm a string in rust";
    SampleStruct {
        field_one: number1 + number2,
        field_two: number1 - number2,
        string_field: store_string_on_heap(test_string),
    }
}
#[no_mangle]
pub extern fn do_stuff_with(param: SampleStruct) {
    println!("struct: {:#?}", param);
}

fn store_string_on_heap(string_to_store: &'static str) -> *mut c_char {
    //create a new raw pointer
    let pntr = CString::new(string_to_store).unwrap().into_raw();
    //store it in our static variable (REQUIRES UNSAFE)
    unsafe {
        STRING_POINTER = pntr;
    }
    //return the c_char
    return pntr;
}

#[no_mangle]
pub extern fn free_string() {
    unsafe {
        let _ = CString::from_raw(STRING_POINTER);
        STRING_POINTER = 0 as *mut c_char;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(do_stuff(2, 2), SampleStruct {
            field_one:4,
            field_two:0
        });
    }
}
