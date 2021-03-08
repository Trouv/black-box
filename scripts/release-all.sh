#!/bin/bash
sh scripts/release.sh x86_64-unknown-linux-gnu
sh scripts/release.sh x86_64-pc-windows-gnu
sh scripts/release.sh x86_64-apple-darwin --features metal
