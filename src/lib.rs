// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.
use orbtk::prelude::*;
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

		Application::new()
			.window(move |ctx| {
				Window::new()
					.title(String::from("test"))
					.position((100.0, 100.0))
					.size(400.0, 400.0)
					.child(TextBlock::new().text("Sample").build(ctx))
					.build(ctx)
			})
			.run();

		let result = b"test";
		let result_box: Box<[u8]> = Box::new(*result);
		(0, OpResponse::Buffer(result_box))
	};

	Op::Async(fut.boxed())
}
