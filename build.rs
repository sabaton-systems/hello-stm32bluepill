fn main() {
    let mut b = freertos_cargo_build::Builder::new();

    // Path to FreeRTOS kernel or set ENV "FREERTOS_SRC" instead
    b.freertos("FreeRTOS-Kernel");
    b.freertos_config("src"); // Location of `FreeRTOSConfig.h`
    b.freertos_port("GCC/ARM_CM3".into()); // Port dir relativ to 'FreeRTOS-Kernel/portable'
                                           //b.heap("heap4.c".into());              // Set the heap_?.c allocator to use from

    // b.get_cc().file("More.c");   // Optional additional C-Code to be compiled

    b.compile().unwrap_or_else(|e| panic!("{}", e.to_string()));
}
