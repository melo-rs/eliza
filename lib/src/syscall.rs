#[macro_export]
macro_rules! system_call {
    ($nr:expr) => {{
        let result: isize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") $nr as isize => result,
            lateout("rcx") _,
            lateout("r11") _,
        );

        result
    }};

    ($nr:expr, $a1:expr) => {{
        let result: isize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") $nr as isize => result,
            in("rdi") $a1,
            lateout("rcx") _,
            lateout("r11") _,
        );

        result
    }};

    ($nr:expr, $a1:expr, $a2:expr) => {{
        let result: isize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") $nr as isize => result,
            in("rdi") $a1,
            in("rsi") $a2,
            lateout("rcx") _,
            lateout("r11") _,
        );

        result
    }};

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {{
        let result: isize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") $nr as isize => result,
            in("rdi") $a1,
            in("rsi") $a2,
            in("rdx") $a3,
            lateout("rcx") _,
            lateout("r11") _,
        );

        result
    }};

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{
        let result: isize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") $nr as isize => result,
            in("rdi") $a1,
            in("rsi") $a2,
            in("rdx") $a3,
            in("r10") $a4,
            lateout("rcx") _,
            lateout("r11") _,
        );

        result
    }};

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{
        let result: isize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") $nr as isize => result,
            in("rdi") $a1,
            in("rsi") $a2,
            in("rdx") $a3,
            in("r10") $a4,
            in("r8") $a5,
            lateout("rcx") _,
            lateout("r11") _,
        );

        result
    }};

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let result: isize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") $nr as isize => result,
            in("rdi") $a1,
            in("rsi") $a2,
            in("rdx") $a3,
            in("r10") $a4,
            in("r8") $a5,
            in("r9") $a6,
            lateout("rcx") _,
            lateout("r11") _,
        );

        result
    }};
}
