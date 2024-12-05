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

### {{ guide.title }}

<div class="heading-meta">
  <div>
    <span class="heading-meta__title">Areas:</span>
    {% for area in guide.areas %}<span class="heading-meta__item">{{ area }}</span>{% if not loop.last %}, {% else %}. {% endif %}{% endfor %}
  </div>
  <div>
    <span class="heading-meta__title"> PRs:</span>
    {% for pr in guide.prs %}<a class="heading-meta__item" href="https://github.com/bevyengine/bevy/pull/{{ pr }}">#{{ pr }}</a>{% if not loop.last %}, {% endif %}
    {% endfor %}
  </div>
</div>

{{ guide_body }}

{% endfor %}
</div>
