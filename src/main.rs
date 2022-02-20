use asteracea::bumpalo::Bump;
use asteracea_website::site::Site;
use lignin_html::render_document;
use rhizome::Node;
use std::thread;
use std::time::Instant;
use std::{pin::Pin, sync::Arc};
use tiny_http::{Response, Server};

fn main() {
	let server = Arc::new(Server::http("127.0.0.1:3000").unwrap());

	let mut handles = Vec::new();

	for _ in 0..num_cpus::get() * 2 - 2 {
		let server = Arc::clone(&server);
		handles.push(thread::spawn(move || {
			let root_node = Node::new_for::<()>().into_arc();
			let mut bump = Bump::new();
			let mut body = String::new();
			for rq in server.incoming_requests() {
				let generation_time = Instant::now();

				// Instantiate page.
				let construction_time = Instant::now();
				let site = Site::new(&root_node, Site::new_args_builder().build())
					.expect("Error instantiating site.");
				let construction_time = Instant::now() - construction_time;
				let site = unsafe { Pin::new_unchecked(&site) };

				//TODO: Async-evaluate.
				let async_time = Instant::now();
				let async_time = Instant::now() - async_time;

				let mut status_code: u16 = 200;

				// Render VDOM.
				let render_time = Instant::now();
				let vdom = site
					.render(
						&bump,
						Site::render_args_builder()
							.path(rq.url())
							.http_status(&mut status_code)
							.build(),
					)
					.expect("Error rendering site.");
				let render_time = Instant::now() - render_time;

				// Render HTML.
				let html_time = Instant::now();
				render_document(&vdom, &mut body, 1000).unwrap();
				let html_time = Instant::now() - html_time;
				bump.reset();

				let generation_time = Instant::now() - generation_time;

				let body_with_timings = body
					.replace("%construction_time%", &format!("{:?}", construction_time))
					.replace("%async_time%", &format!("{:?}", async_time))
					.replace("%render_time%", &format!("{:?}", render_time))
					.replace("%html_time%", &format!("{:?}", html_time))
					.replace("%generation_time%", &format!("{:?}", generation_time));
				body.clear();

				// Queue up response.
				let response = Response::from_data(body_with_timings).with_status_code(status_code);
				rq.respond(response).unwrap()
			}
		}))
	}

	for h in handles {
		h.join().unwrap()
	}
}
