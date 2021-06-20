fn main() {
    #[cfg(target_os = "windows")]
    windows::build! {
        Windows::Win32::System::Threading::THREADINFOCLASS,
        Windows::Win32::System::Threading::THREAD_INFORMATION_CLASS
    };
}
