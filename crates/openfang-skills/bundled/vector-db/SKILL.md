---
name: vector-db
description: "Vector database expert for embeddings, similarity search, RAG patterns, and indexing strategies"
---
# Vector Database Expert

A retrieval systems specialist with deep expertise in embedding models, vector indexing algorithms, and Retrieval-Augmented Generation (RAG) architectures. This skill provides guidance for designing and operating vector search systems that power semantic search, recommendation engines, and LLM knowledge augmentation, covering embedding selection, indexing strategies, chunking, hybrid search, and production deployment.

## Key Principles

- Choose the embedding model based on your domain and retrieval task; general-purpose models work well for broad use cases, but domain-specific fine-tuned embeddings significantly improve recall for specialized content
- Select the distance metric that matches your embedding model's training objective: cosine similarity for normalized embeddings, dot product for magnitude-aware comparisons, and L2 (Euclidean) for spatial distance
- Chunk documents thoughtfully; chunk size directly impacts retrieval quality because too-large chunks dilute relevance while too-small chunks lose context
- Index choice determines the trade-off between search speed, memory usage, and recall accuracy; understand HNSW, IVF, and flat index characteristics before choosing
- Combine dense vector search with sparse keyword search (hybrid retrieval) for production systems; neither approach alone handles all query types optimally

## Techniques

- Generate embeddings with models like OpenAI text-embedding-3-small, Cohere embed-v3, or open-source sentence-transformers (all-MiniLM-L6-v2, BGE, E5) depending on cost and quality requirements
- Configure HNSW indexes with appropriate M (connections per node, typically 16-64) and efConstruction (build quality, typically 100-200) parameters; higher values improve recall at the cost of memory and build time
- Implement chunking strategies: fixed-size with overlap (e.g., 512 tokens with 50-token overlap), semantic chunking at paragraph or section boundaries, or recursive splitting that respects document structure
- Build hybrid search by executing both vector similarity and BM25/keyword queries, then combining results with Reciprocal Rank Fusion (RRF) or a learned reranker like Cohere Rerank or cross-encoder models
- Filter results using metadata (date ranges, categories, access permissions) at query time; most vector databases support pre-filtering or post-filtering with different performance characteristics
- Design the RAG pipeline: query embedding, retrieval (top-k candidates), optional reranking, context assembly with source citations, and LLM generation with the retrieved context in the prompt

## Common Patterns

- **Parent-Child Retrieval**: Embed small chunks for precise matching but return the larger parent document or section as context to the LLM, preserving surrounding information
- **Multi-vector Representation**: Generate multiple embeddings per document (title, summary, full text) and search across all representations to improve recall for different query styles
- **Contextual Retrieval**: Prepend a document-level summary or metadata to each chunk before embedding so that the vector captures both local content and global context
- **Evaluation Pipeline**: Measure retrieval quality with precision@k, recall@k, and NDCG using a labeled relevance dataset; track these metrics as embedding models and chunking strategies change

## Pitfalls to Avoid

- Do not use a single embedding model for all use cases without benchmarking; embedding quality varies dramatically across domains, languages, and query types
- Do not index documents without preprocessing: remove boilerplate, normalize whitespace, and handle tables and code blocks as structured content rather than raw text
- Do not skip reranking in production RAG systems; initial vector retrieval optimizes for speed, but a cross-encoder reranker significantly improves precision in the final results
- Do not store only vectors without the original text and metadata; you need the source content for LLM context assembly, debugging, and auditing retrieval results
