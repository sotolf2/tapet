# Tapet a background downloader switcher

## What is this

First of all this is a project for me to learn some real world rust programming
what's better than to start a real project that actually does something for me,
I really like variety, the background switcher, but it feels quite heavy weight
on my machine, so I thought I'll try to build something that does the things that
I like with it, but hopefully with a bit less of resource usage (And for sure also
a lot less functionality).

## State

As a starter not much is here, I've found some dependencies that I need, and I'm 
starting with argument parsing and configuration parsing. And then hopefully with
time it becomes something that is actually useful.

## Planned

So what I'm thinking for now at least is to have it work basically like this:

- Save configuration in $XDG_CONFIG_HOME/tapet/tapet.toml
- Have a downloads folder for downloaded wallpapers
- Have a favourites folder for wallpapers that you've liked and want to set aside
- Choose a random picture from the downloads folder to use as wallpaper and save the path somewhere
- use tapet -f (--favorite) to copy the wallpaper to favorites folder
- for now at least using external programs such as nitrogen or feh to set the background
- autodetect to see what walpapersetter(s) are installed on the system if none are configured in the
  config
- use tapet -n to go to the next next wallpaper
  - will move the current wallpaper to the previous folder
  - choose a new downloaded wallpaper or favourite (random from one of them)
  - prune the previous wallpapers folder to keep only a set number of previous wallpapers
- build a downloader using the API of wallhaven.cc 
  - keep a set amount of wallpapers in the downloads folder
  - configurable tags to download from
  - maybe accounts features, but nothing planned there yet
- when all of the above kind of works make some daemon mode to run it with so that we don't have to
  set it up with a  cron-job if we don't want to
- add more sites to download from?
- other stuff

## Etc

As written above I'm a beginner at rust, and haven't really made something even at this scope before
so this is probably not going to become something that you can use with a guarantee of it working
well for you, but it's mostly thought as a practice thing for me, and if I get it working it will be
something that I can use for myself.
  
