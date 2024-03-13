# vigilant-archive

![Rust workflow](https://github.com/jfolkerth/vigilant_archive/actions/workflows/rust.yml/badge.svg)
[![CodeQL](https://github.com/jfolkerth/vigilant_archive/actions/workflows/codeql.yml/badge.svg)](https://github.com/jfolkerth/vigilant_archive/actions/workflows/codeql.yml)

## Description

This is a combination playground project so that I can learn Rust and digital assistant for an upcoming tabletop role-playing game I intend to run.

The project name comes from a combination of GitHub's random repo name suggestion and my own word preferences. It may change as the app grows and I understand better what it will be.

## Local Development

This project assumes a local rust setup with cargo and common tools like clippy and rustfmt. Follow the instructions for the relevant tooling to install.

Standard cargo commands are used for build and test. Scripts for working with tailwind css are in the package.json.

### Database

Create a directory called "secrets" in the project root folder and define three files: postgres-db.txt, postgres-password.txt, and postgres-user.txt. Put whatever database name, password, and username you wish to use with the postgres database in the respective files. You can see how these are mapped in the compose.yaml file.