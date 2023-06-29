<p align="center">
 <img src="/assets/banner/banner.png"/>
</p>

# MANDY :rocket: :pill: :fire:

![Mandy CI](https://github.com/angeldollface/mandy/actions/workflows/rust.yml/badge.svg)

***A hypersonic static-site generator written in Rust. :rocket: :pill: :fire:***

## GETTING STARTED :books:

### Showcase

<p align="center">
 <img src="/assets/showcase/showcase.gif"/>
</p>

### Get up and running!

```bash
# Check that Mandy is installed.
mandy -v

# Set the $MANDY_ENV environment variable. (For *Nix systems.)
export MANDY_ENV="development"

# Set the $MANDY_ENV environment variable. (For Windows systems.)
set MANDY_ENV="development"

# Start a new Mandy-powered site in "mysite".
mandy -i mysite -w angeldollface/mandy-template-site

# Compile your Mandy site.
mandy -c mysite

# Clean your Mandy project.
mandy -r mysite

# Test your site on "localhost".
mandy -s mysite
```

## INSTALLATION :inbox_tray:

Installing ***Mandy*** is simple! Run one of the commands below for your platform to download a script that will install ***Mandy***.

- Run this command in a Powershell session with administrator privileges if you are running on Windows 32-bit:

```Powershell
. { iwr -useb https://angeldollface.art/mandys-house/installers/win_32.ps1 } | iex; ./win_32.ps1
```

- Run this command in a Powershell session with administrator privileges if you are running on Windows 64-bit:

```Powershell
. { iwr -useb https://angeldollface.art/mandys-house/installers/win_64.ps1 } | iex; ./win_64.ps1
```

For non-Windows platforms, run this command. You may have to put `sudo` in front of this command:

```bash
curl -s https://angeldollface.art/mandys-house/installers/nix.sh | bash -s
```

## TO DO :black_nib:

- [x] Comment the code.
- [x] Add a `config` flag for copying an assets directory.
- [x] Add an environment variable detector.
- [x] Fix navigation issue.
- [x] Fix iterative content issue.
- [x] Make a Discord server.
- [x] Publish GitHub action for *Mandy*.
- [x] Make banner, logo, and icon.
- [x] Implement support for SASS.
- [x] Implement project scaffolding.
- [x] Finish the compilation module.
- [x] Fix config flag bugs.
- [x] Clean up the code.
- [x] Added colored output messages.
- [x] Added emojis.
- [x] Preliminary testing completed.
- [x] Fixed the `assetsDir` bug.
- [x] Changed some CLI-related things.
- [x] Fix the `about.markdown` to `about/index.html` bug.
- [x] Write documentation.
- [x] Make Mandy's website.
- [x] Cleanup this `README` by using `zoxide` or `deno` as inspiration.
- [x] Re-write the file-copying architecture.
- [x] Write installers for all important platforms.
- [x] Rewrite the GitHub Action for CI testing.
- [ ] Edit the documentation on Mandy's website.
- [ ] Re-order the "DEPLOYMENT" and "ENVIRONMENT VARIABLES" section in the documentation.
- [ ] Buy domain name AFTER first release.
- [ ] Check EVERYTHING before release.
- [ ] Clean up code further.
- [ ] Comment important bits further.
- [ ] Try to break `mandy`.
- [ ] Write a post on Hashnode about *Mandy*.
- [ ] Make noise about *Mandy*.

## MANDY TEMPLATE SITES :art:

- [Mandy's Default Site Template](https://github.com/angeldollface/mandy-template-site): The default template site for creating new Mandy-powered sites. Start a new Mandy site with this site template like this: `mandy -i mysite -w angeldollface/mandy-template-site`.
- [Mandy's main website](https://github.com/angeldollface/mandys-house): The Mandy site for the project page of the Mandy project. Start a new Mandy site with this site template like this: `mandy -i mysite -w angeldollface/mandys-house`.
- [Mandy's fancy template site with in-browser 3D graphics](https://github.com/angeldollface/mandy-threejs-site): A single-page site template using three.js for the Mandy static-site generator. Start a new Mandy site with this site template like this: `mandy -i mysite -w angeldollface/mandy-threejs-site`.

## DEPLOYING A MANDY-POWERED SITE ON GITHUB PAGES :rocket:

Mandy has her own GitHub action to deploy your Mandy-powered site on GitHub Pages. You can find out more about that [here](https://github.com/angeldollface/mandy-github-action).

## HELP AND COMMUNITY :people_hugging:

- [Discord Server](https://discord.gg/VR7eZFrf)
- [Mandy's Website](https://angeldollface.art/mandys-house)
- [Mandy's Documentation](https://angeldollface.art/mandys-house/content/documentation/)

## NOTE :scroll:

- *Mandy :rocket: :pill: :fire:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
