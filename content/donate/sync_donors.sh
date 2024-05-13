#!/bin/sh

RELEASE_JSON=$(curl -sL https://api.github.com/repos/bevyengine/bevy-donors/releases/latest)
METRICS_URL=$(echo $RELEASE_JSON | jq -r '.assets[] | select(.name? | match("metrics.toml$")) | .browser_download_url')
DONORS_URL=$(echo $RELEASE_JSON | jq -r '.assets[] | select(.name? | match("donors.toml$")) | .browser_download_url')
TARBALL_URL=$(echo $RELEASE_JSON | jq -r '.tarball_url')
curl --location  "$METRICS_URL" > metrics.toml
curl --location  "$DONORS_URL" > donors.toml
curl --location  "$TARBALL_URL" | tar zxf - --wildcards "bevyengine-bevy-donors-*/logos"
cp -r bevyengine-bevy-donors-*/logos ../../static/assets/donor_logos
rm -r bevyengine-bevy-donors-*