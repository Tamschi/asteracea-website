use crate::{
	components::{
		code::{Fi, HiddenMiddle, K, L, P, V},
		snippet::{
			component::{End, Start},
			Snippet,
		},
	},
	templates::generic_page::GenericPage,
};

asteracea::component! {
	pub Index()() -> Sync

	<*GenericPage
		<*Snippet [
			<*Start "AsteriskFor"
				'constructor_params: [
					<*V "s"><*P ":">" "<*P "&"><*K "str">
				]
			>
			"\t" <*P "*"><*K "for">" "<*V "c">" "<*K "in">" "<*V "s">" "<*P "{">"\n"
			<*HiddenMiddle [
				"\t\t" <*K "let">" "<*K "self"><*P "."><*Fi "c"><*P ":">" "<*K "char">" "<*P "=">" "<*V "c"><*P ";">"\n"
				"\t\t" <*P "!"><*L r#""{}""#><*P "("><*K "self"><*P "."><*Fi "c"><*P ")">"\n"
			]>
			"\t" <*P "}">"\n"
			<*End>
		] /Snippet>
	>
}
