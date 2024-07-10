use std::collections::HashMap;
use std::sync::Arc;

use dominator::{Dom, events, html};
use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::prelude::*;
use web_sys::window;


mod pages;

use pages::tomorrow::Tomorrow;
use pages::home::Home;


#[wasm_bindgen(start)]
pub fn main_js() {
    dominator::append_dom(&dominator::body(), App::new().render());
}

struct App {
    route: Arc<Mutable<String>>,
    router: Router,
    navbar: Navbar,
}

impl App {
    fn new() -> Self {
        let route = Arc::new(Mutable::new(get_route()));
        let route_clone = Arc::clone(&route);

        let window = window().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            let new_route = get_route();
            route_clone.set(new_route);
        }) as Box<dyn FnMut()>);

        window.set_onhashchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget(); // Prevent the closure from being dropped

        Self {
            route: Arc::clone(&route),
            router: Router::new(),
            navbar: Navbar::new(Arc::clone(&route)),
        }
    }

    fn render(self) -> Dom {
        html!("div", {
            .child(self.navbar.render())
            .child_signal(self.route.signal_cloned().map(move |route| {
                Some(self.router.render(&route))
            }))
        })
    }
}

struct Router {
    components: HashMap<String, Box<dyn Fn() -> Dom>>,
}

impl Router {
    fn new() -> Self {
        let comp = create_components_map();
        Self { components: comp }
    }

    fn render(&self, route: &String) -> Dom {
        match self.components.get(route) {
            Some(comp) => comp(),
            None => not_found(),
        }
    }
}

struct Navbar {
    route: Arc<Mutable<String>>,
}

impl Navbar {
    fn new(route: Arc<Mutable<String>>) -> Self {
        Self { route }
    }

    fn render(self) -> Dom {
        html!("nav", {
            .attr("class", "ml-10 flex items-baseline space-x-4")
            .children_signal_vec(self.route.signal_cloned().map(move |current_route| {
                vec![
                     Self::link("Today", "/", current_route.clone()),
                    Self::link("Tomorrow", "/tomorrow", current_route.clone()),
                ]
            }).to_signal_vec())
        })
    }

    fn link(title: &str, path: &str, current_route: String) -> Dom {
        let active_class = if current_route == path {
            "rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white"
        } else {
            "rounded-md px-3 py-2 text-sm font-medium text-blue transition delay-80 hover:bg-gray-700 hover:text-white"
        };

        let path_clone = path.to_string();

        html!("a", {
            .attr("href", &format!("#{}", path))
            .attr("class", active_class)
            .text(title)
            .event(move |_: events::Click| {
                let window = window().unwrap();
                window.location().set_hash(&path_clone).unwrap();
            })
        })
    }
}


fn create_components_map() -> HashMap<String, Box<dyn Fn() -> Dom>> {
    let mut components = HashMap::new();
    components.insert(
        "/".to_string(),
        Box::new(Home::render) as Box<dyn Fn() -> Dom>,
    );
    components.insert(
        "/tomorrow".to_string(),
        Box::new(Tomorrow::render) as Box<dyn Fn() -> Dom>,
    );
    components.insert("".to_string(), Box::new(not_found) as Box<dyn Fn() -> Dom>);
    components
}

fn not_found() -> Dom {
    html!("div", {
        .text("Page Not Found")
    })
}

fn get_route() -> String {
    let hash = window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_else(|| "".to_string())
        .trim_start_matches("#")
        .to_string();
    if hash.is_empty() {
        "/".to_string()
    } else {
        hash
    }
}
