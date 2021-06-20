fn main() {
    #[cfg(target_os = "windows")]
    windows::build! {
        Windows::Win32::System::Threading::GetCurrentThreadId
    };
}
