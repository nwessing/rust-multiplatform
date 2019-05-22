extern crate core;
mod windows_imports;

use std::io::Error;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::time::Instant;
use std::os::raw::c_void;

use windows_imports::*;
use core::pong::{ImageBuffer, Pong, PongInput, VerticalOrigin};

fn main() {
    let bitmap_width = 800;
    let bitmap_height = 800;

    let bitmap_info = create_bitmap_info(bitmap_width, bitmap_height);
    let mut image_buffer = ImageBuffer::new(bitmap_width, bitmap_height, VerticalOrigin::Bottom, 2, 1, 0);

    let window = create_window(bitmap_width, bitmap_height).unwrap();

    let mut game = Pong::new();

    let mut is_running = true;
    let mut w_down = false;
    let mut s_down = false;
    let mut last_update = Instant::now();
    while is_running {
        unsafe {
            let mut message = mem::uninitialized::<MSG>();
            while PeekMessageW(&mut message, 0 as HWND, 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&message);
                DispatchMessageW(&message);

                match message.message {
                    WM_KEYDOWN =>
                        if message.wParam == 'W' as u64 {
                            w_down = true;
                        } else if message.wParam == 'S' as u64 {
                            s_down = true;
                        },
                    WM_KEYUP =>
                        if message.wParam == 'W' as u64 {
                            w_down = false;
                        } else if message.wParam == 'S' as u64 {
                            s_down = false;
                        },
                    _ => {}
                }

                if message.message == WM_QUIT || message.message == WM_CLOSE || message.message == WM_DESTROY {
                    is_running = false;
                }
            }
        }

        let input = if w_down {
            Some(PongInput::Up)
        } else if s_down {
            Some(PongInput::Down)
        } else {
            None
        };

        let duration = last_update.elapsed();
        let elapsed = duration.as_secs() as f32 + (duration.subsec_millis() as f32 / 1000.0);
        last_update = Instant::now();
        game.update(input, elapsed as f32, &mut image_buffer);

        unsafe {
            let device_context = GetDC(window);

            let mut client_rect = mem::uninitialized::<RECT>();
            GetClientRect(window, &mut client_rect);

            let window_width = client_rect.right - client_rect.left;
            let window_height = client_rect.bottom - client_rect.top;
            StretchDIBits(device_context,
                0, 0, window_width, window_height,
                0, 0, bitmap_width, bitmap_height,
                image_buffer.data.as_ptr() as *const c_void,
                &bitmap_info,
                DIB_RGB_COLORS,
                SRCCOPY);

            ReleaseDC(window, device_context);
        }
    }
}

fn create_window(window_width: i32, window_height: i32) -> Result<HWND, Error> {
    let instance: HMODULE;
    unsafe {
        instance = GetModuleHandleW(null_mut());
    }

    let window_class = WNDCLASSW {
        style: 0,
        lpfnWndProc: main_window_event_callback,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
        lpszClassName: win32_string("RuminatingWindowClass").as_ptr()
    };

    unsafe {
        if RegisterClassW(&window_class) == 0 {
            println!("Could not register windows class.");
            return Err(Error::last_os_error());
        }
    }

    let handle: HWND;
    unsafe {
       handle = CreateWindowExW(
            0,
            window_class.lpszClassName,
            win32_string("Ruminating Rust").as_ptr(),
            WS_OVERLAPPEDWINDOW|WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window_width,
            window_height,
            null_mut(),
            null_mut(),
            instance,
            null_mut()
        );

        if handle.is_null() {
            println!("Handle is null.");
            return Err(Error::last_os_error());
        }
    }

    Ok(handle)
}

fn create_bitmap_info(width: i32, height: i32) -> BITMAPINFO {
    BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biClrImportant: 0,
            biClrUsed: 0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0
        },
        bmiColors: [
            RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }
        ]
    }
}

fn win32_string(value : &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

unsafe extern "C" fn main_window_event_callback(window: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match message {
        WM_CLOSE => {
            PostQuitMessage(0);
            0
        },
        _ => {
            DefWindowProcW(window, message, w_param, l_param)
        }
    }
}