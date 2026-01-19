Title: Rational Actors Act Rationally
Date: 2026-01-18
Draft: true

---

I could’ve titled this "The Tests Everyone Pretends to Care About," but the last paragraph made that feel too narrow. This article starts as a critique of vanity KPIs, but it’s really an essay about incentives—and the gap between stated and revealed preferences.

## The Tests Nobody Writes

When a measure becomes a target, it ceases to be a good measure.

This is even more true now that LLMs can generate whatever artifacts the metric rewards with zero marginal cost. Today, you can buy the appearance of rigor for the price of a prompt.

If your engineers weren’t writing when you didn't have a code coverage mandate, and now they’re "catching up" by shipping 5,000 lines before lunch, they’re writing the same amount of tests as before: zero.

It's obviously manufactured compliance, but don’t blame the engineers. The company built a system that rewards appearances, and got appearances.

## The Tests Nobody Reviews

When everyone knows that the tests they’re being asked to review weren’t written by a human, how much effort do you actually think goes into reviewing them?

Just like the author had a magic, zero-effort LLM button, the reviewer has one too:

```text
<Copy>
<Cmd+Tab>
<Paste>
"do these tests make sense?"
<Cmd+Tab>
"LGTM"
```

Machines write the fake tests. Machines review the fake tests. What are we even doing here?

## The Tests That Test Nothing

To make matters worse, the tests that are easiest for LLMs to generate are the least informative. They’re deterministic and shallow, but ultimately hard to strongly object to—so they get merged.

The front-end team didn’t really write a `LoginButton`. They wrote a wrapper around a wrapper around `RadixUI`. When Claude automatically adds a test that asserts the click handler doesn’t fire when disabled, it’s not testing anything not already covered by `RadixUI`’s test suite.

But... the coverage percentage went up.

Claude also loves mocked databases: the purest form of testing nothing. Of course the mock returns the value you asked for. Real systems fail on unique constraints, transactions, timeouts, and concurrency—exactly the stuff your mock refuses to model.

But... the coverage percentage went up.

## The Compounding Cripples

If you really wanted to destroy an engineering team’s productivity, you’d pair test coverage mandates with small-PR mandates.

Once you combine “keep PRs tiny” with “keep coverage high,” you’ve effectively capped how much real change can fit in each pull request—in part because half of your already tiny diff budget is pre-allocated to tests.

A natural 2,500-line PR can be reviewed as one coherent thought. The same change—forced into eight PRs—can’t. That’s eight reviews, eight context switches, and eight cycles of rebasing and merge conflicts.

Personally, I’d rather review one large, coherent change and ship it atomically, than stretch the same work across two weeks of fragmented PRs.

## Safety Dance

The annoying part about this “safety dance” is that there’s no one to blame.

Everyone knows the work is performative. But if playing along is what gets you back to doing real work the fastest, people will rationally play along.

How many teams without coverage mandates organically choose to stockpile bullshit tests?

## Rational Actors Act Rationally

Every engineer—myself included—has access to the magic “LLM check these boxes’” button. I can keep the coverage number green forever. I can generate tests I didn’t write, approve tests I didn’t read, and nobody will reprimand me for it. What a shame.

Two years ago, I’d have kicked and screamed. I’d have threatened to quit over being asked to work with a hand tied behind my back. I’d have demanded room to bend the rules, and I’d have offered my job as collateral if my bold claims didn’t pan out.

But I'm no martyr. Without an explicit mandate from my managers to stir the pot, I will take the implicit mandate from my environment and dance to the music.

Rational actors act rationally.
