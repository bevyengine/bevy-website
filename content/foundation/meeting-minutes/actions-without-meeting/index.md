+++
title = "Actions by the Board without a Meeting"
date = 2024-09-16
authors = ["Alice I. Cecile"]
[extra]
github = "alice-i-cecile"
+++

<!-- more -->

Not all actions taken by the board require a formal meeting!
For the sake of transparency though, we've recorded the actions of the board performed without a meeting here, in chronological order.
Screenshots are provided for each decision, although for feasibility reasons only the final vote tally is recorded in them (only thumbs up / thumbs down reactions count).

The language here is often informal, but as Secretary I've captured it directly for maximal transparency and accuracy.
I've used `[]` brackets to fill in missing context to quotes when it was natural, and have added a `Context` section where it is not.

Only actions where a vote occurs is recorded, and all such votes will be reported (along with the votes of each board member), regardless of if an initiative passes.
Generally speaking, the Bevy maintainers only act "as the board" when handling foundation business, and vote on matters of relatively high importance or those involving finances.
Ordinary moderation decisions don't rise to the level of the board, and technical discussion is handled in the open.

The board acts with a majority vote, and a recorded "unanimous" vote means that all board members at the time voted the same way with no absences or abstentions.

## 2024-04-15: Using Thera as our Employer of Record

![Using Thera as our Employer of Record](thera_employer_of_record.png)

**Context:** In order to readily employee folks from across the world, the Bevy Foundation uses an [employer of record](https://www.oysterhr.com/glossary/eor) to handle local compliance, taxation and benefits.

**Proposal:** Are we good to move forward with onboarding \[with thera\], given that \[the cost will be $400/month, $200 cheaper than that of Deel\]?

**Proposed by:** Carter Anderson

**Votes:** Yes from Rob Swain, Carter Anderson, Alice I. Cecile, François Mockers. No vote from James Liu.

## 2024-04-15: Initial job titles

![Initial job titles](initial_job_titles.png)

**Proposal:** All in favor of Project Lead (me once we hire me), Staff Engineer (soon to be Alice + future maintainer hires), and Principal for SME hires?

**Proposed by:** Carter Anderson

**Votes:** Unanamious yes

## 2024-04-18: Usage of the Bevy logo by the unofficial Bevy Playground

![Usage of the Bevy logo by the unofficial Bevy Playground](bevy_playground_logo_usage.png)

**Proposal:**  Thumbs up to this message to vote yes to approving [use of the logo](https://discordapp.com/channels/691052431525675048/745355529777315850/1230635309906137098) for [Bevy Playground](https://learnbevy.com/playground).

**Proposed by:** Alice I. Cecile

**Votes:** Yes by Carter Anderson, Alice I. Cecile, and Rob Swain. No vote from James Liu and Francois.

## 2024-05-02: Sick leave documentation policy

![Sick leave documentation policy](sick_leave_documentation_policy.png)

**Proposal:** Should Bevy Foundation employees require documentation for sick leave?

**Proposed by:** Carter Anderson![alt text](image.png)

**Votes:** No from Rob Swain, Carter Anderson, Alice I. Cecile, James Liu. No vote from François Mockers.

## 2024-06-16: Repaying Cart for initial deposit into bank account

![Repaying Cart for initial deposit into bank account](repaying_cart_initial_deposit.png)

**Context:** The Bevy bank account needed funds in it to be opened. Cart lent the Foundation $2000 USD from his personal account to cover this.

**Proposal:**  I do also think we're in a good enough spot reserves-wise (12k USD) for me to reclaim the 2k I added at the beginning. Are we ok with that?

**Proposed by:** Carter Anderson

**Votes:** Yes by Alice I. Cecile, Rob Swain and François Mockers. No vote from James Liu. Carter Anderson abstained.

## 2024-07-08: Increasing Alice's salary

![Increasing Alice's salary](increasing_alice_salary.png)

**Context:**

> Cart: Alrighty it is definitely time to bump Alice's pay. Consider this a start of one of our "async chat-driven board meetings". Here is my proposal, given the current numbers.
>
> First, lets look at our current numbers:
> Our current net volume (aka what we take home after stripe fees, credit card fees, failed payments / disputes, etc) for a 4 week period on Stripe is 7,884 USD
> We have 15,932 USD in our account (with the next monthly Thera charge coming up on the 11th)
> Alice's current monthly "gross" income is 3,000 CAD
> We are charged 2,796 USD each month, which translates to about 3,813 CAD. The 813 CAD difference includes the 400 USD (545.65 CAD) Thera EOR fee, plus additional canadian employer taxes.
> Alice currently makes 675 USD from Github Sponsors
> I currently make 4245 USD from Github Sponsors.
> Given the current 7,884 USD, I think we should leave ~800 USD of wiggle room each month to account for fluctuations. That (in combination with our current 15,932 USD buffer) seems more than safe enough. That means we have 7,884 - 800 = 7,084 USD to distribute each month.
> Lets talk "auto-balance algorithm":
>
> On the topic of taxes: I think it is fair to say that taxes are a cost that is "paid" by the receiver of the funds, given that they pay into public benefits that the receiver benefits from. So when calculating auto-balance, it makes sense to do that pre-tax.
On the topic of Thera fees: I think this cost should be split evenly across those getting paid, as the choice to incorporate in the United States was arbitrary and people outside of the united states should not be penalized for it. Put another way, this is a cost "paid by the Foundation prior to making payouts".
>
> > So if we were to start auto-balancing now, this would be the calculation (in USD):
>
> $7,884 (currently monthly net income) - $800 (buffer) = $7,084
> $7,084 - $400 (current Thera EOR) = $6,684
> $6,684 / 2 = $3,342
> $4,245 (current Cart Github Sponsors) + $3,342 = $7,587 (Cart's pre auto balance total)
> $675 (current Alice Github Sponsors) + $3,342 = $4,017 (Alice's pre auto balance total)
> $7,587 - $4,017 = $3,570 (pre-auto balance difference)
> $3,570 / 2 = $1,785 (amount to redistribute to Alice)
> $4,017 + $1,785 = $5,802 (Alice's post-auto-balance total)
> $7,587 - $1,785 = $5,802 (Cart's post-auto-balance total)
> $5,802 - $675 = $5,127 (Bevy Foundation payment to Alice)
> $5,802 - $4,245 = $1,557 (Bevy Foundation payment to Cart)
> $5,127 + $1,557 = $6,684 (proof that this adds up to the funds allocated for this by the foundation)
>
> From there, we need to sort out Alice's target "gross" income, based on all Canadian fees:
>
> Based on the current numbers:
> 3,000 CAD gross income
> 2,796.62 USD monthly cost to foundation
> 2,796.62 USD - 400 USD (EOR cost) = 2,396.62 USD (cost to foundation minus Thera EOR)
> 2,396.62 USD = 3,269.65 CAD at time of writing
> 3,269.65 CAD / 3,000 CAD = 1.089883333 (canadian overhead multiplier)
>
>
> 5,127 USD ~= 6,994.51 CAD at the time of writing
> 6,994.51 CAD / 1.089883333 (canadian overhead multiplier) = 6,417.67 CAD (Alice monthly gross)
>
> Note that I am still not quite prepared on the "legal" side to take my cut, but I think we should set Alice's payment according to this rate, as I'm not certain how possible lowering monthly income arbitrarily is. We can sort out what to do with the extra funds later (ex: pay out as "fixup" bonuses, or we can discuss alternatives).

**Proposal:** So the proposal is: increase Alice's monthly payments to $6,417.67 CAD ($4,705.28 USD) as soon as Thera can make it possible. Vote on this comment with 👍  or 👎 to officially approve this proposal. Feel free to discuss first if you want to amend the proposal / notice any discrepancies.

**Votes:** Alice I. Cecile abstains. All others voted yes.

## 2024-07-16: Deprecating legacy sponsors

![Deprecating legacy sponsors](deprecating_legacy_sponsors.png)

**Proposal:** Also: I think it is time to deprecate the "legacy sponsors" section. I've prepped [a branch](https://github.com/bevyengine/bevy-website/pull/1564) but I won't create the PR until we agree

**Proposed by:** Carter Anderson

**Votes:** Yes by Alice I. Cecile, Rob Swain and François Mockers. No vote from Carter Anderson or James Liu.

## 2024-09-09: More relaxed voting rules

![More relaxed voting rules](more_relaxed_voting_rules.png)

**Proposal:** We should change "(b) an email transmission that is sent with sufficient information to determine the sender’s identity"" in section 5.16 of the Bevy Foundation Bylaws to "(b) an email transmission with sufficient information to determine the sender's identity or a Discord message or vote in an official Bevy channel or private chat where all maintainers are present." At the time of determining the results of a vote, "edits" to digital messages are considered invalid and the Secretary should take screenshots of the results to include in the meeting minutes.

**Proposed by:** Carter Anderson

**Votes:** Yes by Carter Anderson, Alice I. Cecile, Rob Swain and François Mockers. No vote by James Liu.

## 2024-09-09: Recording who voted for which motions in the minutes

![Recording who voted for which motions in the minutes](recording_who_voted.png)

**Proposal:** We should change "The names of the persons who were present for discussions and votes relating to the transaction or arrangement, the content of the discussion, including any alternatives to the proposed transaction or arrangement, and a record of any votes taken in connection with the proceedings." in section 8.5 of the Bevy Foundation Bylaws to "The names of the persons who were present for discussions and votes relating to the transaction or arrangement, the content of the discussion, including any alternatives to the proposed transaction or arrangement, and a record of any votes taken in connection with the proceedings. When a decision is not unanimous, the name of each voting board member and the content of their vote must be recorded in the minutes."

**Proposed by:** Carter Anderson

**Votes:** Yes by Carter Anderson, Alice I. Cecile, Rob Swain and François Mockers. No vote by James Liu.
