pub fn render_node(component_name: &str) -> Vec<u8>{
    let mut handlebars = handlebars::Handlebars::new();
    let template = r#"
const {{component}} = () => {

}
"#;

    handlebars.register_template_string("node", template).unwrap();

    let mut data = std::collections::HashMap::new();
    data.insert("component", component_name);

    handlebars.render("node", &data).unwrap().as_bytes().to_owned()
}