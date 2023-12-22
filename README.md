# vigilant-archive

![Rust workflow](https://github.com/jfolkerth/vigilant_archive/actions/workflows/rust.yml/badge.svg)
[![CodeQL](https://github.com/jfolkerth/vigilant_archive/actions/workflows/codeql.yml/badge.svg)](https://github.com/jfolkerth/vigilant_archive/actions/workflows/codeql.yml)

## Description

This is a combination playground project so that I can learn Rust and digital assistant for an upcoming tabletop role-playing game I intend to run.

The project name comes from a combination of GitHub's random repo name suggestion and my own word preferences. It may change as the app grows and I understand better what it will be.

## Local Development

This project assumes a local rust setup with cargo and common tools like clippy and rustfmt. Follow the instructions for the relevant tooling to install.

Standard cargo commands are used for build and test. The standalone tailwind cli is used for generating css styles. The command to run that is `<path/to/standalone/cli> -i static/input.css -o static/styles.css`.

### Tests

The dependencies test checks that the versions of certain libraries are up-to-date. Specifically this is used for tailwindcss because I've elected to use the standalone cli instead of adding node to the project just for managing this library and for htmx because hosting the file oneself is the recommended way to use it. Because GitHub's api requires a user agent to be set, the test reads the value from an environment variable VIGILANT_ARCHIVE_USER_AGENT.