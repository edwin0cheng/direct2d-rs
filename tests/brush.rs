extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate direct2d;

use std::{ptr, mem};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::*;
use direct2d::{Factory, RenderTarget};
use direct2d::render_target::RenderTargetBacking;
use direct2d::math::*;

#[test]
fn solid_color() {
    let rt = make_rt();
    
    for i in 0u32..(16*16*16) {
        let color = ColorF(D2D1_COLOR_F {
            r: ((i >> 8) & 0xF) as f32 / 15.0,
            g: ((i >> 4) & 0xF) as f32 / 15.0,
            b: ((i >> 0) & 0xF) as f32 / 15.0,
            a: 1.0,
        });
        
        let brush = rt.target.create_solid_color_brush(
            &color, &BrushProperties::default()
        ).unwrap();
        
        let brush_color = brush.get_color();
        
        assert_eq!(color, brush_color);
    }
}

#[allow(dead_code)]
struct RT {
    target: RenderTarget,
    factory: Factory,
    hwnd: HWND,
}

impl Drop for RT {
    fn drop(&mut self) {
        unsafe { user32::DestroyWindow(self.hwnd) };
    }
}

fn make_rt() -> RT {
    unsafe {
        let factory = Factory::create().unwrap();
        
        let hinst: HINSTANCE = kernel32::GetModuleHandleW(ptr::null());
        let class_name = "Test D2D1 Window Class".to_wide_null();
        let window_name = "Test D2D1 Window".to_wide_null();
        
        user32::RegisterClassW(&WNDCLASSW {
            lpfnWndProc: Some(user32::DefWindowProcW),
            hInstance: hinst,
            lpszClassName: class_name.as_ptr(),
            
            .. mem::zeroed()
        });
        
        let hwnd = user32::CreateWindowExW(
            0, // dwExStyle
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPED,
            CW_USEDEFAULT, // x
            CW_USEDEFAULT, // y
            800, // width
            480, // height
            ptr::null_mut(),
            ptr::null_mut(),
            hinst,
            ptr::null_mut(),
        );
        
        assert!(!hwnd.is_null());
        
        let params = WindowCreate {
            props: D2D1_RENDER_TARGET_PROPERTIES {
                _type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_UNKNOWN,
                    alphaMode: D2D1_ALPHA_MODE_UNKNOWN,
                },
                dpiX: 0.0,
                dpiY: 0.0,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            },
            hprops: D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd: hwnd,
                pixelSize: D2D1_SIZE_U {
                    width: 800,
                    height: 480,
                },
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            },
        };
        
        let target = factory.create_render_target(params).unwrap();
        
        RT {
            target: target,
            factory: factory,
            hwnd: hwnd,
        }
    }
}

struct WindowCreate {
    props: D2D1_RENDER_TARGET_PROPERTIES,
    hprops: D2D1_HWND_RENDER_TARGET_PROPERTIES,
}

unsafe impl RenderTargetBacking for WindowCreate {
    fn create_target(self, factory: &mut ID2D1Factory) -> Result<*mut ID2D1RenderTarget, HRESULT> {
        unsafe {
            let mut ptr: *mut ID2D1HwndRenderTarget = ptr::null_mut();
            let hr = factory.CreateHwndRenderTarget(
                &self.props,
                &self.hprops,
                &mut ptr as *mut _,
            );
            
            let ptr: *mut _ = &mut **ptr;
            
            if SUCCEEDED(hr) {
                Ok(ptr)
            } else {
                Err(From::from(hr))
            }
        }
    }
}

pub trait ToWide { 
    fn to_wide(&self) -> Vec<u16>; 
    fn to_wide_null(&self) -> Vec<u16>; 
} 

impl<T> ToWide for T where T: AsRef<OsStr> { 
    fn to_wide(&self) -> Vec<u16> { 
        self.as_ref().encode_wide().collect()
    } 
    fn to_wide_null(&self) -> Vec<u16> { 
        self.as_ref().encode_wide().chain(Some(0)).collect() 
    } 
} 


