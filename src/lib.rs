#![deny(clippy::all)]

use napi_derive::napi;
use printers;

#[napi]
pub fn get_printers() -> Vec<String> {
  return printers::get_printers().into_iter().map(|printer| printer.name).collect();
}

#[napi]
pub fn print(printer_name: String, buffer: &[u8]) {
  let printer = printers::get_printer_by_name(&printer_name).unwrap();
  let _ = printer.print(buffer, None);
}