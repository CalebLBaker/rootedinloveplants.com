
#!/bin/bash

rm -rf public/*
cargo run content.json
mv index.html public
cp -r images public/
cp styles.css public
cp index.js public
cp -r fancybox public/

