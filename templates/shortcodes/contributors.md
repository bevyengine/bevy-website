{% set data = load_data(path=path) %}

## Contributors

A huge thanks to the {{ data.contributors | length }} contributors that made this release (and associated docs) possible! In random order:

<ul>
{% for contributor in data.contributors %}
<li>{{ contributor.name }}</li>
{% endfor %}
</ul>
