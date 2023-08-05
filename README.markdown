<p align="center">
 <img src="https://raw.githubusercontent.com/angeldollface/mandy/main/assets/banner/banner.png"/>
</p>

# MANDY

![Mandy CI](https://github.com/angeldollface/mandy/actions/workflows/rust.yml/badge.svg)
![Mandy Release CI](https://github.com/angeldollface/mandy/actions/workflows/release.yml/badge.svg)

***A hypersonic static-site generator written in Rust.***

## ABOUT

***Mandy is fast, easy to use, easy to deploy, and very flexible! Get her today!***
***[Visit Mandy's website!](https://angeldollface.art/mandys-house)***

## GETTING STARTED

### Showcase

<p align="center">
 <img src="https://raw.githubusercontent.com/angeldollface/mandy/main/assets/showcase/showcase.gif"/>
</p>

### Installation via Cargo

If you already have Cargo, Rust's package manager, installed, you can install Mandy with this command on your machine:

```bash
cargo install mandy-bin
```

Alternatively, you can download a compiled binary for 64-bit desktop systems from this repository's [Releases](https://github.com/angeldollface/mandy/releases) section.

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

## INSTALLATION

To view all installation options for ***Mandy*** please view the project's official [installation page](https://angeldollface.art/mandys-house/documentation/installation/).

## MANDY TEMPLATE SITES

You can find all available template sites for starting a new Mandy-powered site [here](https://angeldollface.art/mandys-house/content/templates/).

## DEPLOYING A MANDY-POWERED SITE ON GITHUB PAGES

Mandy has her own GitHub Action to deploy your Mandy-powered site on GitHub Pages. You can find out more about that [here](https://angeldollface.art/mandys-house/documentation/deployment/). Other deployment services are also supported.

## CHANGELOG

There is a detailed log of current features and past ones [here](https://angeldollface.art/mandys-house/content/releases/).

## HELP AND COMMUNITY

- [Mandy's Website](https://angeldollface.art/mandys-house)
- [Mandy's Documentation](https://angeldollface.art/mandys-house/content/documentation/)

## NOTE

- *Mandy* by Alexander Abraham a.k.a. *"Angel Dollface"*
- Licensed under the MIT license.