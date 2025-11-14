+++
title = "AI Policy"
insert_anchor_links = "right"
[extra]
weight = 1
+++

In recent times, there have been a growing number of contributions that are
fully or partially produced by generative AI (e.g. large language models and
friends) which exhibit characteristics that result in undue extra work for other
contributors and maintainers. While we've seen PRs and issues with these
characteristics produced entirely by humans, generative AI tools have
significantly lowered the level of effort required to produce
"plausibly-worthwhile" contributions that are otherwise entirely unmergeable or
incorrectly report bugs, and so have become a major source of burdensome PRs and
issues.

Whether AI generated code is subject to copyright protection is also a
hot-button legal topic that is still being openly debated and litigated. How
this impacts the legal aspects of maintaining a FOSS project is currently an
unresolved question.

This policy is established as a response targeted at the problem of an
increasing frequency of burdensome PRs/issues and to address the potential legal
issues currently surrounding the intersection of AI generated code and the FOSS
contribution model.

## AI Generated Communications

The unsolicited use of automated systems to communicate issues, bugs, or
security vulnerabilities about Bevy Organization projects under the guise of a
human is considered unacceptable and a Code of Conduct violation. Any individual
contributor, operator of automated systems, or company they may represent may be
barred from future contributions and banned from regular communication channels,
especially if these communications were found to be submitted in bad faith.

This policy applies to all regular channels of communication used by members of
the Bevy Organization, including but not limited to GitHub Issues, GitHub Pull
Requests, Discord, other social media platforms, etc.

We recognize that English may not be the primarily language for all contributors
and that machine translation is an indispensable tool for proper collaboration.
Therefore machine translation is not subject to the above policy. The community
recommends that you instruct the LLM to produce a concise output or use non-LLM
machine translation options, as they tend to be less verbose while still getting
the point across.

## AI Generated Contributions and Copyright

At the current time of writing (August 11th, 2025), the US Copyright Office has
[stated publicly][us-copyright-office-response] that "human authorship is a
pre-requisite to copyright protection". A
[more recent report][us-copyright-office-report] from the same institution shows
a much more contested legal space, both within the US and internationally.
Unanswered open questions in the space include, but are not limited to:

- In the case that AI generated works are protected under copyright, would AI
  generated works be considered derivatives of any input to the model, including
  but not limited to: the model's training dataset, the dataset used for fine
  tuning the model, any data fetched during retrieval augmented generation
  (RAG), or extra context provided to the model in the prompt?
- If AI generated works are considered derivative works, do the FOSS licenses
  currently in use by the Bevy Organization have the language and legal
  framework to provide the same guarantees and protections to both the licensor
  and licensee?
- In the case that AI generated works are protected under copyright, who owns
  the copyright to the generated work? Is it the user that requested the
  generation? The owner of the LLM model or service? Who holds the rights to
  license out the generated work for use in open source projects? Is the
  copyright transferable through the same legal framework that exists for works
  that were not AI generated?
- If there is a minimum threshold of human contribution to a combined work
  derived from AI-generated works for it to be considered copyrightable, where
  does that threshold lie, and is it consistently applicable to all types of
  contributions that Bevy Organization accepts?
- Does the local law in various countries and jurisdictions around the world
  provide consistent answers to all of the questions above?

Until there are well established answers to these questions, the use and/or
distribution of AI-generated code and assets may constitute copyright
infringement or may be subject to licensing terms incompatible with the FOSS
licenses used by the Bevy Organization.

Erring on the side of caution in light of a openly debated legal topic, all[^1]
forms of AI-generated contributions cannot be merged into repositories
maintained by the Bevy Organization. This includes both code and non-code game
assets (e.g. textures, audio, etc).

Any triage team member suspecting a pull request to be made primarily through
the use of large language models or other generative tools should mark the PR as
`S-Nominated-to-Close` , upon which a maintainer can then review the PR for
closure. To help identify these cases, pull requests subject to this policy have
characteristics such as (but not limited to):

- Needlessly or overly verbose descriptions or responses.
- Not internally coherent or even self-contradictory.
- Demonstrates misunderstanding of important aspects of what the code is doing
  or the purpose of the change.

Any contributor, operator of automated systems, or company they may represent
found to have repeatedly submitted contributions with majority AI-generated code
or assets may be subject to:

- Blanket rejection of all future contributions to Bevy Organization projects.
- Retroactive removal of any potentially suspect AI-generated code and asset
  contributions.
- Further Code of Conduct actions if these contributions were found to be
  submitted in bad faith.

This policy may be revisited when the legal debate has settled.

\[^1\]: Trivial LLM generated content such as variable renames or autocompleted
function calls, often branded "predictions" or "suggestions", that is otherwise
indistinguishable from traditional methods such as a regex search/replace or an
LSP autocompletion is by definition not detectable and can be treated like other
regular IDE tools such as Intellisense. This does not include cases where the
prediction generates things like entire function blocks.

[us-copyright-office-report]: https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-2-Copyrightability-Report.pdf
[us-copyright-office-response]: https://www.copyright.gov/rulings-filings/review-board/docs/a-recent-entrance-to-paradise.pdf
