{% import "macros/path_join.html" as macros %}

{% set guides_path = macros::path_join(path_a=release_content_path, path_b="/migration-guides/") %}
{% set guides_data = load_data(path=macros::path_join(path_a=guides_path, path_b="/_guides.toml")) %}
{% for guide in guides_data.guides %}
  {% set guide_body = load_data(path=macros::path_join(path_a = guides_path, path_b = guide.file_name)) %}

### [{{ guide.title }}]({{ guide.url }})

<div class="migration-guide-area-tags">
  {% for area in guide.areas %}
    <div class="migration-guide-area-tag">{{ area }}</div>
  {% endfor %}
</div>

{{ guide_body }}

{% endfor %}
