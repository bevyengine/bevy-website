<!-- Cosmic text -->
<!-- https://github.com/bevyengine/bevy/pull/10193 -->

Historically, Bevy has used the `ab_glyph` library to render text. This handled simple latin text rendering reasonably well. But Bevy aims to be a generic app framework usable with any language, and there were a number of areas where `ab_glyph` wasn't meeting our needs.

The Rust text space has evolved significantly since we selected `ab_glyph`. Fortunately there are a number of good options now. We chose [`cosmic-text`](https://github.com/pop-os/cosmic-text) because of its robust feature set and its use in production applications (Iced, Cosmic Desktop, Zed, Lapce, etc). Notably, `cosmic-text` gives us support for:

* **Font Shaping**: The ability to take a string of character codes and perform layout and transformation rules. This can involve moving, modifying, and combining characters (such as ligatures). This is _extremely_ important for non-Latin-based languages.
* **System Font Loading**: The ability to scan for the available fonts installed on a system and load them.
* **Bidirectional Text**: Not all languages go from left to right! Cosmic Text gives us support for bidirectional text.
* **Text Editing**: Cosmic Text has its own internal text editing model, which we can take advantage of.

In **Bevy 0.15** we ported our text rendering to `cosmic-text`. This was largely an internal change (unlike the other "high level" text API changes this release, such as the port to Required Components).

That being said, you will definitely notice our improved ability to render text! Here is Bevy rendering Arabic text, right-to-left, using the Noto Sans Arabic font:

![arabic text](arabic_text.png)

Note that we haven't yet wired up `cosmic-text`'s "system font loading" features, but we're working on it!
