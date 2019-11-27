use std::ffi::CString;
use std::os::raw::c_char;

static HASH: &'static str = "Hash_to_call_contract";
static MESSAGE: &'static str = "i am a message";

// we return a pointer to the hash
// and its length, so the VM can extract
// it from memory after instantiating 
// the contract
#[no_mangle] //optional
pub fn hash_index() -> *mut c_char {
	let index = CString::new(HASH).unwrap();
	index.into_raw()
}

#[no_mangle] //optional
pub fn hash_len() -> i32 {
	HASH.len() as i32
}

// if we want to return strings from a
// contract, we have to define 3 functions
// over our string variables, these are
// fn {name}, fn {name}_index, fn {name}_len

pub fn msg() {
	// this can be empty
	// it only signals the vm
	// to look for potential 
	// memory addresses
}

pub fn msg_index() -> *mut c_char {
	let index = CString::new(MESSAGE).unwrap();
	index.into_raw()
}

pub fn msg_len() -> i32 {
	MESSAGE.len() as i32
}

fn main() { 

}
