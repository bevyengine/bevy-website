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

We are broadly wary of AI's use in the development of Bevy,
but tolerate careful human-driven AI use for code consistent with the above values.

In concrete terms, Bevy expects its community members to uphold the following norms around AI:

1. **No AI-generated media.** Images, 3d models, audio assets (and so on) submitted to Bevy may not be authored or modified with the use of generative AI tools. Use a permissively licensed asset, make one by hand, or commission an artist.
2. **No AI-generated prose.** AI cannot be used to write public-facing prose, including documentation, issues, PR descriptions, or release notes.
3. **No AI-generated communication.** We want to talk to *you*, not a machine. Do not paste AI-generated output directly into conversations with humans, no matter where they take place. AI may be used to assist for translation and accessibility purposes, but please include the original untranslated text as well.
4. **No AI-authored commits.** This includes "Co-authored by" and is enforced via automated checks in CI.
5. **Ownership.** As a contributor, you are personally responsible for everything you submit. You must personally understand every line of code or documentation that you put forward for review, and be able to articulate the design rationale and implementation tradeoffs when asked. Work within your skill level and knowledge of the project.
6. **Disclosure.** Any use of AI to contribute to Bevy must be disclosed, with context about how it was used.
7. **Quality.** Bevy holds a high bar for quality, and is willing to take the time to do things well. AI-assisted PRs will be held to a rigorous standard of review: correctness, tests, rationale, incremental reviewable units, appropriate design and consensus building and so on. 
8. **Harassment.** Do not harass, demean, pressure, or bully others. This continues to apply, even when the basis of your disagreement centers around use of, abstinence from or views on AI. See the [appendix on harassment](#appendix-harassment-policy-details) for examples of acceptable and unacceptable behavior.
9. **Refusal.** Bevy contributors and community members are not required to review AI-assisted work, use AI tools, or engage in discussions about AI. We want to make sure that those who reject AI remain a valued part of both the work and community of Bevy.

Individual reviewers are empowered to request changes to contributions that fail to live up to these norms, or in cases where the work is not salvageable (commonly due to a lack of understanding by the original author), close them completely.
Bevy is excited to mentor new contributors, and help you develop the skills you need to contribute, but we need you to engage with us directly, as people, not just serve as a proxy for a machine.
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
  - engine-related conversation on adoption or abandonment of AI workflows should similarly be threaded.
- refusal to disengage from an argument with a specific individual after being asked to stop
- broad negative generalizations about groups of people based on their views on or usage of or abstinence from AI
- participating in or condoning harassment campaigns
- unconstructive negative comments or reactions in response to contributors or community members who disclose AI use

If you have frustrations or concerns with other community members' behavior or project-wide rules, come to the moderation team.
We will try our best to resolve problems, even if it is a repeated pattern of subthreshold behavior!
Once our decision has been made, do your best to respect it unless new problems have arisen.

The following behaviors are permissible, when handled with social grace and at the appropriate channels:

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
