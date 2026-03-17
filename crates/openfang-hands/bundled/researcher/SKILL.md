---
name: researcher-hand-skill
version: "1.0.0"
description: "Expert knowledge for AI deep research — methodology, source evaluation, search optimization, cross-referencing, synthesis, and citation formats"
runtime: prompt_only
---

# Deep Research Expert Knowledge

## Research Methodology

### Research Process (5 phases)
1. **Define**: Clarify the question, identify what's known vs unknown, set scope
2. **Search**: Systematic multi-strategy search across diverse sources
3. **Evaluate**: Assess source quality, extract relevant data, note limitations
4. **Synthesize**: Combine findings into coherent answer, resolve contradictions
5. **Verify**: Cross-check critical claims, identify remaining uncertainties

### Question Types & Strategies
| Question Type | Strategy | Example |
|--------------|----------|---------|
| Factual | Find authoritative primary source | "What is the population of Tokyo?" |
| Comparative | Multi-source balanced analysis | "React vs Vue for large apps?" |
| Causal | Evidence chain + counterfactuals | "Why did Theranos fail?" |
| Predictive | Trend analysis + expert consensus | "Will quantum computing replace classical?" |
| How-to | Step-by-step from practitioners | "How to set up a Kubernetes cluster?" |
| Survey | Comprehensive landscape mapping | "What are the options for vector databases?" |
| Controversial | Multiple perspectives + primary sources | "Is remote work more productive?" |

### Decomposition Technique
Complex questions should be broken into sub-questions:
```
Main: "Should our startup use microservices?"
Sub-questions:
  1. What are microservices? (definitional)
  2. What are the benefits vs monolith? (comparative)
  3. What team size/stage is appropriate? (contextual)
  4. What are the operational costs? (factual)
  5. What do similar startups use? (case studies)
  6. What are the migration paths? (how-to)
```

---

## CRAAP Source Evaluation Framework

### Currency
- When was it published or last updated?
- Is the information still current for the topic?
- Are the links functional?
- For technology topics: anything >2 years old may be outdated

### Relevance
- Does it directly address your question?
- Who is the intended audience?
- Is the level of detail appropriate?
- Would you cite this in your report?

### Authority
- Who is the author? What are their credentials?
- What institution published this?
- Is there contact information?
- Does the URL domain indicate authority? (.gov, .edu, reputable org)

### Accuracy
- Is the information supported by evidence?
- Has it been reviewed or refereed?
- Can you verify the claims from other sources?
- Are there factual errors, typos, or broken logic?

### Purpose
- Why does this information exist?
- Is it informational, commercial, persuasive, or entertainment?
- Is the bias clear or hidden?
- Does the author/organization benefit from you believing this?

### Scoring
```
A (Authoritative):  Passes all 5 CRAAP criteria
B (Reliable):       Passes 4/5, minor concern on one
C (Useful):         Passes 3/5, use with caveats
D (Weak):           Passes 2/5 or fewer
F (Unreliable):     Fails most criteria, do not cite
```

---

## Search Query Optimization

### Query Construction Techniques

**Exact phrase**: `"specific phrase"` — use for names, quotes, error messages
**Site-specific**: `site:domain.com query` — search within a specific site
**Exclude**: `query -unwanted_term` — remove irrelevant results
**File type**: `filetype:pdf query` — find specific document types
**Recency**: `query after:2024-01-01` — recent results only
**OR operator**: `query (option1 OR option2)` — broaden search
**Wildcard**: `"how to * in python"` — fill-in-the-blank

### Multi-Strategy Search Pattern
For each research question, use at least 3 search strategies:
1. **Direct**: The question as-is
2. **Authoritative**: `site:gov OR site:edu OR site:org [topic]`
3. **Academic**: `[topic] research paper [year]` or `site:arxiv.org [topic]`
4. **Practical**: `[topic] guide` or `[topic] tutorial` or `[topic] how to`
5. **Data**: `[topic] statistics` or `[topic] data [year]`
6. **Contrarian**: `[topic] criticism` or `[topic] problems` or `[topic] myths`

### Source Discovery by Domain
| Domain | Best Sources | Search Pattern |
|--------|-------------|---------------|
| Technology | Official docs, GitHub, Stack Overflow, engineering blogs | `[tech] documentation`, `site:github.com [tech]` |
| Science | PubMed, arXiv, Nature, Science | `site:arxiv.org [topic]`, `[topic] systematic review` |
| Business | SEC filings, industry reports, HBR | `[company] 10-K`, `[industry] report [year]` |
| Medicine | PubMed, WHO, CDC, Cochrane | `site:pubmed.ncbi.nlm.nih.gov [topic]` |
| Legal | Court records, law reviews, statute databases | `[case] ruling`, `[law] analysis` |
| Statistics | Census, BLS, World Bank, OECD | `site:data.worldbank.org [metric]` |
| Current events | Reuters, AP, BBC, primary sources | `[event] statement`, `[event] official` |

---

## Cross-Referencing Techniques

### Verification Levels
```
Level 1: Single source (unverified)
  → Mark as "reported by [source]"

Level 2: Two independent sources agree (corroborated)
  → Mark as "confirmed by multiple sources"

Level 3: Primary source + secondary confirmation (verified)
  → Mark as "verified — primary source: [X]"

Level 4: Expert consensus (well-established)
  → Mark as "widely accepted" or "scientific consensus"
```

### Contradiction Resolution
When sources disagree:
1. Check which source is more authoritative (CRAAP scores)
2. Check which is more recent (newer may have updated info)
3. Check if they're measuring different things (apples vs oranges)
4. Check for known biases or conflicts of interest
5. Present both views with evidence for each
6. State which view the evidence better supports (if clear)
7. If genuinely uncertain, say so — don't force a conclusion

---

## Synthesis Patterns

### Narrative Synthesis
```
The evidence suggests [main finding].

[Source A] found that [finding 1], which is consistent with
[Source B]'s observation that [finding 2]. However, [Source C]
presents a contrasting view: [finding 3].

The weight of evidence favors [conclusion] because [reasoning].
A key limitation is [gap or uncertainty].
```

### Structured Synthesis
```
FINDING 1: [Claim]
  Evidence for: [Source A], [Source B] — [details]
  Evidence against: [Source C] — [details]
  Confidence: [high/medium/low]
  Reasoning: [why the evidence supports this finding]

FINDING 2: [Claim]
  ...
```

### Gap Analysis
After synthesis, explicitly note:
- What questions remain unanswered?
- What data would strengthen the conclusions?
- What are the limitations of the available sources?
- What follow-up research would be valuable?

---

## Citation Formats

### Inline URL
```
According to a 2024 study (https://example.com/study), the effect was significant.
```

### Footnotes
```
According to a 2024 study[1], the effect was significant.

---
[1] https://example.com/study — "Title of Study" by Author, Published Date
```

### Academic (APA)
```
In-text: (Smith, 2024)
Reference: Smith, J. (2024). Title of the article. *Journal Name*, 42(3), 123-145. https://doi.org/10.xxxx
```

For web sources (APA):
```
Author, A. A. (Year, Month Day). Title of page. Site Name. https://url
```

### Numbered References
```
According to recent research [1], the finding was confirmed by independent analysis [2].

## References
1. Author (Year). Title. URL
2. Author (Year). Title. URL
```

---

## Output Templates

### Brief Report
```markdown
# [Question]
**Date**: YYYY-MM-DD | **Sources**: N | **Confidence**: high/medium/low

## Answer
[2-3 paragraph direct answer]

## Key Evidence
- [Finding 1] — [source]
- [Finding 2] — [source]
- [Finding 3] — [source]

## Caveats
- [Limitation or uncertainty]

## Sources
1. [Source](url)
2. [Source](url)
```

### Detailed Report
```markdown
# Research Report: [Question]
**Date**: YYYY-MM-DD | **Depth**: thorough | **Sources Consulted**: N

## Executive Summary
[1 paragraph synthesis]

## Background
[Context needed to understand the findings]

## Methodology
[How the research was conducted, what was searched, how sources were evaluated]

## Findings

### [Sub-question 1]
[Detailed findings with inline citations]

### [Sub-question 2]
[Detailed findings with inline citations]

## Analysis
[Synthesis across findings, patterns identified, implications]

## Contradictions & Open Questions
[Areas of disagreement, gaps in knowledge]

## Confidence Assessment
[Overall confidence level with reasoning]

## Sources
[Full bibliography in chosen citation format]
```

---

## Cognitive Bias in Research

Be aware of these biases during research:

1. **Confirmation bias**: Favoring information that confirms your initial hypothesis
   - Mitigation: Explicitly search for disconfirming evidence

2. **Authority bias**: Over-trusting sources from prestigious institutions
   - Mitigation: Evaluate evidence quality, not just source prestige

3. **Anchoring**: Fixating on the first piece of information found
   - Mitigation: Gather multiple sources before forming conclusions

4. **Selection bias**: Only finding sources that are easy to access
   - Mitigation: Vary search strategies, check non-English sources

5. **Recency bias**: Over-weighting recent publications
   - Mitigation: Include foundational/historical sources when relevant

6. **Framing effect**: Being influenced by how information is presented
   - Mitigation: Look at raw data, not just interpretations

---

## Domain-Specific Research Tips

### Technology Research
- Always check the official documentation first
- Compare documentation version with the latest release
- Stack Overflow answers may be outdated — check the date
- GitHub issues/discussions often have the most current information
- Benchmarks without methodology descriptions are unreliable

### Business Research
- SEC filings (10-K, 10-Q) are the most reliable public company data
- Press releases are marketing — verify claims independently
- Analyst reports may have conflicts of interest — check disclaimers
- Employee reviews (Glassdoor) provide internal perspective but are biased

### Scientific Research
- Systematic reviews and meta-analyses are strongest evidence
- Single studies should not be treated as definitive
- Check if findings have been replicated
- Preprints have not been peer-reviewed — note this caveat
- p-values and effect sizes both matter — not just "statistically significant"
