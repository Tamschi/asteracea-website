use crate::pages::index::Index;
use asteracea::components::Router;

asteracea::component! {
	pub Site()(
		path: &'bump str,
		http_status: &mut u16,
	) -> Sync

	//TODO: For better performance measures with multiple pages, some construction time routing may be good.
	<*Router .path={path}
		->path={"/"} <*Index>

		->path={"/*"} with {
			*http_status = 404;
		} "No content found for this URL."
	>
}
