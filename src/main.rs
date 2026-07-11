use windows::{
    core::PCSTR,
    Win32::Foundation::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*,
};

use crate::ppm::create_ppm;

mod ppm;

// WndProc -> Responsável pelo comportamento (responder a eventos/mensagens) que acontecem na janela
extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                let mut rect = RECT::default();
                let _ = GetClientRect(hwnd, &mut rect);

                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;

                let rect_width = 200;
                let rect_height = 150;
                let rect_left = (width - rect_width) / 2;
                let rect_top = (height - rect_height) / 2;

                let blue_brush = CreateSolidBrush(COLORREF(0x00FF0000));
                let old_brush = SelectObject(hdc, blue_brush);

                let _ = Rectangle(
                    hdc,
                    rect_left,
                    rect_top,
                    rect_left + rect_width,
                    rect_top + rect_height,
                );

                let _ = SelectObject(hdc, old_brush);
                let _ = DeleteObject(blue_brush);

                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_CLOSE => {
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(hwnd, msg, w_param, l_param),
        }
    }
}

fn main() {


    create_ppm(200, 100);

    unsafe {
        let h_instance = GetModuleHandleA(None).unwrap();
        let hb = CreateSolidBrush(COLORREF(0x00000000));
        
        let class_name = "myWindowClass\0";
        let window_name = "My Window\0";
        
        // Converter para PCSTR
        let class_name_pcstr = PCSTR::from_raw(class_name.as_ptr());
        
        let wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: WNDCLASS_STYLES(0),
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance.into(),
            hIcon: LoadIconW(None, IDI_APPLICATION).unwrap(),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hbrBackground: hb,
            lpszMenuName: PCSTR::null(),
            lpszClassName: class_name_pcstr,
            hIconSm: LoadIconW(None, IDI_APPLICATION).unwrap(),
        };

        if RegisterClassExA(&wc) == 0 {
            MessageBoxA(
                None,
                PCSTR::from_raw("Windows Registration Failed!\0".as_ptr()),
                PCSTR::from_raw("Error!\0".as_ptr()),
                MB_ICONINFORMATION | MB_OK,
            );
            return;
        }

        let window_name_pcstr = PCSTR::from_raw(window_name.as_ptr());
        
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE(0),
            class_name_pcstr,
            window_name_pcstr,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            h_instance,
            None,
        );

        match hwnd {
            Ok(hwnd) => {
                let _ = ShowWindow(hwnd, SW_SHOW);
                let _ = UpdateWindow(hwnd);

                let mut msg = MSG::default();
                while GetMessageA(&mut msg, None, 0, 0).as_bool() {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
            }
            Err(_) => {
                MessageBoxA(
                    None,
                    PCSTR::from_raw("Create Window failed!\0".as_ptr()),
                    PCSTR::from_raw("Error!\0".as_ptr()),
                    MB_ICONEXCLAMATION | MB_OK,
                );
            }
        }
    }
}