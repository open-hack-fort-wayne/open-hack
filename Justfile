#!/usr/bin/env just --justfile
set dotenv-load := true

# interactive menu of recipes
default:
  @just --choose

# lists out all public recipes
list:
  @just --list

# Prepare Environment for Development
init-dev:
  @just _green "Preparing Environment for OpenHack"

  @just _binstall "cargo-nextest"

  @just _binstall "mdbook" \
                  "mdbook-journal" \
                  "mdbook-mermaid"

  @just _binstall "cargo-criterion"

# Private Helpers

_green msg:
  @echo "\033[0;32m{{msg}}\033[0m"
_binstall package *additional:
  @echo
  @just _green "# Install or Update for [{{package}}]"
  @just _green "==============================================================="
  @cargo-binstall {{package}} {{additional}} --no-confirm 
