{% import "macros/path.html" as macros %}

{% set base_path = macros::release_path(version=version, path="/release-notes/") %}
{% set release_notes_data = load_data(path=macros::path_join(path_a=base_path, path_b="/_release-notes.toml")) %}
{% for release_note in release_notes_data.release_notes %}
{% set release_note_body = load_data(path=macros::path_join(path_a=base_path, path_b=release_note.file_name)) %}

## {{ release_note.title }}

<div class="release-feature-meta">
  <div>
    <span class="release-feature-meta-title">Authors:</span>
    {% for author in release_note.authors %}{% if author is starting_with("@") %}<a href="https://github.com/{{ author | trim_start_matches(pat="@") }}" class="release-feature-meta-item">{{ author }}</a>{% else %}<span class="release-feature-meta-item">{{ author }}</span>{% endif %}{% if not loop.last %},{% endif %}
    {% endfor %}
  </div>
  <div>
    <span class="release-feature-meta-title">PRs:</span>
    {% for pr in release_note.prs %}<a class="release-feature-meta-item" href="https://github.com/bevyengine/bevy/pull/{{ pr }}">#{{ pr }}</a>{% if not loop.last %},{% endif %}
    {% endfor %}
  </div>
</div>

{{ release_note_body | replace(from='POST_PATH', to=page.colocated_path) | markdown }}

{% endfor %}
