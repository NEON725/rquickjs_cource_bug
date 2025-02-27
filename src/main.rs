use rquickjs::{async_with, AsyncContext, AsyncRuntime, Value};

#[tokio::main(flavor = "current_thread")]
async fn main()
{
	let runtime = AsyncRuntime::new().unwrap();
	let context = AsyncContext::builder()
		.build_async(&runtime)
        .await.unwrap();
	async_with!(context=> |context|
	{
		let globals = context.globals();
        globals.set("test", Value::from_string(rquickjs::String::from_str(context, "HELLO, WORLD").unwrap()))
    }).await.unwrap();
}
