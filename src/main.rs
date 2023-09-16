#![allow(non_snake_case)]
use std::{env, mem, io::*, time::Duration};
use image::{DynamicImage, RgbaImage, codecs::png::PngEncoder, imageops::FilterType};
use windows::{
	core::*, Win32::{Foundation::*, Graphics::Gdi::*, Storage::Xps::*, UI::WindowsAndMessaging::*}
};

const TITLE: PCWSTR = w!("명일방주");
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const POLL: i32 = 1000 / 250;

fn main() {
	let args: Vec<String> = env::args().collect();
	let command = args.join(" ");

	if command.contains("connect") {
		println!("connected to Google Play Games");
	} else if command.contains("input tap") {
		let x = args[6].parse::<i32>().unwrap();
		let y = args[7].parse::<i32>().unwrap();

		input_tap(x, y);
	} else if command.contains("input swipe") {
		let x1 = args[6].parse::<i32>().unwrap();
		let y1 = args[7].parse::<i32>().unwrap();
		let x2 = args[8].parse::<i32>().unwrap();
		let y2 = args[9].parse::<i32>().unwrap();
		let dur = args[10].parse::<i32>().unwrap();

		input_swipe(x1, y1, x2, y2, dur);
	} else if command.contains("input keyevent 111") {
		input_keyevent(0x01);
	} else if command.contains("dumpsys window displays") {
		println!("{}", WIDTH as i32);
		println!("{}", HEIGHT as i32);
	} else if command.contains("exec-out screencap -p") {
		let image = capture();

		let mut stdout = stdout().lock();
		image.write_with_encoder(PngEncoder::new(&mut stdout)).unwrap();
	} else if command.contains("am force-stop") {
		terminate();
	}
}

fn get_gpg_info() -> (HWND, i32, i32) {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), TITLE) };

	let mut client_rect = RECT::default();
	let _ = unsafe { GetClientRect(hwnd, &mut client_rect) };

	let width = client_rect.right - client_rect.left;
	let height = client_rect.bottom - client_rect.top;

	(hwnd, width, height)
}

fn get_relative_point(x: i32, y: i32, w: i32, h: i32) -> isize {
	let nx = (x as f32 / WIDTH * w as f32) as isize;
	let ny = (y as f32 / HEIGHT * h as f32) as isize;

	ny << 16 | nx
}

fn input_tap(x: i32, y: i32) {
	let (hwnd, w, h) = get_gpg_info();
	let pos = get_relative_point(x, y, w, h);

	unsafe {
		let _ = SendMessageA(hwnd, WM_LBUTTONDOWN, WPARAM(1), LPARAM(pos));
		let _ = SendMessageA(hwnd, WM_LBUTTONUP, WPARAM(1), LPARAM(pos));
	}
}

fn input_swipe(x1: i32, y1: i32, x2: i32, y2: i32, dur: i32) {
	let (hwnd, w, h) = get_gpg_info();

	let time = dur as f32 / POLL as f32;
	let ends = time.floor() as i32;

	let dx = ((x2 - x1) as f32) / time;
	let dy = ((y2 - y1) as f32) / time;

	unsafe {
		let mut cnt = 0;
		loop {
			if cnt >= ends {
				break;
			}

			let nx = x1 + (dx * cnt as f32) as i32;
			let ny = y1 + (dy * cnt as f32) as i32;
			let pos = get_relative_point(nx, ny, w, h);

			let _ = SendMessageA(hwnd, WM_LBUTTONDOWN, WPARAM(1), LPARAM(pos));
			
			spin_sleep::sleep(Duration::new(0, POLL as u32 * 1000000));
			cnt += 1;
		}

		let pos = get_relative_point(x2, y2, w, h);
		let _ = SendMessageA(hwnd, WM_LBUTTONUP, WPARAM(1), LPARAM(pos));
	}
}

fn input_keyevent(keycode: i32) {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), TITLE) };

	let wparam = WPARAM(keycode as usize);
	let down = LPARAM((keycode << 16) as isize);
	let up = LPARAM((keycode << 16 | 1 << 30 | 1 << 31) as isize);

	unsafe {
		let _ = SendMessageA(hwnd, WM_KEYDOWN, wparam, down);
		let _ = SendMessageA(hwnd, WM_KEYUP, wparam, up);
	}
}

fn capture() -> DynamicImage {
	let main = unsafe { FindWindowW(PCWSTR::null(), TITLE) };
	let hwnd = unsafe { FindWindowExA(main, HWND(0), s!("subWin"), PCSTR::null()) };

	let mut rect = RECT::default();
	let _ = unsafe { GetWindowRect(hwnd, &mut rect) };

	let width = rect.right - rect.left;
	let height = rect.bottom - rect.top;
	
	let mut binfo = BITMAPINFO {
		bmiHeader: BITMAPINFOHEADER {
			biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
			biWidth: width,
			biHeight: height,
			biPlanes: 1,
			biBitCount: 32,
			biCompression: 0,
			biSizeImage: 0,
			biXPelsPerMeter: 0,
			biYPelsPerMeter: 0,
			biClrUsed: 0,
			biClrImportant: 0,
		},
		bmiColors: [RGBQUAD::default(); 1],
	};

	let mut buffer = vec![0u8; (width * height) as usize * 4];
	let mut bitmap = BITMAP::default();
	let bitmap_ptr = <*mut _>::cast(&mut bitmap);

	unsafe {
		let dc = GetDC(main);
		let cdc = CreateCompatibleDC(dc);
		let cbmp = CreateCompatibleBitmap(dc, width, height);

		SelectObject(cdc, cbmp);
		PrintWindow(main, cdc, PW_CLIENTONLY);

		GetDIBits(cdc, cbmp, 0, height as u32, Some(buffer.as_mut_ptr() as *mut _), &mut binfo, DIB_RGB_COLORS);
		GetObjectW(cbmp, mem::size_of::<BITMAP>() as i32, Some(bitmap_ptr));

		DeleteDC(dc);
		DeleteDC(cdc);
		ReleaseDC(main, dc);
	}

	let mut chunks: Vec<Vec<u8>> = buffer.chunks(width as usize * 4).map(|x| x.to_vec()).collect();
	chunks.reverse();

	let rgba = chunks.concat().chunks_exact(4).take((bitmap.bmWidth * bitmap.bmHeight) as usize).flat_map(|bgra| [bgra[2], bgra[1], bgra[0], bgra[3]]).collect();
	let image = RgbaImage::from_vec(bitmap.bmWidth as u32, bitmap.bmHeight as u32, rgba).unwrap();
	let native = image::DynamicImage::ImageRgba8(image);
	
	native.resize(WIDTH as u32, HEIGHT as u32, FilterType::CatmullRom)
}

fn terminate() {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), TITLE) };
	let _ = unsafe { PostMessageA(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0)) };
}