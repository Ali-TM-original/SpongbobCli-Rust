use clap::{App, AppSettings};
mod commands;
mod check_deps;
mod utility;

fn main() {
    let version: String = String::from("Parrot-1.1.0");
    check_deps::check_deps();
    let app = App::new("SpongbobCli")
        .version(&*version)
        .author("Ali™")
        .about("Watch classic spongebob from the terminal!")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::AllowHyphenValues)
        .bin_name("SpongCli")
        .subcommand(App::new("play").alias("-p").about("Play a given episode. Alias = -p"))
        .subcommand(App::new("list").alias("-ls").about("List all playable episodes. Alias = -ls"))
        .subcommand(App::new("download").alias("-dn").about("Download a given episode. Alias = -dn"))
        .subcommand(App::new("rand").alias("-rn").about("Play a random episode. Alias = -rn"));

        let matches = app.get_matches();
    
        if matches.subcommand_matches("play").is_some(){
            commands::play::run();
        }else if matches.subcommand_matches("rand").is_some(){
            commands::rand::run()
        }
        else if matches.subcommand_matches("list").is_some(){
            commands::list::run();
        }else if matches.subcommand_matches("download").is_some(){
            commands::download::run();
        }
}

//https://www.megacartoons.net/video/SpongeBob-SquarePants-Help-Wanted.mp4