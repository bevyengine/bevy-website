#!/bin/bash
rm -rf public
zola build
./pagefind
