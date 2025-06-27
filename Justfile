#!/usr/bin/env just --justfile
set dotenv-load := true

# interactive menu of recipes
default:
  @just --choose

# lists out all public recipes
list:
  @just --list

# resets the database
db-reset:
  sqlx db drop
  sqlx db create
  cd ./crates/openhack/ && sqlx migrate run

# run any matched tests or all by default
test *match:
  @cargo nextest run \
    --all-features \
    --workspace \
    --exclude openhack_ui \
    -- {{match}}

# runs all fast tests
fast-test:
  @just test \
    --skip "::query::" \
    --skip "hasher_"

# Prepare Environment for Development
init-dev:
  @just _green "Preparing Environment for OpenHack"

  @just _binstall "cargo-nextest"

  @just _binstall "mdbook" \
                  "mdbook-journal" \
                  "mdbook-mermaid" \
                  "mdbook-anchors-aweigh"

  @just _binstall "cargo-criterion"

  @just _binstall "dioxus-cli"

# Private Helpers

_green msg:
  @echo "\033[0;32m{{msg}}\033[0m"
_binstall package *additional:
  @echo
  @just _green "# Install or Update for [{{package}}]"
  @just _green "==============================================================="
  @cargo-binstall {{package}} {{additional}} --no-confirm 
