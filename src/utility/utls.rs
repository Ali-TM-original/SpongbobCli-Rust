use scraper::{Html, Selector};

pub fn request_files()->String{
    let mut response = chttp::get("https://www.megacartoons.net/help-wanted/").unwrap();
    let body = response.body_mut().text().unwrap();
    body
}

pub fn parse_data(html_data:&str)->Vec<String>{
    let mut counter:i32 = 1;
    let document = Html::parse_document(html_data);
    let mut all_epiodes:Vec<String> = vec![]; 
    let selector = Selector::parse("a.btn.btn-sm.btn-default").unwrap();
    for element in document.select(&selector) {
        let title = element.value().attr("title").unwrap();
        let href = element.value().attr("href").unwrap();
        println!("[{}]:{} ", counter, title);
        all_epiodes.push(href.to_string());
        counter+=1;
    };
    all_epiodes

}

pub fn get_direct_link(link:&str)->String{
    let mut stream_link = "";
    let mut response = chttp::get(link).unwrap();
    let body = response.body_mut().text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"input[name="main_video_url"]"#).unwrap();
    for element in document.select(&selector){
        stream_link = element.value().attr("value").unwrap();  
    };
    stream_link.to_string()
}