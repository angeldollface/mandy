/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the "Server"
/// structure from the "file-serve"
/// crate to serve a directory on
/// "localhost".
use file_serve::Server;

/// Importing the trait
/// to colorize strings.
use colorize::AnsiColor;

/// Importing the method to
/// look up emojis by shortcode.
use emojis::get_by_shortcode;

/// A function to serve static files in a directory.
/// Returns nothing.
pub fn serve_project(project_dir: &String) -> () {
    let dir_to_serve: &String = &format!("{}/dist", project_dir);
    let rocket_emoji: String = get_by_shortcode("rocket").unwrap().to_string();
    let x_emoji: String = get_by_shortcode("skull").unwrap().to_string();
    if dir_is(dir_to_serve){
        let server = Server::new(&dir_to_serve);
        let msg: String = format!(
            "{} Serving your site on \"http://localhost:{}\".\n{} Press Ctrl+C to exit.", 
            rocket_emoji, 
            server.addr(),
            rocket_emoji
        ).cyan().to_string();
        println!("{}", msg);
        match server.serve(){
            Ok(_x) => {}
            Err(e) => {
                let msg: String = format!(
                    "{} {}", 
                    x_emoji, 
                    &e.to_string()
                ).red().to_string();
                println!("{}", msg);
            }
        }
    }
    else {
        let msg: String = format!("{} Directory \"{}\" not found.", x_emoji, dir_to_serve).red().to_string();
        println!("{}", msg);
    }
}