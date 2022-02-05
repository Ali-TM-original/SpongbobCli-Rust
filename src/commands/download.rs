use colored;
use colored::*;
use std::process::Command;
use crate::utility::utls;
use terminal_spinners::{SpinnerBuilder, DOTS};

pub fn run(downone:bool){
    let html_body:String = utls::request_files();
    let episodes = utls::parse_data(&html_body);
    loop{
        if downone{
            let user_opt = utls::prompt_user(&episodes);
            if user_opt == 1000{break}
            let episode_to_play = &episodes[user_opt as usize -1];
            let stream_link = utls::get_direct_link(episode_to_play);
            let mut owned_string: String = " Downloading ".to_owned();
            owned_string.push_str(episode_to_play);
            let handle = SpinnerBuilder::new().spinner(&DOTS).text(owned_string).start();
            let out = Command::new("youtube-dl")
                .arg(stream_link)
                .output()
                .expect("failed to execute process");
            handle.done();
            if out.status.success(){
                println!("{} {}", "Downloaded Episode".green(), episode_to_play);  
            }else{
                println!("{} {}", "Could Not Downloaded Episode".red(), episode_to_play);  
            }
        }else{
            println!("{}", "Getting Ready to Download all Episodes".red());
            for i in &episodes{
                let stream_link = utls::get_direct_link(i);
                let mut owned_string: String = " Downloading ".to_owned();
                owned_string.push_str(&i);
                let handle = SpinnerBuilder::new().spinner(&DOTS).text(owned_string).start();
                let out = Command::new("youtube-dl")
                .arg(stream_link)
                .output()
                .expect("failed to execute process");
                handle.done();
                if out.status.success(){
                    println!("{} {}", "Downloaded Episode".green(), &i);  
                }else{
                    println!("{} {}", "Could Not Downloaded Episode".red(), &i);  
                }
            }
        }
    }
}

