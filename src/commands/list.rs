use crate::utility::utls;

pub fn run(){
    let html_body:String = utls::request_files();
    utls::parse_data_list(&html_body);
}
