



pub fn render_react(component_name: &str) -> Vec<u8>{
    let mut handlebars = handlebars::Handlebars::new();
    let template = r#"
import React from "react";

export const {{component}} = () => {

}
"#;

    handlebars.register_template_string("react", template).unwrap();

    let mut data = std::collections::HashMap::new();
    data.insert("component", component_name);

    handlebars.render("react", &data).unwrap().as_bytes().to_owned()
}