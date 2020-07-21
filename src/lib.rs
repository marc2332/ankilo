use orbtk::prelude::*;
use deno_core;
use calcite;

#[calcite::deno_op]
fn createApp(name: String) {
	Application::new()
	.window(move |ctx| {
		Window::new()
		.title(String::from(&name))
		.position((100.0, 100.0))
		.size(400.0, 400.0)
		.child(TextBlock::new().text("Sample").build(ctx))
		.build(ctx)
	})
	.run();
}

calcite::export!(createApp);