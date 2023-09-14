#![allow(non_snake_case)]
use std::{env, io::*, time::Duration};
use image::{codecs::png::PngEncoder, DynamicImage};
use screenshots::Screen;
use windows::{
    core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let command = args.join(" ");

	if command.contains("connect") {
		let mut stdout = stdout().lock();
		stdout.write_all(b"connected to Google Play Games").unwrap();
	}

	if command.contains("shell input tap") {
		let x = args[6].parse::<i32>().unwrap();
		let y = args[7].parse::<i32>().unwrap();

		input_tap(x, y);
	}

	if command.contains("shell input swipe") {
		let x1 = args[6].parse::<i32>().unwrap();
		let y1 = args[7].parse::<i32>().unwrap();
		let x2 = args[8].parse::<i32>().unwrap();
		let y2 = args[9].parse::<i32>().unwrap();
		let dur = args[10].parse::<i32>().unwrap();

		input_swipe(x1, y1, x2, y2, dur);
	}

	if command.contains("shell input keyevent 111") {
		input_keyevent(0x01);
	}

	if command.contains("shell dumpsys window displays") {
		let (x, y) = get_screen_size();

		let mut stdout = stdout().lock();
		stdout.write_all(format!("{x}\n{y}").as_bytes()).unwrap();
	}

	if command.contains("exec-out screencap -p") {
		let image = get_screen_capture();

		let mut stdout = stdout().lock();
		image.write_with_encoder(PngEncoder::new(&mut stdout)).unwrap();
	}
}

fn get_screen_size() -> (i32, i32) {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), w!("명일방주")) };
	let mut rect = RECT::default();
	let _ = unsafe { GetWindowRect(hwnd, &mut rect) };

	(rect.right - rect.left, rect.bottom - rect.top)
}

fn input_tap(x: i32, y: i32) {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), w!("명일방주")) };

	let pos = (y << 16 | x) as isize;

	unsafe {
		let _ = SendMessageA(hwnd, WM_LBUTTONDOWN, WPARAM(1), LPARAM(pos));
		let _ = SendMessageA(hwnd, WM_LBUTTONUP, WPARAM(1), LPARAM(pos));
	}
}

fn input_swipe(x1: i32, y1: i32, x2: i32, y2: i32, dur: i32) {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), w!("명일방주")) };

	let polling = 1000 / 250;
	let times = dur as f32 / polling as f32;

	let dx = ((x2 - x1) as f32) / times;
	let dy = ((y2 - y1) as f32) / times;

	unsafe {
		let mut count = 0f32;
		loop {
			if count >= times {
				break;
			}

			let pos = ((y1 + (dy * count as f32) as i32) << 16 | (x1 + (dx * count as f32) as i32)) as isize;
			let _ = SendMessageA(hwnd, WM_LBUTTONDOWN, WPARAM(1), LPARAM(pos));
			
			spin_sleep::sleep(Duration::new(0, polling * 1000000));
			count += 1.0;
		}

		let pos = (y2 << 16 | x2) as isize;
		let _ = SendMessageA(hwnd, WM_LBUTTONUP, WPARAM(1), LPARAM(pos));
	}
}

fn input_keyevent(keycode: i32) {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), w!("명일방주")) };

	let wparam = WPARAM(keycode as usize);
	let down = LPARAM((keycode << 16) as isize);
	let up = LPARAM((keycode << 16 | 1 << 30 | 1 << 31) as isize);

	unsafe {
		let _ = SendMessageA(hwnd, WM_KEYDOWN, wparam, down);
		let _ = SendMessageA(hwnd, WM_KEYUP, wparam, up);
	}
}

fn get_screen_capture() -> DynamicImage {
	let hwnd = unsafe { FindWindowW(PCWSTR::null(), w!("명일방주")) };
	
	let mut rect = RECT::default();
	let _ = unsafe { GetWindowRect(hwnd, &mut rect) };
	
	let screen = Screen::from_point((rect.left + rect.right) / 2, (rect.top + rect.bottom) / 2).unwrap();
	let capture = screen.capture().unwrap();

	image::DynamicImage::ImageRgba8(capture)
}