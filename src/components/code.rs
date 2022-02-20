use asteracea::lignin::Node;

macro_rules! code_tags {
	($(
		$(#[$($attributes:tt)*])*
		$ident:ident => $class:literal
	),*$(,)?) => {$(
		asteracea::component! {
			$(#[$($attributes)*])*
			pub $ident()(..) -> Sync

			<span .class=$class ..>
		}
	)*};
}

code_tags! {
	/// Field.
	Fi => "fi",
	/// Function.
	Fn => "fn",
	/// Keyword.
	K => "k",
	/// Literal.
	L => "l",
	/// Macro.
	Ma => "ma",
	/// Module.
	Mo => "mo",
	/// Punctuation.
	P => "p",
	/// Type.
	T => "t",
	/// Variable.
	V => "v",
}

asteracea::component! {
	pub Details()(
		symbol: &'bump str,
		..,
	) -> Sync

	<details
		<summary
			{ Node::Text {
				dom_binding: None,
				text:symbol
			} }
		>
		..
	>
}

macro_rules! hidden {
	($(
		$(#[$($attributes:tt)*])*
		$ident:ident => $symbol:literal
	),*$(,)?) => {$(
		asteracea::component! {
			$(#[$($attributes)*])*
			pub $ident()(..) -> Sync

			<*Details .symbol={$symbol} ..>
		}
	)*};
}

hidden! {
	HiddenDown => "↧",
	HiddenMiddle => "↕",
	HiddenUp => "↥",
}
