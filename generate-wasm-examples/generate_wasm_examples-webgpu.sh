#!/bin/sh

git init bevy-main
cd bevy-main
git remote add origin https://github.com/mockersf/bevy
git pull --depth=1 origin example-runner

cargo run -p example-showcase -- build-website-list --content-folder content
mv content ../../content/examples-webgpu
