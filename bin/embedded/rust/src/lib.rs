extern crate rustc_serialize;

use rustc_serialize::json::{self};
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn nth_prime(num: u32) -> u32 {
    let mut i     = 0;
    let mut count = 0;
    while count <= num {
        i += 1;
        if prime(i) {
            count += 1;
        }
    }
    i
}

fn prime(num: u32) -> bool {
    let mut i = 2;
    while i < num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub extern fn read_json(ro: *const c_char) -> *const c_char {
    let roo                                 = make_string(ro);
    let json_for_rust: HashMap<String, i32> = json::decode(&roo).unwrap();
    let encoded                             = json::encode(&json_for_rust).unwrap();
    turn_into_null_string(encoded)
}

fn turn_into_null_string(st: String)  -> *const c_char {
    let concated_string = CString::new(st).unwrap();
    concated_string.into_raw()
}

fn make_string(s1: *const c_char) -> String {
    let s1_cstr    = unsafe { CStr::from_ptr(s1) };  // &std::ffi::c_str::CStr
    let s1_and_str = s1_cstr.to_str().unwrap();  // &str
    let s1_string  = s1_and_str.to_string();  // collections::string::String
    s1_string
}

#[test]
fn it_reads_the_json() {
    let string   = CString::new("{\"wow\":\"6\"}").unwrap().into_raw();
    let ex_st    = "{\"wow\":\"6\"}".to_string();
    let exp      = turn_into_null_string(ex_st);

    let expected1  = make_string(exp);
    let expected2  = make_string(read_json(string));

    assert_eq!(expected1, "{\"wow\":\"6\"}");
    assert_eq!(expected2, "{\"wow\":6}");
}
