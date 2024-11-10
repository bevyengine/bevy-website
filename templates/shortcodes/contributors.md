{% import "macros/path.html" as macros %}
{% set data = load_data(path=macros::release_path(version=version, path="/contributors.toml")) %}

## Contributors

A huge thanks to the {{ data.contributors | length }} contributors that made this release (and associated docs) possible! In random order:

<ul class="contributors">
{% for contributor in data.contributors %}
<li>{{ contributor.name }}</li>
{% endfor %}
</ul>
