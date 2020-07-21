use orbtk::prelude::*;
use deno_core;
use calcite;

extern crate dces;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Comp<'a> {
	_type: &'a str,
	_text:  &'a str
}

widget!(
	Element {
		text: String16
	}
);

impl Template for Element {
	fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
		self.name("element")
	}
}

fn createComp(child: Comp, ctx: &mut BuildContext) -> dces::entity::Entity {
	Element::new().text(child._text).build(ctx)
}

#[calcite::deno_op]
fn createApp(name: String, root: Comp<'static>) {
	Application::new()
	.window(move |ctx| {
		Window::new()
		.title(String::from(&name))
		.position((100.0, 100.0))
		.size(400.0, 400.0)
		.child(createComp(root, ctx))
		.build(ctx)
	})
	.run();
}

calcite::export!(createApp);