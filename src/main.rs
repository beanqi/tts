use epub::doc::EpubDoc;
use scraper::Html;

fn main() {
    let doc = EpubDoc::new("TheEconomist.2024.01.20.trans.epub");
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();
    doc.go_next();
    doc.go_next();
    doc.go_next();

    let document = Html::parse_document(&doc.get_current_str().unwrap().0);
}
