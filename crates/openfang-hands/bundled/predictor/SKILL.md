---
name: predictor-hand-skill
version: "1.0.0"
description: "Expert knowledge for AI forecasting — superforecasting principles, signal taxonomy, confidence calibration, reasoning chains, and accuracy tracking"
runtime: prompt_only
---

# Forecasting Expert Knowledge

## Superforecasting Principles

Based on research by Philip Tetlock and the Good Judgment Project:

1. **Triage**: Focus on questions that are hard enough to be interesting but not so hard they're unknowable
2. **Break problems apart**: Decompose big questions into smaller, researchable sub-questions (Fermi estimation)
3. **Balance inside and outside views**: Use both specific evidence AND base rates from reference classes
4. **Update incrementally**: Adjust predictions in small steps as new evidence arrives (Bayesian updating)
5. **Look for clashing forces**: Identify factors pulling in opposite directions
6. **Distinguish signal from noise**: Weight signals by their reliability and relevance
7. **Calibrate**: Your 70% predictions should come true ~70% of the time
8. **Post-mortem**: Analyze why predictions went wrong, not just celebrate the right ones
9. **Avoid the narrative trap**: A compelling story is not the same as a likely outcome
10. **Collaborate**: Aggregate views from diverse perspectives

---

## Signal Taxonomy

### Signal Types
| Type | Description | Weight | Example |
|------|-----------|--------|---------|
| Leading indicator | Predicts future movement | High | Job postings surge → company expanding |
| Lagging indicator | Confirms past movement | Medium | Quarterly earnings → business health |
| Base rate | Historical frequency | High | "80% of startups fail within 5 years" |
| Expert opinion | Informed prediction | Medium | Analyst forecast, CEO statement |
| Data point | Factual measurement | High | Revenue figure, user count, benchmark |
| Anomaly | Deviation from pattern | High | Unusual trading volume, sudden hiring freeze |
| Structural change | Systemic shift | Very High | New regulation, technology breakthrough |
| Sentiment shift | Collective mood change | Medium | Media tone change, social media trend |

### Signal Strength Assessment
```
STRONG signal (high predictive value):
  - Multiple independent sources confirm
  - Quantitative data (not just opinions)
  - Leading indicator with historical track record
  - Structural change with clear causal mechanism

MODERATE signal (some predictive value):
  - Single authoritative source
  - Expert opinion from domain specialist
  - Historical pattern that may or may not repeat
  - Lagging indicator (confirms direction)

WEAK signal (limited predictive value):
  - Social media buzz without substance
  - Single anecdote or case study
  - Rumor or unconfirmed report
  - Opinion from non-specialist
```

---

## Confidence Calibration

### Probability Scale
```
95% — Almost certain (would bet 19:1)
90% — Very likely (would bet 9:1)
80% — Likely (would bet 4:1)
70% — Probable (would bet 7:3)
60% — Slightly more likely than not
50% — Toss-up (genuine uncertainty)
40% — Slightly less likely than not
30% — Unlikely (but plausible)
20% — Very unlikely (but possible)
10% — Extremely unlikely
5%  — Almost impossible (but not zero)
```

### Calibration Rules
1. NEVER use 0% or 100% — nothing is absolutely certain
2. If you haven't done research, default to the base rate (outside view)
3. Your first estimate should be the reference class base rate
4. Adjust from the base rate using specific evidence (inside view)
5. Typical adjustment: ±5-15% per strong signal, ±2-5% per moderate signal
6. If your gut says 80% but your analysis says 55%, trust the analysis

### Brier Score
The gold standard for measuring prediction accuracy:
```
Brier Score = (predicted_probability - actual_outcome)^2

actual_outcome = 1 if prediction came true, 0 if not

Perfect score: 0.0 (you're always right with perfect confidence)
Coin flip: 0.25 (saying 50% on everything)
Terrible: 1.0 (100% confident, always wrong)

Good forecaster: < 0.15
Average forecaster: 0.20-0.30
Bad forecaster: > 0.35
```

---

## Domain-Specific Source Guide

### Technology Predictions
| Source Type | Examples | Use For |
|-------------|---------|---------|
| Product roadmaps | GitHub issues, release notes, blog posts | Feature predictions |
| Adoption data | Stack Overflow surveys, NPM downloads, DB-Engines | Technology trends |
| Funding data | Crunchbase, PitchBook, TechCrunch | Startup success/failure |
| Patent filings | Google Patents, USPTO | Innovation direction |
| Job postings | LinkedIn, Indeed, Levels.fyi | Technology demand |
| Benchmark data | TechEmpower, MLPerf, Geekbench | Performance trends |

### Finance Predictions
| Source Type | Examples | Use For |
|-------------|---------|---------|
| Economic data | FRED, BLS, Census | Macro trends |
| Earnings | SEC filings, earnings calls | Company performance |
| Analyst reports | Bloomberg, Reuters, S&P | Market consensus |
| Central bank | Fed minutes, ECB statements | Interest rates, policy |
| Commodity data | EIA, OPEC reports | Energy/commodity prices |
| Sentiment | VIX, put/call ratio, AAII survey | Market mood |

### Geopolitics Predictions
| Source Type | Examples | Use For |
|-------------|---------|---------|
| Official sources | Government statements, UN reports | Policy direction |
| Think tanks | RAND, Brookings, Chatham House | Analysis |
| Election data | Polls, voter registration, 538 | Election outcomes |
| Trade data | WTO, customs data, trade balances | Trade policy |
| Military data | SIPRI, defense budgets, deployments | Conflict risk |
| Diplomatic signals | Ambassador recalls, sanctions, treaties | Relations |

### Climate Predictions
| Source Type | Examples | Use For |
|-------------|---------|---------|
| Scientific data | IPCC, NASA, NOAA | Climate trends |
| Energy data | IEA, EIA, IRENA | Energy transition |
| Policy data | COP agreements, national plans | Regulation |
| Corporate data | CDP disclosures, sustainability reports | Corporate action |
| Technology data | BloombergNEF, patent filings | Clean tech trends |
| Investment data | Green bond issuance, ESG flows | Capital allocation |

---

## Reasoning Chain Construction

### Template
```
PREDICTION: [Specific, falsifiable claim]

1. REFERENCE CLASS (Outside View)
   Base rate: [What % of similar events occur?]
   Reference examples: [3-5 historical analogues]

2. SPECIFIC EVIDENCE (Inside View)
   Signals FOR (+):
   a. [Signal] — strength: [strong/moderate/weak] — adjustment: +X%
   b. [Signal] — strength: [strong/moderate/weak] — adjustment: +X%

   Signals AGAINST (-):
   a. [Signal] — strength: [strong/moderate/weak] — adjustment: -X%
   b. [Signal] — strength: [strong/moderate/weak] — adjustment: -X%

3. SYNTHESIS
   Starting probability (base rate): X%
   Net adjustment: +/-Y%
   Final probability: Z%

4. KEY ASSUMPTIONS
   - [Assumption 1]: If wrong, probability shifts to [W%]
   - [Assumption 2]: If wrong, probability shifts to [V%]

5. RESOLUTION
   Date: [When can this be resolved?]
   Criteria: [Exactly how to determine if correct]
   Data source: [Where to check the outcome]
```

---

## Prediction Tracking & Scoring

### Prediction Ledger Format
```json
{
  "id": "pred_001",
  "created": "2025-01-15",
  "prediction": "OpenAI will release GPT-5 before July 2025",
  "confidence": 0.65,
  "domain": "tech",
  "time_horizon": "2025-07-01",
  "reasoning_chain": "...",
  "key_signals": ["leaked roadmap", "compute scaling", "hiring patterns"],
  "status": "active|resolved|expired",
  "resolution": {
    "date": "2025-06-30",
    "outcome": true,
    "evidence": "Released June 15, 2025",
    "brier_score": 0.1225
  },
  "updates": [
    {"date": "2025-03-01", "new_confidence": 0.75, "reason": "New evidence: leaked demo"}
  ]
}
```

### Accuracy Report Template
```
ACCURACY DASHBOARD
==================
Total predictions:     N
Resolved predictions:  N (N correct, N incorrect, N partial)
Active predictions:    N
Expired (unresolvable):N

Overall accuracy:      X%
Brier score:           0.XX

Calibration:
  Predicted 90%+ → Actual: X% (N predictions)
  Predicted 70-89% → Actual: X% (N predictions)
  Predicted 50-69% → Actual: X% (N predictions)
  Predicted 30-49% → Actual: X% (N predictions)
  Predicted <30% → Actual: X% (N predictions)

Strengths: [domains/types where you perform well]
Weaknesses: [domains/types where you perform poorly]
```

---

## Cognitive Bias Checklist

Before finalizing any prediction, check for these biases:

1. **Anchoring**: Am I fixated on the first number I encountered?
   - Fix: Deliberately consider the base rate before looking at specific evidence

2. **Availability bias**: Am I overweighting recent or memorable events?
   - Fix: Check the actual frequency, not just what comes to mind

3. **Confirmation bias**: Am I only looking for evidence that supports my prediction?
   - Fix: Actively search for contradicting evidence (steel-man the opposite)

4. **Narrative bias**: Am I choosing a prediction because it makes a good story?
   - Fix: Boring predictions are often more accurate

5. **Overconfidence**: Am I too sure?
   - Fix: If you've never been wrong at this confidence level, you're probably overconfident

6. **Scope insensitivity**: Am I treating very different scales the same?
   - Fix: Be specific about magnitudes and timeframes

7. **Recency bias**: Am I extrapolating recent trends too far?
   - Fix: Check longer time horizons and mean reversion patterns

8. **Status quo bias**: Am I defaulting to "nothing will change"?
   - Fix: Consider structural changes that could break the status quo

### Contrarian Mode
When enabled, for each consensus prediction:
1. Identify what the consensus view is
2. Search for evidence the consensus is wrong
3. Consider: "What would have to be true for the opposite to happen?"
4. If credible contrarian evidence exists, include a contrarian prediction
5. Always label contrarian predictions clearly with the consensus for comparison
