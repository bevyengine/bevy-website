{%-if not language-%}
  {%-set language = "rs"-%}
{%-endif-%}
{%-set code = load_data(path="learning-code-examples/examples/" ~ file)-%}
{%-if example-%}
  {%-set code_lines = code | split(pat="\n")-%}
  {%-set code = ""-%}
  {%-set in_example = false-%}
  {%-set hidden_line = false-%}
  {%-for line in code_lines-%}
    {%-if line is ending_with("// END_EXAMPLE: " ~ example)-%}
      {%-set_global in_example = false-%}
    {%-endif-%}
    {%-if line is ending_with("// HIDE")-%}
      {%-set_global hidden_line = true-%}
    {%-else-%}
      {%-set_global hidden_line = false-%}
    {%-endif-%}
    {%-if in_example and not hidden_line-%}
    {%-set_global code = code ~ line ~ "
"-%}
    {%-endif-%}
    {%-if line is ending_with("// EXAMPLE: " ~ example)-%}
      {%-set_global in_example = true-%}
    {%-endif-%}
  {%-endfor-%}
{%-endif-%}
```{{language}}
{{code-}}
```
