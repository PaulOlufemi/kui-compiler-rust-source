#![no_std]
use core::panic::PanicInfo;

#[cfg(target_os = "windows")]
mod startup {
    use core::arch::global_asm;
    // lib.rs or apptype.rs
    // #[unsafe(no_mangle)]
    // #[cfg(not(feature = "gui"))]
    // pub static __mingw_app_type: i32 = 0; // console

    // #[unsafe(no_mangle)]
    // #[cfg(feature = "gui")]
    // pub static __mingw_app_type: i32 = 1; // GUI

    global_asm!(
        ".global mainCRTStartup",
        "mainCRTStartup:",
        "and rsp, -16",
        "call __kui_runtime_init_inner",
        "call main",
        "mov rcx, rax",
        "call ExitProcess",
    );

    global_asm!(
        ".global WinMainCRTStartup",
        "WinMainCRTStartup:",
        "and rsp, -16",
        "call __kui_runtime_init_inner",
        "call main",
        "mov rcx, rax",
        "call ExitProcess",
    );

    unsafe extern "system" {
        fn ExitProcess(code: u32) -> !;
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __kui_runtime_init_inner() {
        unsafe {
            SetConsoleCP(65001);
            SetConsoleOutputCP(65001);
        }
    }

    unsafe extern "system" {
        fn SetConsoleCP(wCodePageID: u32) -> i32;
        fn SetConsoleOutputCP(wCodePageID: u32) -> i32;
    }
}

#[cfg(target_os = "linux")]
mod startup {
    use core::arch::global_asm;

    // Linux x86_64 — OS calls _start directly
    // rsp points to argc, then argv, then envp
    global_asm!(
        ".global _start",
        "_start:",
        "xor rbp, rbp",          // mark outermost frame
        "and rsp, -16",          // align stack
        "call __kui_runtime_init_inner",
        "call main",
        // exit syscall
        "mov rdi, rax",          // exit code
        "mov rax, 60",           // SYS_exit
        "syscall",
    );

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __kui_runtime_init_inner() {
        // Linux init — set locale, signal handlers etc
        // locale is usually handled by libc but since we're no_std
        // just leave empty for now
    }
}

#[cfg(target_os = "macos")]
mod startup {
    use core::arch::global_asm;

    // macOS x86_64 — same as Linux but different syscall number
    global_asm!(
        ".global _main",
        "_main:",
        "xor rbp, rbp",
        "and rsp, -16",
        "call __kui_runtime_init_inner",
        "call main",
        // exit syscall on macOS
        "mov rdi, rax",
        "mov rax, 0x2000001",    // SYS_exit on macOS
        "syscall",
    );

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __kui_runtime_init_inner() {
        // macOS init
    }
}

// Keep the no_std panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
