use dominator::{Dom, html};

pub struct Tomorrow {}

impl Tomorrow {
    pub fn render() -> Dom {
        html!("div" ,{
            .text("Tomorrow Page")
        })
    }
}
