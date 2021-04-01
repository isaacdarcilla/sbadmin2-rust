<p align="center"><img width="120" src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fupload.wikimedia.org%2Fwikipedia%2Fcommons%2Fthumb%2Fd%2Fd5%2FRust_programming_language_black_logo.svg%2F1200px-Rust_programming_language_black_logo.svg.png&f=1&nofb=1"></p>

<h2 align="center">Rust SB Admin 2</h2>

<p align="center">Admin dashboard template for Rust and Rocket.rs based on SbAdmin2.</p>

![Screenshot](https://github.com/isaacdarcilla/sbadmin2-rust/blob/master/img/1.png)

## 🚀 Demonstration

<a target="_blank" href="https://rust-admin.herokuapp.com/">
  <img src="https://www.herokucdn.com/deploy/button.svg" alt="Deploy">
</a>

## 🚀 Prerequisites

* Rust Nightly Version
* Git Version Control
* Cargo
* Heroku (Optional)

## 🚀 Installation in Host

* `$ git clone https://github.com/isaacdarcilla/sbadmin2-rust.git`
* `$ cd sbadmin2-rust` 
* `$ cargo run`

## 🚀 Deploying to Heroku

* `$ heroku create --buildpack https://github.com/emk/heroku-buildpack-rust.git`
* `$ git remote add heroku https://git.heroku.com/<heroku-project-name>.git`
* `$ echo "web: ROCKET_PORT=$PORT ROCKET_ENV=prod ./target/release/sbadmin_rust" > Procfile`
* `$ echo "VERSION=nightly" > RustConfig`
* `$ git add . && git commit -m "Add Heroku deploy configuration"`
* `$ git push heroku master`

## 🚀 Developer

Developed by Isaac [(facebook.com/isaacdarcilla)](https://web.facebook.com/isaacdarcilla)

## 🚀 Support

Fork or star this repository for support.
