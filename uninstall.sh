#!/bin/sh
set -e

REPO="https://github.com/Samujalphukan228/fetchy"
curl -fsSL "$REPO/raw/master/install.sh" | sh -s -- --uninstall