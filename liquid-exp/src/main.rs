use coutils::read_file;
use liquid::ObjectView;
use liquid::ValueView;
use liquid::ParserBuilder;
use std::fmt::Debug;
use liquid::Template;
use liquid::object;
use coutils::clean_split;
use coutils::create_file;
use coutils::write_to_file;
use std::collections::HashMap;
use liquid::partials::EagerCompiler;
use liquid::partials::InMemorySource;

#[derive(ObjectView, ValueView, Clone, Debug)]
pub struct SiteContext {
    title: String,
    content: String,
}

impl SiteContext {
    pub fn new(title: &str, content: &str) -> SiteContext {
        return SiteContext{
            title: title.to_owned(),
            content: content.to_owned(),
        }
    }
}

pub fn render_template(template_path: &String, ctx: &SiteContext) -> Result<(), String> {
    let template_string: String = read_file(template_path);
    type Partials =  EagerCompiler<InMemorySource>;
    let mut new_ctx: HashMap<String, SiteContext> = HashMap::new();
    let src: &String = &String::from("header.liquid");
    let src_contents: &String = &read_file(src);
    let mut partial_source = Partials::empty();
    partial_source.add(src, src_contents);
    new_ctx.insert(String::from("site"), ctx.to_owned());
    let mut template: Template = match ParserBuilder::with_stdlib().partials(partial_source)
        .build().unwrap()
        .parse(&template_string){
            Ok(template) => template,
            Err(e) =>  {
                return Err(e.to_string());
            }
        };
    let mut globals = object!(new_ctx);
    let mut html_string = match template.render(&globals) {
        Ok(html_string) => html_string,
        Err(e) =>  {
            return Err(e.to_string());
        }
    };
    let path_items: Vec<String> = clean_split(template_path, &String::from("/"));
    let last_index: usize = path_items.len() -1 ;
    let fname_components: Vec<String> = clean_split(&path_items[last_index], &String::from("."));
    let base: &String = &fname_components[0];
    let new_name: &String = &format!("{}.html", base);
    create_file(new_name);
    write_to_file(new_name, &html_string);
    return Ok(());
}

fn main(){
    let ctx: SiteContext = SiteContext::new(
        &"TEST",
        &"TEXT TEXT TEXT TEXT TEXT TEXT\nTEXT TEXT TEXT TEXT TEXT TEXT"
    );
    let template_path = &String::from("test.liquid");
    match render_template(template_path, &ctx) {
        Ok(_x) => {},
        Err(e) => {
            println!("{}", e);
        }
    }
}