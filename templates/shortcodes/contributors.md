{% set data = load_data(path=path) %}
## Contributors

A huge thanks to the {{ data.contributors | length }} contributors that made this release (and associated docs) possible! In random order:

{% for contributor in data.contributors %}
- @{{ contributor.name }}
{% endfor %}
