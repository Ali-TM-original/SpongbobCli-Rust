use scraper::{Html, Selector};
use crate::utility::utls;

pub fn run(){
    let html_body:String = utls::request_files();
    parse_data(&html_body);
}

fn parse_data(html_data:&str){
    let document = Html::parse_document(html_data);
    let mut counter:i32 = 1;
    let selector = Selector::parse("a.btn.btn-sm.btn-default").unwrap();
    for element in document.select(&selector) {
        println!("[{}]:{} ", counter, element.value().attr("title").unwrap());
        counter+=1
    }

}