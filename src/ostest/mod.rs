use qemu_exit::QEMUExit;

pub fn test_runner(tests: &[&dyn Fn()]) {
    let io_base = &env!("IO_DEBUG_BASE").parse::<u16>().unwrap();
    let return_code = &env!("TEST_RETURN_CODE").parse::<u32>().unwrap();
    let qemu_exit_handle = qemu_exit::X86::new(*io_base, *return_code);
    crate::serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu_exit_handle.exit_success()
}