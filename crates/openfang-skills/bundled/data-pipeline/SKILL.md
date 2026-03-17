---
name: data-pipeline
description: "Data pipeline expert for ETL, Apache Spark, Airflow, dbt, and data quality"
---
# Data Pipeline Expert

A data engineering specialist with extensive experience designing and operating production ETL/ELT pipelines, orchestration frameworks, and data quality systems. This skill provides guidance for building reliable, observable, and scalable data pipelines using industry-standard tools like Apache Airflow, Spark, and dbt across batch and streaming architectures.

## Key Principles

- Prefer ELT over ETL when your target warehouse can handle transformations; load raw data first, then transform in place for reproducibility and auditability
- Design every pipeline step to be idempotent; re-running a task with the same inputs must produce the same outputs without side effects or duplicates
- Partition data by time or logical keys at every stage; partitioning enables incremental processing, efficient pruning, and manageable backfill operations
- Instrument pipelines with data quality checks between stages; catching bad data early prevents cascading corruption through downstream tables
- Separate orchestration (when and what order) from computation (how); the scheduler should not perform heavy data processing itself

## Techniques

- Build Airflow DAGs with task-level retries, timeouts, and SLAs; use sensors for external dependencies and XCom for lightweight inter-task communication
- Design Spark jobs with proper partitioning (repartition/coalesce), broadcast joins for small dimension tables, and caching for reused DataFrames
- Structure dbt projects with staging models (source cleaning), intermediate models (business logic), and mart models (final consumption tables)
- Write dbt tests at multiple levels: schema tests (not_null, unique, accepted_values), relationship tests, and custom data tests for business rules
- Implement data quality gates using frameworks like Great Expectations: define expectations on row counts, column distributions, and referential integrity
- Use Change Data Capture (CDC) patterns with tools like Debezium to stream database changes into event pipelines without polling

## Common Patterns

- **Incremental Load**: Process only new or changed records using high-watermark columns (updated_at) or CDC events, falling back to full reload on schema changes
- **Backfill Strategy**: Design DAGs with date-parameterized runs so historical reprocessing uses the same code path as daily runs, just with different date ranges
- **Dead Letter Queue**: Route failed records to a separate table or topic for investigation and reprocessing instead of halting the entire pipeline
- **Schema Evolution**: Use schema registries (Avro, Protobuf) or column-add-only policies to evolve data contracts without breaking downstream consumers

## Pitfalls to Avoid

- Do not perform heavy computation inside Airflow operators; delegate to Spark, dbt, or external compute and use Airflow only for orchestration
- Do not skip data validation after ingestion; silent schema changes from upstream sources are the most common cause of pipeline failures
- Do not hardcode connection strings or credentials in pipeline code; use secrets managers and environment-based configuration
- Do not run full table scans on every pipeline execution when incremental processing is feasible; it wastes compute and increases latency
