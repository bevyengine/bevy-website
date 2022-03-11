+++
title = "rendering"
weight = 6
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

TODO: Define basic rendering concepts

<!--

@alice on discord

So, I would probably split it like the ECS chapter is: briefly explain the core abstractions / pipeline, then start with the "basic tools to be productive", and then have "advanced" pages that dig into the details  to help people make the jump into helping with development or doing advanced things as a user.

@robswain on discord:

Personally I like practical documentation the most. I don’t know about 2D, but for 3D I’d probably want to start with a few common use cases. Probably first an overview of the key things involved in rendering (geometry, materials), then the basics of the core transform, mesh (maybe how to make one as an advanced topic), pbr material stuff and configuration thereof (the various parts of standard material and their meanings), lights (point, directional, how to configure them), shadows (for point and directional lights and how to configure them). Then customisation and an overview of the key things there - it would need to give a bit more of an overview of the renderer architecture with the two apps (main and render), the render stages (extract, prepare, queue are most important), and the render graph and nodes. Then to dig into customisation there are the topics of shaders, and how to get data into shaders, and have them execute when you want them to, drawing the things you want them to. It’s probably most straightforward with a use case that is embellished with the other possibilities 

-->

* overview
  * camera
  * geometry
  * materials
  * transform
  * mesh
  * pbr material
    * standard material
    * configuration
  * lights
    * point
    * directional
    * configuration
  * shadows
    * point
    * directional
    * configuration
  * Custom materials
  * renderer architechture
    * two apps
      * main
      * render
    * render stages
      * extract
      * prepare
      * queue
    * render graph
    * nodes
  * Shaders
    * custom shaders
    * shader defs
