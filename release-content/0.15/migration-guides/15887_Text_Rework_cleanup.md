Doubles as #15591 migration guide.

Text bundles (`TextBundle` and `Text2dBundle`) were removed in favor of `Text` and `Text2d`.
Shared configuration fields were replaced with `TextLayout`, `TextFont` and `TextColor` components.
Just `TextBundle`’s additional field turned into `TextNodeFlags` component,
while `Text2dBundle`’s additional fields turned into `TextBounds` and `Anchor` components.

Text sections were removed in favor of hierarchy-based approach.
For root text entities with `Text` or `Text2d` components, child entities with `TextSpan` will act as additional text sections.
To still access text spans by index, use the new `TextUiReader`, `Text2dReader` and `TextUiWriter`, `Text2dWriter` system parameters.
