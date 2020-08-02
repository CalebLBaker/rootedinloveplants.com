#!/bin/bash

rm -rf public/*
cargo run content.json
mv index.html public
cp -r images public/
cp syles.css publi
firebase deploy

