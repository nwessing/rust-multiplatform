#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::os::raw::c_void;

pub type PVOID = *mut c_void;
pub type HANDLE = PVOID;
pub type HINSTANCE = HANDLE;
pub type HWND = HANDLE;
pub type HDC = HANDLE;
pub type BYTE = u8;
pub type UINT = u32;
pub type WORD = u16;
pub type DWORD = u32;
pub type int = i32;
pub type LONG = i32;
pub type BOOL = i32;
pub type WPARAM = u64;
pub type LPARAM = i64;
pub type LRESULT = i64;
pub type HMODULE = HINSTANCE;
pub type CHAR = u8;
pub type WCHAR = u16;
pub type LPCSTR = *const CHAR;
pub type HICON = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;
pub type HMENU = HANDLE;
pub type LPCWSTR = *const WCHAR;
pub type ATOM = WORD;
pub type LPVOID = *const c_void;
pub type WNDPROC = unsafe extern "C" fn(window: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;

#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}

#[repr(C)]
pub struct POINT {
  pub x: LONG,
  pub y: LONG,
}

#[repr(C)]
pub struct MSG {
  pub hwnd: HWND,
  pub message: UINT,
  pub wParam: WPARAM,
  pub lParam: LPARAM,
  pub time: DWORD,
  pub pt: POINT,
  pub lPrivate: DWORD,
}
pub type LPMSG = *mut MSG;

#[repr(C)]
pub struct BITMAPINFOHEADER {
    pub biSize: DWORD,
    pub biWidth: LONG,
    pub biHeight: LONG,
    pub biPlanes: WORD,
    pub biBitCount: WORD,
    pub biCompression: DWORD,
    pub biSizeImage: DWORD,
    pub biXPelsPerMeter: LONG,
    pub biYPelsPerMeter: LONG,
    pub biClrUsed: DWORD,
    pub biClrImportant: DWORD,
}

#[repr(C)]
pub struct RGBQUAD {
    pub rgbBlue: BYTE,
    pub rgbGreen: BYTE,
    pub rgbRed: BYTE,
    pub rgbReserved: BYTE,
}

#[repr(C)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub type LPRECT = *mut RECT;

pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME
    | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const CW_USEDEFAULT: int = 0x80000000;

pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUIT: UINT = 0x0012;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;

pub const PM_REMOVE: UINT = 0x0001;

pub const BI_RGB: DWORD = 0;

pub const DIB_RGB_COLORS: DWORD = 0;
pub const SRCCOPY: DWORD = 0x00CC0020;

#[link(name = "user32", kind = "static")]
extern {
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn GetModuleHandleW(lpModuleName: LPCSTR) -> HMODULE;
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: int,
        Y: int,
        nWidth: int,
        nHeight: int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID) -> HWND;
    pub fn PostQuitMessage(nExitCode: int) -> c_void;
    pub fn PeekMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    pub fn GetDC(hWnd: HWND) -> HDC;
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> int;
    pub fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
}

#[link(name = "gdi32", kind = "static")]
extern {
    pub fn StretchDIBits(
        hdc: HDC,
        xDest: int,
        yDest: int,
        DestWidth: int,
        DestHeight: int,
        xSrc: int,
        ySrc: int,
        SrcWidth: int,
        SrcHeight: int,
        lpBits: *const c_void,
        lpbmi: *const BITMAPINFO,
        iUsage: UINT,
        rop: DWORD) -> int;
}