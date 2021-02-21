#![no_std]

// use core::panic::PanicInfo;

// struct HStderr {}

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//   let mut host_stderr = HStderr::new();

//   // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
//   writeln!(host_stderr, "{}", info).ok();

//   loop {}
// }

fn main() {
  if libraries::largest::find_largest(&[1, 2]) != 2 {
    todo!()
  }
}
