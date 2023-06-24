/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the "serve_dir"
/// function from the "coutils"
/// crate to serve directories.
use coutils::serve_dir;

/// We import the "ServerInfo"
/// struct from the "coutils"
/// crate for more flexibility.
use coutils::ServerInfo;

/// A function to serve static files in a directory.
/// Returns nothing.
pub fn serve_project(project_dir: &String) -> () {
    let dir_to_serve: &String = &format!("{}/dist", project_dir);
    let server_info: ServerInfo = ServerInfo::new(
        &true,
        &Some("Serving your site!".to_string())
    );
    serve_dir(dir_to_serve, &server_info);
}