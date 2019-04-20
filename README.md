# Spriteron
Sprite simple command line application for generating ron files for 
[Amethyst](https://github.com/amethyst/amethyst), because hand writing them is a pain.


# Installation
Install using Cargo:
```
cargo install spriteron
```

# Example
Assuming you have a spritesheet called `spritesheet.png`:
```
$ spriteron spritesheet.png generate -h 4 -v 4 -d horizontal
```

`-h` separates the file in 4 horizontal columns, and `-v` in 4 vertical rows. 
`-d` sets the direction if the output to either `vertical` or `horizontal`. Useful if 
your spritesheets contains animations. 

By default the generated ron output will be printed to stdout, but it can easily be redirected
using your standard stdout direction commands: 
```
$ spriteron spritesheet.png generate -h 4 -v 4 -d horizontal > spritesheet.ron
```