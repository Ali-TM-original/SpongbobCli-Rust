use std::process::Command;
use crate::utility::utls;
use terminal_spinners::{SpinnerBuilder, DOTS};
use rand::Rng;

pub fn run(){
    let html_body:String = utls::request_files();
    let episodes = utls::parse_data(&html_body);
    let mut episode_len = episodes.len() as i32;
    episode_len+=1;
    // This contains links to episodes not links to the mpv files
    let num:i32 = rand::thread_rng().gen_range(0..episode_len);
    let episode_to_play = &episodes[num as usize];
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