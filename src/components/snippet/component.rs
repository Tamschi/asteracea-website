use crate::components::code::{HiddenDown, HiddenUp, Ma, Mo, K, P, T};
use asteracea::{include::render_callback::RenderOnce, lignin::ThreadSafe, __::Built};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct NoParentParameters {}
impl Built for NoParentParameters {
	type Builder = NoParentParametersBuilder<()>;

	fn builder() -> Self::Builder {
		Self::builder()
	}
}

asteracea::component! {
	pub Start()(
		..,
		constructor_params?: (NoParentParameters, Box<RenderOnce<'_, 'bump, ThreadSafe>>),
		render_params?: (NoParentParameters, Box<RenderOnce<'_, 'bump, ThreadSafe>>),
	) -> Sync

	<*HiddenUp [
		<*Mo "asteracea"><*P "::"><*Ma "component"><*P "!">" "<*P "{">"\n"
		"\t"<*K "pub">" "<*T ..><*P "(">
		spread if { let Some(constructor_params) = constructor_params}
			{ constructor_params.1(bump)? }
		<*P ")(">
		spread if { let Some(render_params) = render_params}
			{ render_params.1(bump)? }
		<*P ")">"\n\n"
	] /HiddenUp>
}

asteracea::component! {
	pub End()() -> Sync

	<*HiddenDown
		<*P "}">
	>
}
