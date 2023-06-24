/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the
/// "App" structure
/// from the "cleasy"
/// crate to make a new
/// CLI app.
use cleasy::App;

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the trait
/// to colorize strings.
use colorize::AnsiColor;

/// Importing the error
/// structure for "cleasy"
/// from the "cleasy" crate.
use cleasy::CleasyError;

/// Importing the method
/// look up emojis by shortcode.
use emojis::get_by_shortcode;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing the method to
/// delete the "dist" directory
/// in a Mandy site.
use super::utils::clean_project;

/// Importing the method
/// to compile a Mandy site
/// from "./compile.rs".
use super::compile::compile_site;

/// Importing the method
/// to serve a Mandy site
/// from "$project/dist".
use super::server::serve_project;

/// Importing the method
/// to scaffold a new Mandy site.
use super::scaffold::scaffold_site;

/// Mandy's CLI.
pub fn cli() -> () {
    let mut mandy: App = App::new(
        &"Mandy",
        &"0.1.0",
        &"Angel Dollface"
    );
    mandy.add_arg(
        &"comps",
        &"  Builds your site.",
        &"true"
    );
    mandy.add_arg(
        &"servs",
        &"  Builds and serves a Mandy site.",
        &"true"
    );
    mandy.add_arg(
        &"inits",
        &"  Initializes a new Mandy site.",
        &"true"
    );
    mandy.add_arg(
        &"reset",
        &"  Cleans a Mandy project site.",
        &"true"
    );
    mandy.add_arg(
        &"wtmpl",
        &"  Which template site to use from Git.",
        &"true"
    );
    let rocket_emoji: String = get_by_shortcode("rocket").unwrap().to_string();
    let x_emoji: String = get_by_shortcode("skull").unwrap().to_string();
    if mandy.version_is() == true {
        println!("{}", mandy.version_info().green().to_string());
    }
    else if mandy.help_is() == true {
        println!("{}", mandy.help_info().green().to_string());
    }
    else if mandy.arg_was_used(&"comps") {
        let project_dir: Result<String, CleasyError> = mandy.get_arg_data(
            &"comps"
        );
        match project_dir {
            Ok(mandy_site) => {
                let build_op: Result<(), MandyError> = compile_site(&mandy_site);
                match build_op {
                    Ok(_x) => {
                        let dist_path: &String = &format!("{}/dist", mandy_site);
                        if dir_is(dist_path){
                            let msg: &String = &format!(
                                "{} Your Mandy site has been compiled at \"{}\".",
                                &x_emoji,
                                &dist_path
                            ).red().to_string();
                            println!("{}", msg);
                        }
                        let msg: &String = &format!(
                            "{} Your Mandy site at \"{}\" has been compiled into \"dist\".",
                            &rocket_emoji,
                            &mandy_site
                        ).cyan().to_string();
                        println!("{}", msg);
                    },
                    Err(e) => {
                        let err_msg: &String = &format!("{} {}", &x_emoji, &e).red().to_string();
                        println!("{}", err_msg);
                    }
                };
            },
            Err(error) => {
                let err_msg: &String = &format!("{} {}", &x_emoji, error).red().to_string();
                println!("{}", err_msg);
            }
        };
    }
    else if mandy.arg_was_used(&"servs") {
        let project_dir: Result<String, CleasyError> = mandy.get_arg_data(
            &"servs"
        );
        match project_dir {
            Ok(mandy_site) => {
                let build_op: Result<(), MandyError> = compile_site(&mandy_site);
                match build_op {
                    Ok(_x) => {
                        let dist_path: &String = &format!("{}/dist", mandy_site);
                        if dir_is(dist_path){
                            let msg: &String = &format!(
                                "{} Your Mandy site has been compiled at \"{}\".",
                                &x_emoji,
                                &dist_path
                            ).red().to_string();
                            println!("{}", msg);
                        }
                        let msg: &String = &format!(
                            "{} Your Mandy site at \"{}\" has been compiled into \"dist\".",
                            &rocket_emoji,
                            &mandy_site
                        ).cyan().to_string();
                        println!("{}", msg);
                    },
                    Err(e) => {
                        let err_msg: &String = &format!("{} {}", &x_emoji, &e).red().to_string();
                        println!("{}", err_msg);
                    }
                };
            },
            Err(error) => {
                let err_msg: &String = &format!("{} {}", &x_emoji, error).red().to_string();
                println!("{}", err_msg);
            }
        };
    }
    else if mandy.arg_was_used(&"inits") &&
        mandy.arg_was_used(&"wtmpl") {
            match mandy.get_arg_data(&"inits"){
                Ok(project_path) => {
                    match mandy.get_arg_data(&"wtmpl"){
                        Ok(x) => {
                            match scaffold_site(&project_path, &x) {
                                Ok(_x) => {
                                    let msg: &String = &format!(
                                        "{} Your Mandy site at \"{}\" has been created.",
                                        &rocket_emoji,
                                        &project_path,
                                    ).cyan().to_string();
                                    println!("{}", msg);
                                },
                                Err(e) => {
                                    let err_msg: &String = &format!("{} {}", &x_emoji, &e).red().to_string();
                                    println!("{}", err_msg);
                                }
                            }
                        },
                        Err(repo_err) => {
                            let err_msg: &String = &format!("{} {}", &x_emoji, &repo_err).red().to_string();
                            println!("{}", err_msg);
                        }
                    }
                },
                Err(path_err) => {
                    let err_msg: &String = &format!("{} {}", &x_emoji, &path_err).red().to_string();
                    println!("{}", err_msg);
                }
            };
    }
    else if mandy.arg_was_used(&"reset") {
        let project_dir: Result<String, CleasyError> = mandy.get_arg_data(
            &"reset"
        );
        match project_dir {
            Ok(mandy_site) => {
                let dist_path: &String = &format!("{}/dist", &mandy_site);
                match clean_project(&mandy_site){
                    Ok(_x) => {
                        let msg: &String = &format!("{} Built site at \"{}\" cleaned!", &rocket_emoji, dist_path).cyan().to_string();
                        println!("{}", msg);
                    },
                    Err(e) => {
                        let err_msg: &String = &format!("{} {}", &x_emoji, &e).red().to_string();
                        println!("{}", err_msg);
                    }
                }
            },
            Err(error) => {
                let err_msg: &String = &format!("{} {}", &x_emoji, &error).red().to_string();
                println!("{}", err_msg);
            }
        };
    }
    else {
        println!("{}", mandy.help_info().red().to_string());
    }
}