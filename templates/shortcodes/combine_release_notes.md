{% import "macros/path_join.html" as macros %}

{% set release_notes_path = macros::path_join(path_a=release_content_path, path_b="/release-notes/") %}
{% set release_notes_data = load_data(path=macros::path_join(path_a=release_notes_path, path_b="/_release-notes.toml")) %}
{% for release_note in release_notes_data.release_notes %}
  {% set release_note_content = load_data(path=macros::path_join(path_a=release_notes_path, path_b=release_note.file_name)) %}
  {{ release_note_content | safe }}
{% endfor %}
