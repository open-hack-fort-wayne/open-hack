# Open Hack

An application to support software development enthusiast meetup groups.

## 🔌 Preparing Local Development Environment

The goal of this project is to drive as much of the development tasks
with the `just` command runner; however, that's not very helpful if
you don't have the runner already installed.  The following steps will
get you ready to begin local development.

1. Running the following script will ensure that you can initialize
   the development environment with `just` by providing these packages:

   - `rustup` : Rust Package Manager
   - `cargo-binstall` : Installs pre-built cargo binary packages
   - `just` : Simple Command Runner

   ```bash
   ./bin/setup.sh
   ```

2. Once these packages are installed all that's left is to run:

   ```bash
   just init-dev
   ```

## 🧩 Roadmap

- [ ] Base Project Setup
  - [x] Development Setup Support
  - [ ] Markdown Book
  - [ ] Cargo Workspaces Setup
    - [ ] `app`
    - [ ] `context`
    - [ ] `db`
    - [ ] `entity`
    - [ ] `www`
  - [ ] `Justfile` Workflow
    - [ ] Build
    - [ ] Test
    - [ ] Documentation
    - [ ] Formatting
  - [ ] Github Workflows Setup
    - [ ] Docs
    - [ ] Release
    - [ ] CI Tests
