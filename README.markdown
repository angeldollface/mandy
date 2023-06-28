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

I'm still testing ***Mandy*** and completing the other items on the to-do list, so I haven't figured out the conventional ways of packaging yet. To install ***Mandy*** for now, make sure you have [Git](https://git-scm.org) and [Rust](https://rust-lang.org) installed. Once you have both installed, run this command for any platform that isn't Windows:

```Powershell
. { iwr -useb https://omnitruck.chef.io/install.ps1 } | iex; install
```

For non-Windows platforms, run this command:

```bash
curl -s http://server/path/script.sh | bash -s
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
- [ ] Re-write the file-copying architecture.
- [ ] Rewrite the GitHub Action for CI testing.
- [ ] Clean up code further.
- [ ] Comment important bits further.
- [ ] Try to break `mandy`.
- [ ] Get list of important arches by using `zoxide` or `deno` as inspiration.
- [ ] Release for most important platforms.
- [ ] Write a post on Hashnode about *Mandy*.
- [ ] Make noise about *Mandy*.
- [ ] Investigate packaging options.

## HELP AND COMMUNITY :people_hugging:

- [Discord Server](https://discord.gg/VR7eZFrf)
- [Default Mandy Site Template](https://github.com/angeldollface/mandy-template-site)
- [GitHub Action](https://github.com/angeldollface/mandy-github-action)
- [Mandy's Website](https://angeldollface.art/mandys-house)
- [Mandy's Documentation](https://angeldollface.art/mandys-house/content/documentation/)

## NOTE :scroll:

- *Mandy :rocket: :pill: :fire:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
