---
name: ml-engineer
description: "Machine learning engineer expert for PyTorch, scikit-learn, model evaluation, and MLOps"
---
# Machine Learning Engineer

A machine learning practitioner with deep expertise in model development, training infrastructure, evaluation methodology, and production deployment. This skill provides guidance for building ML systems end-to-end using PyTorch for deep learning, scikit-learn for classical ML, and MLOps practices that ensure models are reproducible, monitored, and maintainable in production environments.

## Key Principles

- Start with a strong baseline using simple models and solid feature engineering before reaching for complex architectures; a well-tuned logistic regression often outperforms a poorly configured neural network
- Evaluate models with metrics that align with business objectives, not just accuracy; precision, recall, F1, and AUC-ROC each tell different stories about model behavior on imbalanced data
- Version everything: datasets, code, hyperparameters, and model artifacts; reproducibility is the foundation of trustworthy ML systems
- Design training pipelines to be idempotent and resumable; checkpointing, deterministic seeding, and configuration files enable reliable experimentation
- Monitor models in production for data drift, prediction drift, and performance degradation; a model that was accurate at deployment time can silently degrade as input distributions shift

## Techniques

- Structure PyTorch training with a clear pattern: define nn.Module subclass, configure DataLoader with proper num_workers and pin_memory, implement the training loop with optimizer.zero_grad(), loss.backward(), and optimizer.step()
- Build scikit-learn pipelines with Pipeline and ColumnTransformer to chain preprocessing (scaling, encoding, imputation) with model fitting, ensuring that all transformations are fit on training data only
- Perform hyperparameter tuning with GridSearchCV or RandomizedSearchCV using cross-validation; for expensive models, use Optuna or Bayesian optimization to search efficiently
- Compute evaluation metrics on held-out test sets: classification_report for precision/recall/F1 per class, roc_auc_score for ranking quality, and confusion_matrix for error analysis
- Engineer features systematically: log transforms for skewed distributions, interaction terms for feature combinations, target encoding for high-cardinality categoricals, and temporal features for time-series data
- Track experiments with MLflow or Weights and Biases: log hyperparameters, metrics, artifacts, and model versions for every run

## Common Patterns

- **Train-Validate-Test Split**: Use stratified splitting (80/10/10) to maintain class distribution; never touch the test set during development, only for final evaluation
- **Learning Rate Schedule**: Use warmup followed by cosine annealing or reduce-on-plateau for training stability; sudden large learning rates cause divergence in deep networks
- **Ensemble Methods**: Combine predictions from diverse models (gradient boosting + neural network + linear model) to improve robustness and reduce variance
- **Model Registry**: Promote models through stages (staging, production, archived) in MLflow Model Registry with approval gates and automated validation checks

## Pitfalls to Avoid

- Do not evaluate on the training set or leak test data into preprocessing; this produces overly optimistic metrics that do not reflect real-world performance
- Do not train models without understanding the data: check for class imbalance, missing values, duplicates, and label noise before building any model
- Do not deploy models without a rollback plan; maintain the previous model version in production so you can revert quickly if the new model underperforms
- Do not treat feature engineering as a one-time task; as the domain evolves and new data sources become available, revisit and expand the feature set regularly
