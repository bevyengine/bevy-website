+++
title = "AI Policy"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Bevy, as a project and a community, values:

1. **Community:** Collaboration and camaraderie between contributors
2. **Growth:** Developing understanding and wisdom within our contributors
3. **Quality:** Creating a reliable, high-quality foundation for others to build on

Our policy around AI use is designed pragmatically,
attempting to find ways of working together that will, in practice, honor these values.

In concrete terms, Bevy has the following norms around AI use that we expect our contributors and community members to uphold.
Some of these are firm boundaries, while others describe the standard of craft that we expect from our contributors:

1. **Ownership:** Contributors are wholly responsible for their own submissions, regardless of how they are created. 
  You must personally understand every line of code and documentation that you submit.
  Contributions must be original (or appropriately credited and license-compliant), well-made and crafted with care: something that you are proud of.
  Do not include "Co-authored by Claude" etc commits; these will be caught by CI and automatically rejected.
2. **Disclosure:** AI use, regardless of extent, must be disclosed, typically in the PR description, with context about how AI was used.
  This gives reviewers the information they need to evaluate both the output and your personal expertise effectively, and guide contributors to use these tools in a constructive way.
3. **Learning:** We want to help our contributors learn and grow.
  Work within the limits of your skills, and do not allow AI to substitute for understanding or thinking.
  Verify each finding yourself, carefully and critically evaluate plans and designs, and test things rigorously, both automatically and manually.
  Never serve as a direct pipe for an LLM: we want to talk to *you*, not to Claude-via-proxy!
4. **Large changes:** Please do not prepare or submit sweeping changes (by number of files touched, PRs opened or total lines of code) without prior discussion, even though AI makes doing so easier.
  Doing so will typically result in your PR being closed or ignored.
  These are a lot of work to review, attract merge conflicts quickly, and risk wasting resources for nothing.
  Just because you *could* generate a major feature in a single 10k LoC PR does not mean you *should* skip the design and consensus-building process!
5. **Collaboration:** Do not use AI tools to supercede or accelerate others' active work without their explicit consent.
  Check for open efforts before embarking on new major features, and prefer collaborating to improve that work, 
  with the understanding that experimenting with alternative approaches is often an important part of collaboration.
6. **Refinement:** Do not dump raw generative AI output on other contributors: this is disrespectful of their time and attention.
  Aggressively review and revise generated code before submission, working carefully to refine the design and polish the implementation.
  For written communication, seek to understand the output and synthesize it from scratch in your own words.
  If you want to share exact output in a discussion, quote sparingly, with attribution.
7. **Quality:** The Bevy project is willing to take the time to do things well.
  Do not simply pressure reviewers, authors or maintainers to increase velocity.
  Sit down, understand where the bottlenecks currently are, and offer tangible and humane suggestions to improve how we work.
  Don't use AI tools as a shortcut, or suggest that others do so.
8. **Writing:** Everyone is sick of reading LLM-flavored text, no matter their feelings on its use.
  AI should never be used to draft large blocks of public-facing prose: module docs, release notes, lengthy PR descriptions, or book chapters, to give a few examples.
  We would rather you submit something simple that you wrote yourself; our editors can help expand and revise it later.
  Using AI as a critic or for mechanical fixes is permitted, but substantive revisions should be made yourself to preserve both structure and voice.
9. **Design:** Writing is thinking.
  While AI can be helpful during the design process, it should be used to explore the space, not to simply generate a solution.
  Publicly shared design documents (or problem definitions) must be personally authored, synthesizing your own understanding of the problem space and any proposed solution.
  We want to foster genuine expertise, and feel that taking the time to draft plans personally is critical for both learning and refinement. Care and effort spent planning pays dividends later.
10. **Art:** Out of respect for the craft of artists, AI must not be used for creating or manipulating art assets submitted to Bevy: find a permissively licensed asset, commission something appropriate, or make one by hand.
11. **Harassment:** Do not harass, pressure, or demean others because of their AI use or lack thereof.
  See the attached Appendix for examples of acceptable and unacceptable behavior.
  There are a wide range of perspectives within our community, and community health is more important than winning arguments on the internet or accomplishing any particular bit of work.
12. **Refusal:** Contributors are allowed to refuse to review AI-assisted code for any reason, or reject AI-assisted reviews. Please, do not pressure them to change their minds.
13. **Translation:** LLMs may be used for translation, but you must provide the original untranslated text as well
  (preferably in a [collapsed section]).
  We want to understand when we're talking to a human vs an LLM, and translations which destroy your natural voice without an original to cross-reference are indistinguishable from text that was simply generated.
14. **Good faith:** Welcome others to the project and to the community, and presume that they are contributing in good faith.
  If and when others violate these rules, start by gently correcting them; guiding them so that they can follow our community norms.

Individual reviewers are empowered to request changes to contributions that fail to live up to these norms, or in cases where the work is not salvageable (commonly due to a lack of understanding by the original author), close them completely.
Bevy is excited to mentor new contributors, and help you develop the skills you need to contribute, but we need you to engage with us directly, as people, not just serve as a proxy for an LLM.
Repeated or severe violations may result in warnings, suspensions, or bans: please try your best to respect and uphold these norms.

[collapsed section]: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/organizing-information-with-collapsed-sections

## Appendix: Harassment Policy Details

We understand that AI is a controversial topic, with important societal implications, many of which directly impact topics the Bevy community cares about.
Nevertheless, Bevy's [Code of Conduct] continues to apply when AI-related topics are discussed.
To help you understand where Bevy draws the line, the following AI-related behaviors are not acceptable anywhere within the Bevy community:

- inflammatory comments about AI or patterns of behavior that seek to get a rise out of others
- comments that imply that AI use is or will be mandatory (inside of Bevy or outside of it), or which otherwise disparage those who do not want to use it / think it should be used
- insistently advocating for the use of AI, in general or with respect to specific practices / tools, in either Bevy's development or others' personal workflows
  - note that discussion of personal AI-related tools and practices should be contained to the `#machine-learning` channel on Discord to ensure others can easily avoid these discussions.
  - engine-related conversation on adoption or abandonment of AI workflows should be similarly threaded.
- refusal to disengage from an argument with a specific individual after being asked to stop
- broad negative generalizations about groups of people based on their views on or usage of or abstinence from AI
- unsubstantive negative comments or reactions in response to contributors or community members who disclose AI use

If you have frustrations or concerns with other community members' behavior or project-wide rules, come to the moderation team.
We will try our best to resolve problems, even if it is a repeated pattern of subthreshold behavior!
Once our decision has been made, do your best to respect it unless new problems have arisen.

The following behaviors are permissible, when handled with social grace:

- discussing the politics and current events of AI
- discussing, in the general case, the role of AI in art
- criticizing products, companies, and public figures use of or stance on AI
- discussing best practices for using (or avoiding) AI
- discussing, in tangible ways, how Bevy's development practices should change to incorporate, avoid or respond to AI
- specific feedback about flaws in a given piece of work
- constructive feedback about recurring problems in the way that others work or act 
- criticizing decisions, regardless of their nature, by Bevy leadership

Exercise judgement and do not fixate exclusively on these topics: the Bevy community is not a debate platform, and, per the [Code of Conduct], doing so disruptively or unkindly *will* result in moderation action.

[Code of Conduct]: https://bevy.org/learn/contribute/policies/code-of-conduct/

## Appendix: History and Rationale

While there is substantial diversity in opinion, Bevy's maintainers and contributors hold, on the whole, fairly skeptical and nuanced opinions about both the societal effects of AI and its technical merit within development workflows.
If you are new to the Bevy community, you might ask: why not simply ban AI contributions and be done with it?

In brief: [we tried that](https://github.com/bevyengine/bevy-website/pull/2204).
Motivated by concerns around quality, ethics and legal risk, the Bevy maintainers voted to ban all AI-generated contributions.
It didn't go very well.

In practice, we found that we could not fairly and reliably enforce or adjudicate this policy.
AI-generated prose is *sometimes* detectable, but rarely to the level of evidence desirable.
AI-generated code is even less so.
AI-generated bug detection, research, or design input could be *completely* invisible,
and while this was not barred, many community members were unhappy with such use.

Submissions were made that were *maybe* AI-generated with no way to ever prove it one way or another,
forcing reviewers and moderators to constantly decide to act on or overlook violations on the slimmest of evidence.
Any accusation threatened complete rejection of submitted work regardless of quality or effort,
with a looming possibility of formal and informal exclusion from the community.
Reviewers were caught between unproven, disruptive suspicion and blithe naïveté,
accepting "no Mom I didn't eat any cookies" at face value because the cookie jar is stored in a locked room.

This was deeply unpleasant: either choice was damaging to the community in the event of a mistake,
and evaluating these meta-questions around provenance was emotionally exhausting.
It did not, contrary to our initial goals, save maintainers time!

This was, unsurprisingly, stressful and contentious!
The day-to-day details of "how does this policy actually function in practice" matter.
The simple cases of "new contributor, AI disclosed, code bad" were easily dealt with,
but we were left without a settled answer to the socially complex problems buried in the edge cases.

Over time, we found that what we were punishing was *disclosure*, not *use*.
Without reliable detection, there was a strong incentive to defect: silently use AI, don't tell anyone about it.
Even if no one *took* it, the temptation weighed on our contributors (who wants to do tedious refactors by hand?),
and those who felt strongly that Bevy should not allow AI-assisted contributions worried about what might be slipping by.

While looking for an alternative, we studied and discussed dozens of other AI policies in open source.
The policies of [Blender], [Godot], [Rust], [Mastodon] and [GCC] were particularly useful as inspiration,
and encourage others seeking to write their own policy to start their research there.
While we do not follow any of those policies exactly, we've integrated meaningful elements from each,
and are grateful for the hard work done to draft and pass those policies.

Ultimately, the changes to Bevy's AI policy were not driven by a desire to "10x productivity" or "embrace the future",
but out of a need to make review and moderation a better, more humane experience, fostering transparency and psychological safety.
Our policy is a pragmatic reflection of our values and collective experience:
establishing shared norms that allow us to work together healthily to build a better Bevy.

For *extensive* discussion about this policy from contributors, maintainers and community members, and more information about how and why this decision was made,
please see [the PR that introduced this policy](https://github.com/bevyengine/bevy-website/pull/2551).

[Blender]: https://projects.blender.org/blender/blender-developer-docs/commit/c7c4280a41bba631c42c6247429bf9c80216c090
[Godot]: https://godotengine.org/article/contribution-policy-2026/
[Rust]: https://github.com/rust-lang/rust-forge/commit/392e02626777c7388fb8532bee956f1361559d0b
[Mastodon]: https://github.com/mastodon/.github/commit/4eb2a1c3cb7caca9789d2aa9446a105bf1fae801
[GCC]: https://forge.sourceware.org/redi/gcc-wwwdocs/commit/4d0793a6a14bf9bfe9e92ac1599840780355199d
