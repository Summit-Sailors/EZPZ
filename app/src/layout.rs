use {crate::router::Route, dioxus::prelude::*, strum::IntoEnumIterator};

#[component]
pub fn Layout(children: Element) -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
		document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
		document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
		document::Link {
			rel: "stylesheet",
			href: "https://fonts.googleapis.com/css2?family=DM+Mono:wght@400;500&family=Poppins:ital,wght@0,400;0,500;0,600;0,700;1,400;1,500;1,600;1,700&display=swap",
		}
		div { class: "grid grid-cols-7 gap-4 max-h-screen h-full",
			Sidebar {}
			div { class: "p-4 rounded col-span-6 h-full min-h-screen", Outlet::<Route> {} }
		}
	}
}

#[component]
pub fn Sidebar() -> Element {
	let navigator = use_navigator();
	let router = router();
	let current_route = router.current::<Route>();

	rsx! {
		div { class: "h-full overflow-hidden py-8",
			nav { class: "gap-y-5 flex flex-col overflow-y-auto px-4 h-full",
				Fragment {
					for route in Route::iter() {
						div {
							key: "{route.to_string()}",
							class: "px-3 py-2 rounded-md hover:bg-gray-800 transition-colors cursor-pointer ease-linear",
							class: if route.to_string() == current_route.to_string() { "bg-gray-800 text-indigo-200" },
							onclick: move |_| {
									navigator.push(route.clone());
							},
							span { {route.name()} }
						}
					}
				}
			}
		}
	}
}
