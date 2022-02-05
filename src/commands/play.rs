use std::process::Command;
use crate::utility::utls;
use terminal_spinners::{SpinnerBuilder, DOTS};

pub fn run(binge:bool){
    let html_body:String = utls::request_files();
    let episodes = utls::parse_data(&html_body);

    if binge{
        let mut episode = 1;
        loop{
            let episode_to_play = &episodes[episode -1];
            let stream_link = utls::get_direct_link(episode_to_play); 
            let mut owned_string: String = " Now playing ".to_owned();
            owned_string.push_str(episode_to_play);
            let handle = SpinnerBuilder::new().spinner(&DOTS).text(owned_string).start();
            Command::new("mpv")
            .arg(stream_link)
            .output()
            .expect("failed to execute process");

            handle.done();
            episode+=1;
        }
    }else{
        loop{
        let user_ep_cvt = utls::prompt_user(&episodes);
        if user_ep_cvt == 1000 {break}
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
}
