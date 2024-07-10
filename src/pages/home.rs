

use dominator::{Dom,  html};

pub struct Home {}

impl Home {
    pub fn render() -> Dom {
        html!("div", {
            .text("Home Page").class("")

        })
    }
}

