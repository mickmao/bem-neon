extern crate neon;

mod traits;
// mod util; // If you have utility functions

use traits::ToJsObject;
use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
	cx.export_function("parseBEM", parse_bem)?;
	Ok(())
}

fn parse_bem(mut cx: FunctionContext) -> JsResult<JsObject> {
	let input = cx.argument::<JsString>(0)?.value(&mut cx); // Read the first argument as a string.
	match bem::parse(&input) {
		Ok(bem_block) => { bem_block.to_object(&mut cx) }
		Err(e) => {
			cx.throw_type_error(format!("Failed to parse BEM: {}", e)) // Throw a JS TypeError with your error message.
		}
	}
}
