use std::mem;
use std::ptr;
use std::os::raw::c_char;
use utils::c_bindings::*;
use utils::vk_traits::*;

const SIZE_OF_PTR : usize = mem::size_of::<*const u8>();

pub fn free_ptr<T>(ptr: *const T) {
    unsafe {
        free(mem::transmute(ptr));
    }
}

pub fn free_vk_ptr<T : VkFree>(ptr: *const T) {
    unsafe {
        if !ptr.is_null() {
            (&*ptr).vk_free();
            free_ptr(ptr);
        }

    }
}

pub fn free_vk_ptr_array<T : VkFree>(size: usize, ptr: *const T) {
    unsafe {
        if !ptr.is_null() {
            for i in 0..size {
                (&*ptr.add(i)).vk_free();
            }

            free_ptr(ptr);
        }
    }
}

pub fn free_vk_ptr_array_array<T : VkFree>(size: usize, ptr: *const *const T) {
    unsafe {
        if !ptr.is_null() {
            for i in 0..size {
                let addr = *ptr.add(i);
                (&*addr).vk_free();
            }

            free_ptr(ptr);
        }
    }
}

pub fn new_ptr_value<R : Copy>(value: R) -> *mut R {
    unsafe {
        let ptr = malloc(mem::size_of::<R>()) as *mut R;
        *ptr = value;

        ptr
    }
}

pub fn new_ptr_array<R : Copy>(array: &[R]) -> *mut R {
    unsafe {
        if array.len() == 0 {
            return ptr::null_mut()
        }

        let ptr = malloc(mem::size_of::<R>() * array.len()) as *mut R;

        for i in 0..array.len() {
            *ptr.add(i) = array[i];
        }

        ptr
    }
}

pub fn new_ptr_vk_value<R, W : VkWrappedType<R>>(value: &W) -> *const R {
    unsafe {
        let ptr = malloc(mem::size_of::<R>()) as *mut R;
        let dst = ptr.as_mut().unwrap();
        W::vk_to_raw(value, dst);

        ptr as *const R
    }
}

pub fn new_ptr_vk_value_checked<R, W : VkWrappedType<R>>(value: &Option<W>) -> *const R {
    match value {
        Some(v) => new_ptr_vk_value(v),
        None => ptr::null()
    }
}

pub fn new_ptr_vk_array<R, W : VkWrappedType<R>>(array: &[W]) -> *const R {
    unsafe {
        if array.len() == 0 {
            return ptr::null()
        }

        let byte_len = array.len() * mem::size_of::<R>();
        let ptr = malloc(byte_len) as *mut R;

        for i in 0..array.len() {
            let dst = ptr.add(i).as_mut().unwrap();
            W::vk_to_raw(&array[i], dst);
        }

        ptr as *const R
    }
}

pub fn new_ptr_vk_array_checked<R, W : VkWrappedType<R>>(array: &Option<Vec<W>>) -> *const R {
    match array {
        Some(v) => new_ptr_vk_array(v),
        None => ptr::null()
    }
}

pub fn new_ptr_string(string: &str) -> *const c_char {
    unsafe {
        let bytes = string.as_bytes();
        let len = bytes.len();
        let ptr = malloc(len + 1) as *mut c_char;

        for i in 0..len {
            *ptr.add(i) = bytes[i] as c_char;
        }

        *ptr.add(len) = 0;

        ptr as *const c_char
    }
}

pub fn new_ptr_string_checked(string: &Option<String>) -> *const c_char {
    match string {
        Some(value) => new_ptr_string(&value),
        None => ptr::null()
    }
}

pub fn new_ptr_string_array(array: &[String]) -> *const *const c_char {
    unsafe {
        let nb_strings = array.len();
        let mut total_strings_len : usize = 0;

        for i in 0..array.len() {
            total_strings_len += array[i].len();
        }

        let byte_len = total_strings_len + (SIZE_OF_PTR + 1) * nb_strings;
        let ptr = malloc(byte_len) as *mut c_char;
        let addr_ptr = ptr as *mut *mut c_char;
        let mut write_start_addr = ptr.add(SIZE_OF_PTR * nb_strings);

        for i in 0..nb_strings {
            let bytes = array[i].as_bytes();
            let len = bytes.len();

            *addr_ptr.add(i) = write_start_addr;

            for j in 0..len {
                *write_start_addr.add(j) = bytes[j] as c_char;
            }

            *write_start_addr.add(len) = 0;
            write_start_addr = write_start_addr.add(len + 1);
        }

        addr_ptr as *const *const c_char
    }
}

pub fn new_ptr_vk_array_array<R, W : VkWrappedType<R>>(array: &[W]) -> *const *const R {
    unsafe {
        let nb_elements = array.len();
        if nb_elements == 0 {
            return ptr::null()
        }

        let byte_len = nb_elements * (SIZE_OF_PTR + mem::size_of::<R>());
        let ptr = malloc(byte_len);
        let ptr_addr = ptr as *mut *mut R;
        let ptr_content = ptr.add(SIZE_OF_PTR * nb_elements) as *mut R;

        for i in 0..nb_elements {
            let elt_ptr = ptr_content.add(i);
            W::vk_to_raw(&array[i], &mut *elt_ptr);
            *ptr_addr.add(i) = elt_ptr;
        }

        ptr_addr as *const *const R
    }
}

pub fn get_array_option_len<T>(value: &Option<Vec<T>>) -> usize {
    match value {
        Some(array) => array.len(),
        None => 0
    }
}

pub fn vec_option_to_ptr<T>(value: &Option<Vec<T>>) -> *const T {
    match value {
        Some(array) => array.as_ptr(),
        None => ptr::null()
    }
}