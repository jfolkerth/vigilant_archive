# vigilant-archive

![Rust workflow](https://github.com/jfolkerth/vigilant_archive/actions/workflows/rust.yml/badge.svg)

## Description

This is a combination playground project so that I can learn Rust and digital assistant for an upcoming tabletop role-playing game I intend to run.

The project name comes from a combination of GitHub's random repo name suggestion and my own word preferences. It may change as the app grows and I understand better what it will be.

## Local Development

This project assumes a local rust setup with cargo and common tools like clippy and rustfmt. Follow the instructions for the relevant tooling to install.

Standard cargo commands are used for build and test. The standalone tailwind cli is used for generating css styles. The command to run that is `<path/to/standalone/cli> -i static/input.css -o static/styles.css`.