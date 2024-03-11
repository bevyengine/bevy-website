# Tonemapping Algorithms

The files in this directory were created using the `tonemapping` example from the `bevy` repository.
The left side is the color sweep mode.
The right side is from two separate EXR images in the image viewer mode, using the `exr` feature.
The top-right image is `dragonscene_ap0_v01_1001.exr` from Scott Dyer's (Senior Imaging Engineer at Academy of Motion Picture Arts and Sciences) [source images intended for tonemapping research and testing](https://www.dropbox.com/sh/zea11rkxkivv7w7/AAD51uKh-gjCl0uCCg15G9oya/original/monsieur-lixm?dl=0&subfolder_nav_tracking=1).
The bottom-right image is `blue_bar_709.exr` which is from [Martin Smekal](https://community.acescentral.com/t/vfx-work-in-acescg-with-out-of-gamut-devices/2385/3) but the specific file used was via [Troy Sobotka's collection of testing imagery](https://github.com/sobotka/Testing_Imagery/blob/main/blue_bar_709.exr).

The example was run, PNG screenshots were taken making sure the ui text did not overlap the images, and then the images were cropped and composed using imagemagick command line tools.
