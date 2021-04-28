// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.
use fltk::{app, prelude::*, window::Window};
use json::{parse, JsonValue};

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
	interface.register_op("create_window", create_window);
}


fn create_window(
	_interface: &mut dyn Interface,
	zero_copy: Option<ZeroCopyBuf>,
) -> Op {
	let fut = async move {

		if let Some(buf) = zero_copy {
			let buf_str = std::str::from_utf8(&buf[..]).unwrap();
			println!("zero_copy: {}", buf_str);
		}

		let app = app::App::default();
		let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
		wind.end();
		wind.show();
		app.run().unwrap();



		let result = b"test";
		let result_box: Box<[u8]> = Box::new(*result);
		(0, OpResponse::Buffer(result_box))
	};

	Op::Async(fut.boxed())
}
