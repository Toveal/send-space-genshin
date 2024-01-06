mod winapi;

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use lazy_static::lazy_static;
use windows::{
    core::{w, HRESULT},
    Win32::{
        Foundation::{ERROR_INVALID_WINDOW_HANDLE, HWND, LPARAM, LRESULT, WPARAM},
        UI::{
            Input::KeyboardAndMouse::{VK_OEM_3, VK_SPACE},
            WindowsAndMessaging::{
                CallNextHookEx, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, LLKHF_UP, MSG,
            },
        },
    },
};

static mut KEY_STATE: [bool; 1] = [false];

lazy_static! {
    static ref POLLING_SPACE: Mutex<bool> = Mutex::new(false);
}

fn main() {
    let option_hwnd = winapi::find_window(w!("Genshin Impact"), w!("UnityWndClass"));
    let hwnd = match option_hwnd {
        None => {
            winapi::message_box(w!("Genshin not found"));
            std::process::exit(0)
        }
        Some(value) => value,
    };
    let hwnd_arc: Arc<HWND> = Arc::new(hwnd);

    let hook = winapi::set_global_keyboard_hook(Some(keyboard_proc))
        .expect("Hook keyboard installation error {msg}");

    let hook_arc: Arc<HHOOK> = Arc::new(hook);
    let hook_arc_clone = hook_arc.clone();
    ctrlc::set_handler(move || {
        let _ = winapi::unhook_windows_hook(*hook_arc);
        std::process::exit(0);
    })
    .expect("Hook ctrl-c installation error {msg}");

    std::thread::spawn(move || {
        let invalid_handle = HRESULT(ERROR_INVALID_WINDOW_HANDLE.0 as i32);
        let process_error = |hresult: HRESULT| {
            if hresult.0 & 0x0000FFFF == invalid_handle.0 {
                winapi::message_box(w!("Genshin not found"));
                let _ = winapi::unhook_windows_hook(*hook_arc_clone);
                std::process::exit(0);
            }
        };

        loop {
            if !*POLLING_SPACE.lock().unwrap() {
                sleep(Duration::from_millis(200));
                continue;
            }

            if let Err(e) = winapi::send_keystroke(*hwnd_arc, VK_SPACE, true) {
                process_error(e.code());
            }
            sleep(Duration::from_millis(70));

            if let Err(e) = winapi::send_keystroke(*hwnd_arc, VK_SPACE, false) {
                process_error(e.code());
            }
        }
    });

    let mut msg = MSG::default();
    while winapi::get_message(&mut msg) {
        winapi::translate_message(&msg);
        winapi::dispatch_message(&msg);
    }
}

extern "system" fn keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code as u32 == HC_ACTION {
        let kbd_struct = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
        let vk_code = kbd_struct.vkCode as u16;
        let key_pressed = kbd_struct.flags != LLKHF_UP;

        match vk_code {
            code if code == VK_OEM_3.0 => process_key_comb(0, key_pressed),
            _ => {}
        };
    }
    unsafe { CallNextHookEx(HHOOK::default(), code, w_param, l_param) }
}

fn process_key_comb(index: usize, b_pressed: bool) {
    unsafe {
        if KEY_STATE[index] != b_pressed {
            KEY_STATE[index] = b_pressed;
        }
    }
    unsafe {
        if KEY_STATE.iter().all(|it| it == &true) {
            let mut pool_space = POLLING_SPACE.lock().unwrap();
            *pool_space = !*pool_space;
        }
    }
}
