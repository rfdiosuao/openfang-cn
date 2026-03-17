---
name: elasticsearch
description: "Elasticsearch expert for queries, mappings, aggregations, index management, and cluster operations"
---
# Elasticsearch Expert

A search and analytics specialist with deep expertise in Elasticsearch cluster architecture, query DSL, mapping design, and performance optimization. This skill provides production-grade guidance for building search experiences, log analytics pipelines, and time-series data platforms using the Elastic stack.

## Key Principles

- Design mappings explicitly before indexing data; relying on dynamic mapping leads to field type conflicts and bloated indices
- Understand the difference between keyword fields (exact match, aggregations, sorting) and text fields (full-text search with analyzers)
- Use index aliases for zero-downtime reindexing, canary deployments, and time-based index rotation
- Size shards between 10-50 GB for optimal performance; too many small shards waste overhead, too few large shards limit parallelism
- Monitor cluster health (green/yellow/red) continuously and investigate yellow status immediately, as it indicates unassigned replica shards

## Techniques

- Construct bool queries with must (scored AND), filter (unscored AND), should (OR with minimum_should_match), and must_not (exclusion) clauses
- Use match queries for full-text search with analyzer-aware tokenization, and term queries for exact keyword lookups without analysis
- Build aggregations: terms for top-N cardinality, date_histogram for time bucketing, nested for sub-document analysis, and pipeline aggs like cumulative_sum
- Apply Index Lifecycle Management (ILM) policies with hot/warm/cold/delete phases to automate rollover and data retention
- Reindex with POST _reindex using source/dest, applying scripts for field transformations during migration
- Check cluster allocation with GET _cluster/allocation/explain to diagnose why shards remain unassigned
- Tune search performance with the search profiler API, request caching, and pre-warming for frequently used queries

## Common Patterns

- **Search-as-you-type**: Use the search_as_you_type field type or edge_ngram tokenizer with a match_phrase_prefix query for autocomplete experiences
- **Parent-Child Relationships**: Use join field types for one-to-many relationships where child documents update independently, avoiding costly nested reindexing
- **Cross-cluster Search**: Configure remote clusters and use cluster:index syntax to query across multiple Elasticsearch deployments transparently
- **Snapshot and Restore**: Register a snapshot repository (S3, GCS, or filesystem) and schedule regular snapshots for disaster recovery with SLM policies

## Pitfalls to Avoid

- Do not use wildcard queries on text fields with leading wildcards, as they bypass the inverted index and cause full field scans
- Do not index large documents (over 100 MB) without splitting them; they cause memory pressure during indexing and merging
- Do not set number_of_replicas to 0 in production; replicas provide both search throughput and data redundancy
- Do not update mappings on existing indices for incompatible type changes; create a new index with the correct mapping and reindex the data
