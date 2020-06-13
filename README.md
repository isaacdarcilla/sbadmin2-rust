<p align="center"><img width="200" src="https://cdn.dribbble.com/users/297466/screenshots/2783613/1.jpg"></p>

<h2 align="center">Rust Admin 2</h2>

<p align="center">🚀 Admin dashboard template for Rust and Rocket.rs based on SbAdmin2</p>

![Screenshot](https://github.com/isaacdarcilla/sbadmin2-rust/blob/master/img/1.png)

## 🚀 Prerequisites

* Rust Nightly Version
* Git Version Control
* Cargo
* Heroku (Optional)

## 🚀 Installation

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