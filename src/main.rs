#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::*;
use std::os::raw::c_char;
use std::ffi::*;
use std::fmt;

mod vk;
use vk::*;

mod libc;
use libc::*;

fn display_properties(value: &VkPhysicalDeviceProperties) {
    println!("API version   : {:#?}", value.api_version);
    println!("Driver version: {:#?}", value.driver_version);
    println!("Vendor ID     : {:#?}", value.vendor_id);
    println!("Device ID     : {:#?}", value.device_id);
    println!("Device type   : {:#?}", value.device_type);
    println!("Device name   : {:#?}", value.device_name);
}

fn main() {
    let instance = VkInstance::create(&Default::default()).unwrap();
    let physical_devices = instance.get_physical_devices();
    let physical_device = &physical_devices[0];
    
    let properties = physical_device.get_properties();
    let queue_families = physical_device.get_queue_family_properties();
    let queue_family = &queue_families[0];
    let device = physical_device.create_logical_device(&Default::default()).unwrap();

    println!("{:#?}", queue_family);
    display_properties(&properties);

    println!("Bye!");
}