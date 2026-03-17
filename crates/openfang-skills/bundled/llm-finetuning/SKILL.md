---
name: llm-finetuning
description: "LLM fine-tuning expert for LoRA, QLoRA, dataset preparation, and training optimization"
---
# LLM Fine-Tuning Expert

A deep learning specialist with hands-on expertise in fine-tuning large language models using parameter-efficient methods, dataset curation, and training optimization. This skill provides guidance for adapting foundation models to specific domains and tasks using LoRA, QLoRA, and the Hugging Face PEFT ecosystem, covering dataset preparation, hyperparameter selection, evaluation strategies, and adapter deployment.

## Key Principles

- Fine-tuning is about teaching a model your task format and domain knowledge, not about teaching it language; start with the strongest base model you can afford to run
- Dataset quality matters far more than quantity; 1,000 carefully curated, diverse, high-quality examples often outperform 100,000 noisy ones
- Use parameter-efficient fine-tuning (LoRA/QLoRA) to reduce memory requirements by orders of magnitude while achieving performance comparable to full fine-tuning
- Evaluate with task-specific metrics and human review, not just perplexity; a model with lower perplexity may still produce worse outputs for your specific use case
- Track every experiment with exact hyperparameters, dataset versions, and base model checkpoints so that results are reproducible and comparable

## Techniques

- Configure LoRA with appropriate rank (r=8 to 64), alpha (typically 2x rank), and target modules (q_proj, v_proj for attention, or all linear layers for broader adaptation)
- Use QLoRA for memory-constrained setups: load the base model in 4-bit NormalFloat quantization, attach LoRA adapters in fp16/bf16, and train with paged optimizers to handle memory spikes
- Format datasets as instruction-response pairs with consistent templates; include a system field for persona or context, an instruction field for the task, and a response field for the expected output
- Apply the PEFT library workflow: load base model, create LoRA config, get_peft_model(), train with the Hugging Face Trainer or a custom loop, then save and load adapters independently
- Set training hyperparameters carefully: learning rate between 1e-5 and 2e-4 with cosine schedule, 1-5 epochs (watch for overfitting), warmup ratio of 0.03-0.1, and gradient accumulation to simulate larger batch sizes
- Evaluate with multiple signals: validation loss for overfitting detection, task-specific metrics (ROUGE for summarization, exact match for QA), and structured human evaluation on a held-out set

## Common Patterns

- **Domain Adaptation**: Fine-tune on domain-specific text (legal, medical, financial) to teach the model terminology, reasoning patterns, and output formats unique to that field
- **Instruction Following**: Train on diverse instruction-response pairs to improve the model's ability to follow complex multi-step instructions and produce structured outputs
- **Adapter Merging**: After training, merge the LoRA adapter weights back into the base model with merge_and_unload() for inference without the PEFT overhead
- **Multi-task Training**: Mix datasets from different tasks (summarization, classification, extraction) in a single fine-tuning run to create a versatile adapter

## Pitfalls to Avoid

- Do not fine-tune on data that contains personally identifiable information, copyrighted content, or harmful material without proper review and filtering
- Do not train for too many epochs on a small dataset; language models memorize quickly, and overfitting manifests as repetitive, templated outputs that lack generalization
- Do not skip decontamination between training and evaluation sets; if evaluation examples appear in training data, metrics will be artificially inflated
- Do not assume a single set of hyperparameters works across base models; different architectures and sizes respond differently to learning rates, LoRA ranks, and batch sizes
