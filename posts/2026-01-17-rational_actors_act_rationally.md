Title: Rational Actors Act Rationally
Date: 2026-01-18
Draft: false

---

I could’ve titled this “The Tests Everyone Pretends to Care About” or “The Safety Dance,” but the last paragraph made that feel too narrow.

This article starts as a critique of vanity KPIs, but it’s really an essay about incentives—and the gap between stated and revealed preferences.

## The Tests Nobody Writes

When a measure becomes a target, it ceases to be a good measure.

This is even more true now that LLMs can generate whatever artifacts the metric rewards, with zero marginal cost to the human. Today, you can buy the appearance of rigor for the price of a prompt.

I really, really don't like test coverage mandates.

If your engineers weren’t writing tests last year, and now they’re “catching up” to a lagging coverage number by shipping 5,000 lines before lunch... they’re writing the same amount of tests as before: zero.

Your team is manufacturing coverage.

Don’t blame the coders. The company built a system that rewards appearances, and got them.

## The Tests Nobody Reviews

When everyone knows deep down that the tests they’re being asked to review weren’t written by a human, how much effort do you think goes into reviewing them?

Just like the author had a magic, zero-effort LLM button, the reviewer has one too:

```text
<Copy>
<Paste>
"do these tests make sense?"
```

Machines write the fake tests. Machines review the fake tests.

The organization’s revealed preference—coverage targets hit—gets satisfied. What are we even doing here?

## The Tests That Test Nothing

To make matters worse, the tests that are easiest for LLMs to generate are the least informative. They’re deterministic and shallow, but ultimately hard to object to in review—so they’re what you get.

The front-end team didn’t really write a `LoginButton`. They wrote a wrapper around a wrapper around `Radix UI`. When Cursor automatically adds a test that asserts the click handler doesn’t fire when disabled, it’s not testing anything not already covered by Radix’s test suite.

But... the coverage went up.

And then there are mocked databases: the purest form of testing nothing. Of course the mock returns the value you asked for. Real systems fail on unique constraints, transactions, timeouts, and concurrency—exactly the stuff your mock refuses to model.

But... the coverage went up.

## The Double Whammy

If you really wanted to cripple an engineering team’s productivity, you’d pair test coverage mandates with small-PR mandates.

Once you combine “keep PRs tiny” with “keep coverage high,” you’ve effectively capped how much real change can fit in each pull request—in part because half of your already tiny diff budget is pre-allocated to tests.

A natural 2,500-line PR can be reviewed as one coherent thought.

The same change forced into eight PRs can’t. That’s eight reviews, eight context switches, and eight cycles of rebasing and merge conflicts.

I’d rather review one large, coherent change and ship it atomically than stretch the same work across two weeks of fragmented PRs.

## Safety Dance

The annoying part about this “safety dance” is that there’s no one to blame.

Everyone knows it’s performative. But if playing along is what gets you back to doing real work the fastest, people will play along. Rational actors act rationally.

How many teams without coverage mandates organically choose to stockpile meaningless tests?

## Rational Actors Act Rationally

Leadership needs to internalize a basic fact: every engineer—myself included—has access to the magic “LLM ‘check these boxes’” button. I can keep the coverage number green forever. I can generate tests I didn’t write, approve tests I didn’t read, and nobody will reprimand me for it.

I don’t think there’s much ambiguity about my views.

Two years ago, I’d have kicked and screamed. I’d have threatened to quit over being asked to work with a hand tied behind my back. I’d have demanded room to bend the rules, and I’d have offered my job as collateral if my bold claims didn’t pan out.

But I'm no martyr. Without an explicit mandate from my managers to stir the pot, I will take the implicit mandate from my environment, and do the safety dance.

Rational actors act rationally.
