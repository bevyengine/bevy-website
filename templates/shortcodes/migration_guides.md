{% import "macros/path.html" as macros %}

{% set base_path = macros::release_path(version=version, path="/migration-guides/") %}
{% set guides_data = load_data(path=macros::path_join(path_a=base_path, path_b="/_guides.toml")) %}

<aside class="callout callout--warning">
  <p>Bevy relies heavily on improvements in the Rust language and compiler. As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.</p>
</aside>

<div class="migration-guide">
{% for guide in guides_data.guides %}
{% set guide_body = load_data(path=macros::path_join(path_a=base_path, path_b=guide.file_name)) %}

### {{ guide.title }}
<ul class="migration-guide-pr-list">
{% for pr in guide.prs %}
<li><a href="https://github.com/bevyengine/bevy/pull/{{ pr }}">PR #{{ pr }}</a></li>
{% endfor %}
</ul>

<div class="migration-guide-area-tags">
{% for area in guide.areas %}
<div class="migration-guide-area-tag">{{ area }}</div>
{% endfor %}
</div>

{{ guide_body }}

{% endfor %}
</div>
