//! Windowless mode example (for Sciter.Lite build).
extern crate sciter;
extern crate winit;

fn main() {
	if !cfg!(feature = "windowless") {
		panic!("This example requires \"windowless\" feature!");
	}

	if !cfg!(feature = "dynamic") {
		panic!("This example requires the \"dynamic\" feature enabled.")
	}

	if let Some(arg) = std::env::args().nth(1) {
		println!("loading sciter from {:?}", arg);
		if let Err(_) = sciter::set_options(sciter::RuntimeOptions::LibraryPath(&arg)) {
			panic!("Invalid sciter-lite dll specified.");
		}
	}


	let mut events = winit::EventsLoop::new();

	println!("create window");

	let wnd = winit::WindowBuilder::new();
	let wnd = wnd.build(&events);

	println!("create sciter instance");
	use sciter::windowless::{Message, handle_message};
	let hwnd = { &wnd as *const _ as sciter::types::HWINDOW };
	handle_message(hwnd, Message::Create { backend: sciter::types::GFX_LAYER::SKIA_OPENGL, transparent: false, });

	// let html = include_bytes!("minimal.htm");
	// let instance = sciter::Host::attach(hwnd);
	// instance.load_html(html, Some("example://minimal.htm"));

	use sciter::windowless::{MouseEvent, KeyboardEvent};
	use sciter::windowless::{MOUSE_BUTTONS, MOUSE_EVENTS, KEYBOARD_STATES, KEY_EVENTS};

	let mut mouse_button = MOUSE_BUTTONS::NONE;
	let mut mouse_pos = (0, 0);

	let as_keys = |modifiers: winit::ModifiersState| {
		let mut keys = 0;
		if modifiers.ctrl {
			keys |= 0x01;
		}
		if modifiers.shift {
			keys |= 0x02;
		}
		if modifiers.alt {
			keys |= 0x04;
		}
		KEYBOARD_STATES::from(keys)
	};

	println!("running...");
	use winit::{Event, WindowEvent};
	let skip = winit::ControlFlow::Continue;
	events.run_forever(|event: winit::Event| {
		match event {
			Event::WindowEvent { event, window_id: _ } => {
				match event {
					WindowEvent::Destroyed => {
						// never called due to loop break on close
						println!("destroy");
						handle_message(hwnd, Message::Destroy);
						winit::ControlFlow::Break
					},

					WindowEvent::CloseRequested => {
						println!("close");
						winit::ControlFlow::Break
					},

					WindowEvent::Resized(size) => {
						// println!("{:?}, size: {:?}", event, size);
						let (width, height): (u32, u32) = size.into();
						handle_message(hwnd, Message::Size { width, height });
						skip
					},

					WindowEvent::Refresh => {
						// println!("{:?}", event);
						handle_message(hwnd, Message::Redraw);
						skip
					},

					WindowEvent::Focused(enter) => {
						handle_message(hwnd, Message::Focus { enter });
						skip
					},

					WindowEvent::CursorEntered { device_id: _ } => {
						println!("mouse enter");
						let event = MouseEvent {
							event: MOUSE_EVENTS::MOUSE_ENTER,
							button: mouse_button,
							modifiers: KEYBOARD_STATES::from(0),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(hwnd, Message::Mouse(event));
						skip
					},

					WindowEvent::CursorLeft { device_id: _ } => {
						println!("mouse leave");
						let event = MouseEvent {
							event: MOUSE_EVENTS::MOUSE_LEAVE,
							button: mouse_button,
							modifiers: KEYBOARD_STATES::from(0),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(hwnd, Message::Mouse(event));
						skip
					},

					WindowEvent::CursorMoved { device_id: _, position, modifiers } => {
						mouse_pos = position.into();

						let event = MouseEvent {
							event: MOUSE_EVENTS::MOUSE_MOVE,
							button: mouse_button,
							modifiers: as_keys(modifiers),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(hwnd, Message::Mouse(event));
						skip
					},

					WindowEvent::MouseInput { device_id: _, state, button, modifiers } => {
						mouse_button = match button {
							winit::MouseButton::Left => MOUSE_BUTTONS::MAIN,
							winit::MouseButton::Right => MOUSE_BUTTONS::PROP,
							winit::MouseButton::Middle => MOUSE_BUTTONS::MIDDLE,
							_ => MOUSE_BUTTONS::NONE,
						};
						println!("mouse {:?} as {:?}", mouse_button, mouse_pos);

						let event = MouseEvent {
							event: if state == winit::ElementState::Pressed { MOUSE_EVENTS::MOUSE_DOWN } else { MOUSE_EVENTS::MOUSE_UP },
							button: mouse_button,
							modifiers: as_keys(modifiers),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(hwnd, Message::Mouse(event));
						skip
					},

					WindowEvent::KeyboardInput { device_id: _, input } => {
						println!("key {} {}", input.scancode, if input.state == winit::ElementState::Pressed { "down" } else { "up" });

						let event = KeyboardEvent {
							event: if input.state == winit::ElementState::Pressed { KEY_EVENTS::KEY_DOWN } else { KEY_EVENTS::KEY_UP },
							code: input.scancode,
							modifiers: as_keys(input.modifiers),
						};

						handle_message(hwnd, Message::Keyboard(event));
						skip
					},

					_	=> winit::ControlFlow::Continue,
				}
			},

			_ => winit::ControlFlow::Continue,
		}
	});

	println!("done, quit");
}
