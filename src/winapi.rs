use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, WPARAM},
        UI::{
            Input::KeyboardAndMouse::VIRTUAL_KEY,
            WindowsAndMessaging::{
                SendMessageW, HHOOK, HOOKPROC, MB_ICONINFORMATION, MSG, WH_KEYBOARD_LL, WM_KEYDOWN,
            },
        },
    },
};

pub fn get_message(msg: &mut MSG) -> bool {
    unsafe { windows::Win32::UI::WindowsAndMessaging::GetMessageW(msg, HWND::default(), 0, 0) }
        .as_bool()
}

pub fn translate_message(msg: &MSG) -> bool {
    unsafe { windows::Win32::UI::WindowsAndMessaging::TranslateMessage(msg) }.as_bool()
}

pub fn dispatch_message(msg: &MSG) {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(msg);
    }
}

pub fn find_window(window_name: PCWSTR, class_name: PCWSTR) -> Option<HWND> {
    unsafe {
        let hwnd = windows::Win32::UI::WindowsAndMessaging::FindWindowW(class_name, window_name);

        match hwnd.0 {
            0 => None,
            _ => Some(hwnd),
        }
    }
}

pub fn message_box(text: PCWSTR) {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
            HWND::default(),
            text,
            PCWSTR::null(),
            MB_ICONINFORMATION,
        );
    }
}

pub fn send_keystroke(
    hwnd: HWND,
    key: VIRTUAL_KEY,
    pressed: bool,
) -> std::result::Result<(), windows::core::Error> {
    unsafe {
        let mut l_param = 0isize;

        if !pressed {
            l_param |= 0x8000_0000;
        }

        l_param |= (key.0 as isize) << 16;

        let _ = SendMessageW(hwnd, WM_KEYDOWN, WPARAM(key.0 as usize), LPARAM(l_param));
        GetLastError()
    }
}

pub fn set_global_keyboard_hook(hook: HOOKPROC) -> Result<HHOOK, windows::core::Error> {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SetWindowsHookExW(
            WH_KEYBOARD_LL,
            hook,
            HINSTANCE::default(),
            0,
        )
    }
}

pub fn unhook_windows_hook(hook: HHOOK) -> Result<(), windows::core::Error> {
    unsafe { windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx(hook) }
}
