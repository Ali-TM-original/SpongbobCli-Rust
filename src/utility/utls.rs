use scraper::{Html, Selector};
use std::io;
use colored;
use colored::*;


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

pub fn parse_data_list(html_data:&str){
    let document = Html::parse_document(html_data);
    let mut counter:i32 = 1;
    let selector = Selector::parse("a.btn.btn-sm.btn-default").unwrap();
    for element in document.select(&selector) {
        println!("[{}]:{} ", counter, element.value().attr("title").unwrap());
        counter+=1
    }

}

pub fn prompt_user(episodes:&Vec<String>)->i32{
    let mut user_episode_num = String::new();
    println!("{}", "\nWhich episode do you want to download? :q to quit".cyan());
    io::stdin().read_line(&mut user_episode_num).expect("Failed to read Input");
    let trimmed = user_episode_num.trim();
    let user_ep_cvt = trimmed.parse::<i32>();

    if user_episode_num.to_lowercase() == ":q".to_owned(){
        return 1000
    }

    let user_ep_cvt = match user_ep_cvt{
        Ok(num)=>num,
        Err(_)=>return 1000
    };

    if user_ep_cvt < 0 || user_ep_cvt > episodes.len() as i32{
        println!("Failed to provide proper input");
        return 1000
    }

    user_ep_cvt
}