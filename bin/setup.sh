#!/usr/bin/env bash

set -e # Fail Fast Captain!

#####[ OpenHack Setup]##########################################################
#                                                                              #
# Automated installation script that boostraps the minimum needed requirements #
# into your development environment.  Once these complete you should be able   #
# to run `just init-dev` to finish setting up your environment.                #
#                                                                              #
################################################################################

green() { # means go!
  echo -e "\033[0;32m$1\033[0m"
}

banner() { # raise the banner for war!
  echo ""
  green "# $1"
  green "======================================================================"
}

if ! command -v rustup &>/dev/null; then
  banner "Installing [rustup]"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash
else
  banner "Updating [rustup]"
  rustup update
fi

if ! command -v cargo-binstall &>/dev/null; then
  banner "Installing [cargo-binstall]"
  host="raw.githubusercontent.com"
  path="/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh"
  curl --proto '=https' --tlsv1.2 -sSf "https://${host}${path}" | bash
else
  banner "Updating [cargo-binstall]"
  cargo-binstall cargo-binstall
fi

if ! command -v just &>/dev/null; then
  banner "Installing [just]"
  cargo-binstall just
else
  banner "Updating [just]"
  cargo-binstall just
fi

echo ""
green "#############[ Setup Successful ]#################"
green "#                                                #"
green "# You should be able to run 'just init-dev' now! #"
green "#                                                #"
green "##################################################"
