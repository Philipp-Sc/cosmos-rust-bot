<div align="center">

  <p>🤖</p>
  <h1>TRB - Terra Rust Bot™</h1>  
  </div>
 
 
## Install

> Tested on Linux.
> Tested on Windows Subsystem for Linux / Ubuntu.


**Install Rust**

* <a href="https://doc.rust-lang.org/book/ch01-00-getting-started.html">Get started here.</a>
* On Linux: Download the file with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust.sh`, view
  it: `less ./rust.sh`, and run the script `./rust.sh` to start rustup installation. The script makes PATH changes only
  to login shell configuration files. You need to `source ~/.cargo/env` until you logout and login back into the system.
  To update rustup afterwards, run `rustup self update`.
* Note: Works both with edition = "2018" and edition = "2021" (Cargo.toml). If you do not want to use the nightly
  version, just edit the config (Cargo.toml).
* To use the nightly edition (edition = "2021") install it with: `rustup default nightly && rustup update`.


* On WSL: You may need to install the following packages first:
* `sudo apt-get install build-essential libssl-dev pkg-config`


* On Fedora: You may need to install the openssl package:
* `sudo dnf install openssl openssl-devel`

**Clone the repository**

* `git clone https://github.com/Philipp-Sc/terra-rust-bot.git`


* `cd ./terra-rust-bot/install`

**Build terra-rust-bot using the installer script**


*You can choose between three different build options:*

* `dev` fast build
* `prod` optimized build
* `native` optimize the build for your CPU

*Additionally you can either build from the local source code or the latest remote code.*

- `local` builds from the already cloned repository
- `remote` gets the latest source code and builds terra-rust-bot


* `nohup ./install.sh dev local all &` (builds `all` packages, to - for example - disable the signal messenger
  integration use `default` instead. Use `minimal` to only build the terra-rust-bot.)
  
* Note: check the file `nohup.out` for errors. (`cat nohup.out`)
* Note: after a successful build you can remove everything except `terra-rust-bot/install/build`
