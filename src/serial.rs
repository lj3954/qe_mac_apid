use anyhow::{Context, Result};
use std::{
    ffi::{c_char, CStr, CString},
    io::{stdin, stdout, Write},
};

const APPLE_CHECK_COVERAGE_URL: &str = "https://checkcoverage.apple.com/";

pub fn find_desired(model_name: &str) -> Result<Serial> {
    println!("To find a valid serial number, you'll need to visit Apple's Check Coverage page: {APPLE_CHECK_COVERAGE_URL}");
    print!("Do you want the page to be opened in your default browser? (y/N) ");
    stdout().flush()?;
    if yes_input()? {
        open::that(APPLE_CHECK_COVERAGE_URL)?;
    }

    loop {
        let serial = Serial::new(model_name)?;
        println!("\nSerial number: {}", serial.serial_number);
        print!("Have you received an error (\"unable to check coverage\")? (y/N) ");
        stdout().flush()?;
        if yes_input()? {
            return Ok(serial);
        }
    }
}

fn yes_input() -> Result<bool> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim();

    Ok(buffer.eq_ignore_ascii_case("y") || buffer.eq_ignore_ascii_case("yes"))
}

#[derive(Debug, Clone)]
pub struct Serial {
    pub serial_number: String,
    pub board_serial: String,
}

impl Serial {
    pub fn new(model_name: &str) -> Result<Self> {
        // SAFETY: We're passing a valid null-terminated CString to a C function guarantees a
        // return of a valid SERIALINFO struct
        let mut board_index = unsafe {
            crate::modelinfo::find_model_info(CString::new(model_name).unwrap().as_ptr())
        };

        // SAFETY: We're passing a pointer to a stack-allocated struct which will not be
        // deallocated into a C function which guarantees the return of a valid SerialResult
        let serial_data = unsafe { crate::modelinfo::find_serial_mlb(&raw mut board_index) };

        // SAFETY: serial refers to a stack-allocated null-terminated c_char array
        let serial_number = unsafe { c_str_to_string(serial_data.serial.as_ptr()) }
            .context("Serial number could not be generated")?;

        // SAFETY: mlb refers to a stack-allocated null-terminated c_char array
        let board_serial = unsafe { c_str_to_string(serial_data.mlb.as_ptr()) }
            .context("Board serial could not be generated")?;

        Ok(Serial {
            serial_number,
            board_serial,
        })
    }
}

unsafe fn c_str_to_string(c_str: *const c_char) -> Option<String> {
    if c_str.is_null() {
        None
    } else {
        Some(CStr::from_ptr(c_str).to_string_lossy().into_owned())
    }
}
