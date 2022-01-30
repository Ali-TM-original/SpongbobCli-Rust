use which::which;

pub fn check_deps() {
    let check_mpv = which("mpv");
    let check_ytdl = which("youtube-dl");
    let _ = match check_mpv {
        Ok(res) => res,
        Err(_) => {
            panic!("Could Not Find Package Mpv");
        }
    };

    let _ = match check_ytdl {
        Ok(res) => res,
        Err(_) => {
            panic!("Could not Find Package Ytdl")
        }
    };
}
