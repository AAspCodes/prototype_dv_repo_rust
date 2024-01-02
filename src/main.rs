use std::{fs::{File, self}, collections::HashMap, path::Path};

use markdown_gen::markdown::{Markdown, AsMarkdown, List};

use crate::data_model::Table;

mod data_model;


fn main() {
    println!("Hello, world!");
    let mut data = Table{
        name: String::from("Aluminum"),
        rows: HashMap::new(), 
    };

    let mut row = HashMap::<String,String>::new();
    row.insert(String::from("Alloy"), String::from("ti-183819"));
    row.insert(String::from("DFR"), String::from("32"));
    row.insert(String::from("shot-peened"), String::from("true"));



    data.rows.insert(String::from("ti-183819"), row );
    let datum = vec![data,];
    write_yaml(datum)
    // let res = read_yaml();
    // println!("{:?}", res);
}

fn write_yaml(data: Vec<Table>) {
    let s = serde_yaml::to_string(&data).unwrap();
    fs::write(Path::new("tempdv.yaml"), s).unwrap();
}

fn read_yaml() -> Vec<Table> {

    let s = fs::read_to_string("tempdv.yaml").expect("should have been able to read the file");
    let res: Vec<Table> = serde_yaml::from_str(&s).unwrap();
    res
}

fn create_md() {
    let file = File::create("test.md").unwrap();
    let mut md = Markdown::new(file);

    md.write("Heading".heading(1)).unwrap();
    md.write("Subheading".italic().heading(2)).unwrap();

    md.write("bold".bold()).unwrap();

    md.write("first paragraph").unwrap();
    md.write(
        "Links: "
            .paragraph()
            .append("Rust".bold().link_to("https://rust-lang.org"))
            .append(", ")
            .append("Google".italic().link_to("https://google.com")),
    )
    .unwrap();

    md.write(
        List::new(true)
            .title("numbered list")
            .item("item 1")
            .item("bold".bold())
            .item(
                List::new(false)
                    .title("nested bullet list")
                    .item("bold".bold().paragraph().append("italic".italic())),
            ),
    )
    .unwrap();

    md.write("quote".quote()).unwrap();
}