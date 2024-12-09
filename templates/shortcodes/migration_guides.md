{% import "macros/path.html" as macros %}

{% set base_path = macros::release_path(version=version, path="/migration-guides/") %}
{% set guides_data = load_data(path=macros::path_join(path_a=base_path, path_b="/_guides.toml")) %}
{% set previous_area = "" %}

<aside class="callout callout--warning">
  <p>Bevy relies heavily on improvements in the Rust language and compiler. As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.</p>
</aside>

<div class="migration-guide">
{% for guide in guides_data.guides %}
{% set guide_body = load_data(path=macros::path_join(path_a=base_path, path_b=guide.file_name)) %}
{% if guide.areas[0] %}
{% set area_name = guide.areas[0] %}
{% else %}
{% set area_name = "Without area" %}
{% endif %}
{% set area_changed = area_name != previous_area %}

{% if area_changed %}
{% set_global previous_area = area_name %}
## {{ area_name }}
{% endif %}

{% if not area_changed %}<hr>{% endif %}

### {{ guide.title }}

<ul class="migration-guide-meta">
{% for pr in guide.prs %}
<li class="migration-guide-meta__pr"><a href="https://github.com/bevyengine/bevy/pull/{{ pr }}">PR #{{ pr }}</a></li>
{% endfor %}
{% for area in guide.areas %}
<li class="migration-guide-meta__area">{{ area }}</li>
{% endfor %}
</ul>

{{ guide_body }}

{% endfor %}

</div>
