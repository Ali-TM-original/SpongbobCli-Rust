use std::io;
use colored;
use colored::*;
use std::process::Command;
use crate::utility::utls;
use terminal_spinners::{SpinnerBuilder, DOTS};

pub fn run(){
    let html_body:String = utls::request_files();
    let episodes = utls::parse_data(&html_body);

    // This contains links to episodes not links to the mpv files
    loop{
    let mut user_episode_num = String::new();
    println!("{}", "\nWhich episode do you want to play? :q to quit".cyan());
    io::stdin().read_line(&mut user_episode_num).expect("Failed to read Input");
    let trimmed = user_episode_num.trim();
    let user_ep_cvt = trimmed.parse::<i32>();

    let user_ep_cvt = match user_ep_cvt{
        Ok(num)=>num,
        Err(_)=>break
    };

    if user_ep_cvt < 0 || user_ep_cvt > episodes.len() as i32{
        panic!("Please input correct episode number")
    }
    let episode_to_play = &episodes[user_ep_cvt as usize -1];
    let stream_link = utls::get_direct_link(episode_to_play);
    let mut owned_string: String = " Now playing ".to_owned();
    owned_string.push_str(episode_to_play);
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(owned_string).start();
    Command::new("mpv")
        .arg(stream_link)
        .output()
        .expect("failed to execute process");

    handle.done();
    
    }
}
