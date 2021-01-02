# Tapet a background downloader switcher

## What is this

Tapet is a wallpaper switcher that I'm working on to learn Rust, it works for me so far, but I
don't guarantee that it will work for everyone, I'm very happy to get suggestions or bug
reports if there is anything off with it. There are probably a lot of things that are still
suboptimal, but I hope to make it better as I learn more.

## Usage
    Tapet 0.1
    A wallpaper helper

    USAGE:
        tapet [FLAGS]

    FLAGS:
        -f, --favorite            Saves the current wallpaper in the favorites
        -h, --help                Prints help information
        -n, --next                sets the next wallpaper
        -r, --random-favourite    Set a random wallpaper from the favourites folder
        -R, --restore             Restores current wallpaper
        -u, --update              Updates new wallpapers
        -V, --version             Prints version information


## State

I've implemented the basic functionality now, and I'm very happy with the start of it, so much so
that I can start to use it for myself switching wallpapers work, favouriting works, and basic
downloading works, I still need to figure out how to make it not redownload images that you
have already seen, but it's something that I'll be working on still

## How does it work

- Saves configuration in $XDG_CONFIG_HOME/tapet/tapet.toml
- For an example one check out the one in the repository
- The downloads folder will store a set amount of wallpapers that tapet has downloaded using one
  of it's supported sites (only wallhaven for now).
- The favourites folder saves your favourited wallpapers
- It uses feh or nitrogen for setting the wallpaper, it's pretty easy to implement more, so
  if I get some tips I can easily add some more here.
  config
- when used with `tapet -n` it goes to the next next wallpaper
  - will move the current wallpaper to the previous folder
  - choose a new downloaded wallpaper or favourite (random from one of them)
  - prune the previous wallpapers folder to keep only a set number of previous wallpapers
- when used with `tapet -u`
  - will use the wallhaven.cc API (for now only that one) 
  - keep a set amount of wallpapers in the downloads folder
  - if you want to search specified tags this can be configured in the configuration file
  set it up with a  cron-job if we don't want to

## Planned for the future

- The first thing is to make the download not download already seen images
- autodetect to see what walpapersetter(s) are installed on the system if none are configured in the
- when all of the above kind of works make some daemon mode to run it with so that we don't have to
- add more sites to download from?
- maybe wallhaven accounts features, but nothing planned there yet
- other stuff
