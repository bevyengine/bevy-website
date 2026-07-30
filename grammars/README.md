# Extra Highlighting Grammars

This folder contains [TextMate grammars](https://macromates.com/manual/en/language_grammars) in JSON form,
for languages that are not bundled with Zola's syntax highlighter.

They are loaded via `extra_grammars` in the `[markdown.highlighting]` section of `config.toml`.

> **Note**
> As of Zola 0.22, highlighting is handled by [Giallo](https://github.com/getzola/giallo) rather than Syntect.
> Giallo consumes JSON TextMate grammars; the `.sublime-syntax` files this folder previously held are no
> longer supported. Grammar and theme JSON files can be browsed at
> <https://textmate-grammars-themes.netlify.app/>.

## RON

The Rusty Object Notation grammar is `ron.json` from
[shikijs/textmate-grammars-themes](https://github.com/shikijs/textmate-grammars-themes/blob/main/packages/tm-grammars/grammars/ron.json),
which sources it from [a5huynh/vscode-ron](https://github.com/a5huynh/vscode-ron).
It is licensed under the MIT license, which you can find at [`LICENSE-RON`](./LICENSE-RON).

## WGSL

No longer vendored — Zola bundles a `wgsl` grammar out of the box.
