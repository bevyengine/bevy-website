+++
title = "Donate"
template = "donate-options.html"
[extra]
header_message = "Donate"
+++

## Should I donate through Every.org or Stripe?

The short answer is "use Every.org if you can". Prefer paying directly through your bank if you want to avoid payment processor fees.

[Every.org](https://every.org/bevy-foundation), like Bevy Foundation, is a non-profit 501(c)(3) public charity, and they only work with other charities.
They are our preferred donation platform because they charge _no fees_, other than the baseline payment provider fees. This is literally as good as it can get.
If you pay with your bank account, 100% of your donation goes to the Bevy Foundation. If you use one of their other payment methods (credit card, PayPal, Google Pay, Apple Pay, etc),
then they charge the baseline fees that those providers charge.

Every.org also supports custom one-time donations, so if you want to do one of those, you should use Every.org.

Stripe does also cut us a deal as a non profit for card transactions (we pay 2.2% + 30 cents instead of 2.9% + 30 cents). However for bank transactions, they charge 0.8%,
and for _all monthly subscriptions_ they add another 0.5% charge (soon to be 0.7%). These fees add up, so only use Stripe if you have to, and prefer paying with your bank account!

## Why can't I use Every.org for Bronze membership?

Every.org has a minimum payment of $10, so we can't offer Bronze through them. This is why the Bronze link on the donate page defaults to Stripe instead of Every.org.

## I donated at a level that lets me add my name / link / logo to the credits. How do I do that?

You have three options:

1. If you donated via Every.org and opted for a public donation, your name will be included automatically. If you set the relevant form values on Stripe, the name and link will be used. If you donated at a level that includes a logo, or you would like to change any of the names / link values that have already been set, see the next two options.
2. Reach out to <support@bevy.org> with the values you want and we will add them. Please provide your name and donation tier to help us find your donation info.
3. You can [set them yourself here](https://github.com/bevyengine/bevy-donors) by submitting a pull request. See the "Adding/Updating Donor Info" section of the readme.

## Can I make a one-time donation instead of monthly?

Yup! Every.org supports this. Click the "custom" link in the table above and select "one time".

## How do I cancel my donation subscription?

If you paid through Every.org, log in to their website and cancel through their portal. You can find that here: <https://www.every.org/my-giving>.

If you paid through Stripe (currently the only way to set up a donation subscription), just [visit this link](https://billing.stripe.com/p/login/7sI3ee5OXbI7dgIaEE) and enter the email you used to set up the subscription.

## I want a donation receipt, how do I get one?

For both platforms (Every.org and Stripe), you will get emailed donation receipts after successful donations.

## I donated at a tier that adds me to the credits, can I use whatever name / logo / link I want?

We reserve the right to refuse to list any name / link / logo for any reason. If it is obnoxious, offensive, rude, or a violation of our [Code of Conduct](https://github.com/bevyengine/bevy/blob/main/CODE_OF_CONDUCT.md), expect us to refuse to list it.

Please only use names, logos, and links that you have the right and permission to use.

## I sponsored but the metrics / names / logos on the donate page haven't updated!

Please wait a little. It might take up to eight hours for the automatic update to add you to the credits.

## How are the metrics on the donation page calculated?

The monthly dollar amount is the "raw pledged amount for active monthly donations". This means two things:

1. This number is "pre payment processor and platform fees". This includes credit card fees and Stripes "processing fees", which vary per payment type.
2. This is the amount we _expect_ to get each month. In practice the payouts will be less, once chargebacks and cancellations are processed.
