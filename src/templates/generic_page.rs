use asteracea::lignin::Node;

asteracea::component! {
	pub GenericPage()(..) -> Sync

	<html .lang="en"
		<head
			<meta .charset="utf-8">
			<meta .name="viewport" .content="width=device-width,initial-scale=1">
			<style {
				Node::Text {
					dom_binding: None,
					text: include_str!("global.css"),
				}
			}>
		/head>
		<body
			..
			<footer
				"Page instantiated in %construction_time%, async-evaluated (TODO) in %async_time%, rendered in %render_time% and HTML written in %html_time%." <br>
				"In total, %generation_time% was spent on page generation. All times measured in real time, so there may be jitter from scheduling."
			/footer>
		/body>
	/html>
}
