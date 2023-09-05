use neon::prelude::*;
use bem::{ BEMBlock, BEMElement };

pub trait ToJsObject {
	fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject>;
}

impl ToJsObject for BEMBlock {
	fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
		let obj = cx.empty_object();

		let name = cx.string(&self.name);
		obj.set(cx, "name", name)?;

		let modifiers = cx.empty_array();
		for (i, modifier) in self.modifiers.iter().enumerate() {
			let js_string = cx.string(modifier);
			modifiers.set(cx, i as u32, js_string)?;
		}
		obj.set(cx, "modifiers", modifiers)?;

		let elements = cx.empty_array();
		for (i, element) in self.elements.iter().enumerate() {
			let js_obj = element.to_object(cx)?;
			elements.set(cx, i as u32, js_obj)?;
		}
		obj.set(cx, "elements", elements)?;

		Ok(obj)
	}
}

impl ToJsObject for BEMElement {
	fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
		let obj = cx.empty_object();

		let name = cx.string(&self.name);
		obj.set(cx, "name", name)?;

		let modifiers = cx.empty_array();
		for (i, modifier) in self.modifiers.iter().enumerate() {
			let js_string = cx.string(modifier);
			modifiers.set(cx, i as u32, js_string)?;
		}
		obj.set(cx, "modifiers", modifiers)?;

		Ok(obj)
	}
}

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
