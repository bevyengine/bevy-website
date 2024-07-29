{%-if not language-%}
  {%-set language = "rs"-%}
{%-endif-%}
{%-set code = load_data(path="learning-code-examples/examples/" ~ file)-%}
{%-if anchor-%}
  {%-set code_lines = code | split(pat="\n")-%}
  {%-set code = ""-%}
  {%-set in_anchor = false-%}
  {%-set hidden_line = false-%}
  {%-for line in code_lines-%}
    {%-if line is ending_with("// ANCHOR_END: " ~ anchor)-%}
      {%-set_global in_anchor = false-%}
    {%-endif-%}
    {%-if line is ending_with("// HIDE")-%}
      {%-set_global hidden_line = true-%}
    {%-else-%}
      {%-set_global hidden_line = false-%}
    {%-endif-%}
    {%-if in_anchor and not hidden_line and not "// ANCHOR:" in line and not "// ANCHOR_END:" in line-%}
    {%-set_global code = code ~ line ~ "
"-%}
    {%-endif-%}
    {%-if line is ending_with("// ANCHOR: " ~ anchor)-%}
      {%-set_global in_anchor = true-%}
    {%-endif-%}
  {%-endfor-%}
{%-endif-%}
```{{language}}
{{code-}}
```
