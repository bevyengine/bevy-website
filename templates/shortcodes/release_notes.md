{% import "macros/path.html" as macros %}

{% set base_path = macros::release_path(version=version, path="/release-notes/") %}
{% set release_notes_data = load_data(path=macros::path_join(path_a=base_path, path_b="/_release-notes.toml")) %}
{% for release_note in release_notes_data.release_notes %}
{% set release_note_body = load_data(path=macros::path_join(path_a=base_path, path_b=release_note.file_name)) %}

### {{ release_note.title }}

<ul class="release-feature-meta">
  <li>Authors: {{ release_note.authors | join(sep=", ")}}</li>
  <li><a href="{{ release_note.url }}">Pull Request</a></li>
</ul>

{{ release_note_body | replace(from='POST_PATH', to=page.colocated_path) | markdown }}

{% endfor %}
