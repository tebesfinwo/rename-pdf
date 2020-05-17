use lopdf::content::Content;
use lopdf::{Document, Object};
use std::collections::BTreeMap;
use std::path::Path;

fn collect_text(encoding: Option<&str>, operands: &[Object]) -> String {
    let mut text = String::new();

    for operand in operands.iter() {
        match *operand {
            Object::String(ref bytes, _) => {
                text.push_str(&Document::decode_text(encoding, bytes));
            }
            Object::Array(ref arr) => text.push_str(&collect_text(encoding, arr)),
            _ => {}
        }
    }

    text
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2, "Not enough argument: pdf_file");

    let pdf_path = Path::new(&args[1]);
    let mut doc = Document::load(pdf_path).unwrap();

    let mut title = String::new();
    for (_, page) in doc.get_pages() {
        let fonts = doc.get_page_fonts(page);
        let encodings = fonts
            .into_iter()
            .map(|(name, font)| (name, font.get_font_encoding()))
            .collect::<BTreeMap<Vec<u8>, &str>>();
        let content_data = doc.get_page_content(page);
        let content = Content::decode(&content_data.unwrap()).unwrap();

        let mut tm_count = 0;
        let mut current_encoding = None;
        for operation in &content.operations {
            if tm_count >= 2 {
                break;
            }
            match operation.operator.as_ref() {
                "Tm" => {
                    tm_count += 1;
                }
                "Tf" => {
                    let current_font = operation.operands.get(0).unwrap().as_name();
                    current_encoding = encodings.get(current_font.unwrap()).cloned();
                }
                "Tj" | "TJ" => {
                    title.push_str(&collect_text(current_encoding, &operation.operands));
                }
                _ => {}
            }
        }

        if tm_count >= 2 {
            break;
        }
    }

    let output_path = pdf_path.with_file_name(format!("{}.pdf", title.to_lowercase()));
    println!("Outputing --> {}", output_path.to_str().unwrap());
    doc.save(output_path).unwrap();
}
