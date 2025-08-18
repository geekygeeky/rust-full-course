# Rust Crash Course

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A simple collection of lessons that I used to get started with the Rust Programming language.

## Setup Rust on your machine

If you already have Rust setup, you can skip this section.

Windows: [https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

MacOS/Linux/WSL: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

> WSL means Windows Subsystem for Linux

For more information, check out the [Rust installation guide](https://www.rust-lang.org/tools/install).

## Setup locally

If you wish to setup the project locally, you can clone the repositories and follow the guide below to run each lesson and create new ones.

### Clone the project

```bash
git clone https://github.com/geekygeeky/rust-full-course
```

### Run each lesson

To run a lesson, just navigate into the directory and use the `make run` command. This would compile the lesson using the Rust compiler and also run the lesson.

#### Example

If you want to run the `1_primivitive_data_types` lesson. Use the command below.

```bash
# We are currently inside the primitive-data-types directory
$ make run p=1_primivitive_data_types
```

### Create new lesson

To create a lesson, make sure you are in the root directory then run the `make run p=<new-lesson>` command. This would create the barebones of what you need to get started running your lessons.

> Replace `<new-lesson>` with your desired lesson name, e.g. `make run p=rust-db-lesson`

## Acknowledgement

This repo is inspired by a [Youtube course on Rust](https://www.youtube.com/watch?v=rQ_J9WH6CGk), you can check out the video if you would like to follow up each lesson with a video.