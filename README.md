
  
# spongebob-cli Rust
Watch classic spongebob from the terminal!
``
Note: THIS PROJECT IS A REMAKE OF THE ORIGINAL SPONGBOBCLI (originally written in Python) IN RUST
  ``
[Link to original Project](https://github.com/trakBan/spongebob-cli)

![Screenshot of app](./screenshots/Capture.PNG)
![Help command of app](./screenshots/help.PNG)


### Dependecies:
-   mpv player  [https://mpv.io/](https://mpv.io/)  (Must be installed through a package manager and added to path ENV)
- Youtube-dl [Official Repo](https://github.com/ytdl-org/youtube-dl) ( Must be installed and added to ENV PATH)
<h1>The main entry point is in ./src/main.rs</h1>

### Installation:
[Check out the crate at Crates.io](https://crates.io/crates/spongbobcli)
- Download and install Cargo from here ==> [Official Rust Compiler

```FIX
cargo install spongbobcli
```

### Build from scratch ( recommended to build your own :)  )
- Download and install Cargo from here ==> [Official Rust Compiler](https://www.rust-lang.org/)
- Clone the repo using git or any other method
- use command ``cargo build --release `` (takes time but is optimized) or ``cargo build`` (not optimized)
- now go to the ``.\target\release`` Directory and find the exe 
- add the exe to path and voila

### Currently In development
This is just a small project i picked to learn Rust ♥

# Things to fix/Do
- Refactor Things


# Contributing
Clone the repo. Make sure you have mpv installed to path. Work on the feature that you want. Make sure to cre