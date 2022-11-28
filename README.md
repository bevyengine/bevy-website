# Bevy Website

The source files for <https://bevyengine.org>. This includes official Bevy news and docs, so if you would like to contribute feel free to create a pull request!

## Zola

The Bevy website is built using the Zola static site engine. In our experience, it is fast, flexible, and straightforward to use.

To check out any local changes you've made:

1. [Download Zola](https://www.getzola.org/).
2. Clone the Bevy Website git repo and enter that directory:
   1. `git clone https://github.com/bevyengine/bevy-website.git`
   2. `cd bevy-website`
3. Start the Zola server with `zola serve`.

A local server should start and you should be able to access a local version of the website from there.

## Assets Page

To access Assets Page with successfully, it's necessary to go into the repository in **.\generate-assets** and run the command in terminal: <br>
``generate_assets.sh``<br>
If you are on Windows, maybe this cannot work, so It`s suggest recommending to use WSL or Git Bash.<br>

## Assets generation

Assets are generated using data from /generate/assets and crates.io using their [datadump](https://crates.io/data-access) trough [cratesio-dbdump-lookup](https://github.com/alyti/cratesio-dbdump-lookup).
Please notice when unpacked, the crates.io dump fills about 500 mb.
