# Butterfly: Formal Specification

## 1. Functional Decomposition Algorithm

### 1.1 Mathematical Foundation

**Definition 1.1 (LLM as Smooth Map)**
An LLM with parameters θ can be formalized as a smooth map:
```
M_θ : TokenSpace → LatentSpace → TokenSpace
M_θ = decode ∘ transform ∘ encode
```

where:
- `encode : TokenSpace → LatentSpace` (embedding + early layers)
- `transform : LatentSpace → LatentSpace` (middle layers)
- `decode : LatentSpace → TokenSpace` (late layers + unembedding)

**Definition 1.2 (Functional Capability)**
A functional capability C is a measurable subset of the model's behavior characterized by:
```
C ⊂ {(input, output) | output = M_θ(input)}
```

Examples:
- C_logic = {(premise, conclusion) | valid logical inference}
- C_pattern = {(partial_pattern, completion) | pattern recognition}
- C_formal = {(informal_math, formal_math) | formalization}

**Definition 1.3 (Capability Projection)**
For capability C, define projection operator P_C:
```
P_C : M_θ → M_C
```
where M_C maximizes performance on capability C while minimizing parameters.

### 1.2 Splitting Algorithm

**Algorithm: Functional Model Decomposition**

```
Input: Model M_θ, capabilities [C_1, ..., C_k], compute budget B
Output: Worker models [M_1, ..., M_k], fusion function F

1. Capability Analysis Phase:
   For each capability C_i:
     a. Identify activation patterns: analyze which neurons activate for C_i
     b. Compute attention flow: track attention weights during C_i tasks
     c. Measure gradient importance: ∇_θ Loss_C_i reveals critical parameters
     d. Build capability signature: S_i = (activations, attention, gradients)

2. Decomposition Phase:
   For each worker i:
     a. Core layers: Select layers with highest signature S_i overlap
     b. Shared base: Include universal low-level feature extractors
     c. Specialized head: Add task-specific output layers
     d. Prune: Remove parameters with low S_i contribution
     e. Fine-tune: Continue training on C_i-specific data

3. Validation Phase:
   For each worker M_i:
     a. Measure capability retention: accuracy on C_i tasks
     b. Measure independence: correlation(M_i output, M_j output) for i≠j
     c. Ensure diversity: ||M_i(x) - M_j(x)|| > threshold for variety

4. Fusion Function Learning:
   Learn F : [Output_1, ..., Output_k] → FinalOutput
   Subject to:
     - Smoothness: F is C^∞ in output space
     - Monotonicity: Better individual outputs → better fusion
     - Robustness: F(x_1,...,x_k) valid even if x_i fails for some i
```

### 1.3 Metadata Format

**Worker Capability Manifest**
```json
{
  "worker_id": "uuid-v4",
  "base_model": "model-name-version",
  "capabilities": [
    {
      "name": "logical_reasoning",
      "weight": 0.85,
      "validation_accuracy": 0.92,
      "layer_range": [8, 24],
      "attention_patterns": ["global", "causal"],
      "specialized_heads": [
        {"type": "proof_verification", "layers": [22, 23, 24]}
      ]
    }
  ],
  "parameter_count": "7.3B",
  "quantization": "int8",
  "hardware_affinity": ["cuda", "tpu"],
  "latency_profile": {
    "p50": "23ms",
    "p95": "45ms",
    "p99": "78ms"
  }
}
```

**Splitting Configuration**
```json
{
  "strategy": "hybrid_functional",
  "base_model": "llama-3-70b",
  "workers": [
    {
      "worker_id": "worker-logic-001",
      "capability_focus": "logical_reasoning",
      "layer_selection": {
        "shared_base": [0, 1, 2, 3],
        "specialized_core": [12, 13, 14, 15, 16, 17, 18],
        "output_head": [22, 23, 24]
      },
      "fine_tune_dataset": "logical_reasoning_corpus",
      "pruning_threshold": 0.01
    },
    {
      "worker_id": "worker-pattern-001",
      "capability_focus": "pattern_recognition",
      "layer_selection": {
        "shared_base": [0, 1, 2, 3],
        "specialized_core": [4, 5, 6, 7, 8, 9, 10],
        "output_head": [11, 12, 13]
      },
      "fine_tune_dataset": "arc_patterns_corpus",
      "pruning_threshold": 0.01
    },
    {
      "worker_id": "worker-formal-001",
      "capability_focus": "formal_verification",
      "layer_selection": {
        "shared_base": [0, 1, 2, 3],
        "specialized_core": [16, 17, 18, 19, 20, 21],
        "output_head": [23, 24]
      },
      "fine_tune_dataset": "proof_verification_corpus",
      "pruning_threshold": 0.005
    }
  ],
  "fusion_strategy": "sctt_smooth_combination",
  "fusion_hyperparams": {
    "smoothness_order": 2,
    "combination_weights_learnable": true,
    "confidence_weighting": true
  }
}
```

## 2. Smoothness-Preserving Decomposition Theorem

**Theorem 2.1 (Smooth Decomposition Exists)**
Let M : TokenSpace → TokenSpace be a C^∞ smooth map (language model).
Then there exist smooth maps M_1, ..., M_k and a smooth fusion F such that:
```
||M(x) - F(M_1(x), ..., M_k(x))|| < ε
```
for all x in a compact domain, where ε can be made arbitrarily small.

**Proof Sketch:**
1. By Stone-Weierstrass theorem, smooth functions are dense in continuous functions
2. Neural networks are universal approximators for smooth functions
3. Partition of unity allows smooth decomposition into local pieces
4. Each M_i captures one "piece" of M's functionality
5. Smooth fusion F reconstructs the whole via smooth interpolation

**Corollary 2.2 (Capability Orthogonality)**
Workers can be constructed such that their capabilities are approximately orthogonal:
```
<M_i, M_j> / (||M_i|| ||M_j||) < δ for i ≠ j
```
where <·,·> is an appropriate inner product on function space.

## 3. Type-Theoretic Specification

### 3.1 SCTT Formalization

In Smooth Cubical Type Theory, we model:

```
-- Type of all language models
LLM : Type

-- A model is a smooth map
model : (θ : Parameters) → (TokenSpace → TokenSpace)

-- Capability is a predicate on input-output pairs
Capability : Type
Capability = (TokenSpace × TokenSpace) → Prop

-- Worker specialized for capability C
Worker : Capability → Type
Worker C = Σ (M : LLM) , (∀ x y, C(x,y) → high_accuracy(M(x), y))

-- Fusion function that combines workers smoothly
Fusion : (n : Nat) → (Vec (Worker C_i) n) → LLM
Fusion n workers = λ input →
  let outputs := map (λ w → w.model(input)) workers
  in smooth_combine(outputs)

-- Correctness property: fusion preserves all capabilities
fusion_correct : (workers : Vec Worker n) →
  ∀ i, ∀ x y, C_i(x,y) →
    high_accuracy((Fusion n workers)(x), y)
```

### 3.2 Smoothness Requirements

**Definition 3.1 (Smooth Combination)**
The fusion function F must satisfy:
```
F : Product_i (LatentSpace_i) → LatentSpace
```
with the property that F is C^∞ smooth, meaning:
- F is infinitely differentiable
- All partial derivatives are continuous
- Hessian of F is positive semi-definite (convexity)

**Lemma 3.2 (Gradient Flow)**
Under smooth fusion, gradient information flows correctly:
```
∇_θ Loss(F(M_1(x), ..., M_k(x))) =
  Σ_i (∂F/∂M_i) · ∇_θ_i Loss(M_i(x))
```
This ensures end-to-end differentiability for meta-learning.

## 4. Capability Identification Algorithm

### 4.1 Activation Pattern Analysis

**Algorithm: Extract Capability Signatures**

```python
def extract_capability_signature(model, capability_dataset):
    """
    Analyze which parts of the model are critical for a capability

    Returns:
        signature: dict mapping layer_id → importance_score
    """
    signatures = defaultdict(float)

    for batch in capability_dataset:
        # Forward pass with activation collection
        activations = {}
        with torch.no_grad():
            for layer_id, layer in enumerate(model.layers):
                x = layer(x)
                activations[layer_id] = x.detach()

        # Measure activation magnitude (indication of usage)
        for layer_id, act in activations.items():
            signatures[layer_id] += act.abs().mean().item()

    # Normalize by dataset size
    for layer_id in signatures:
        signatures[layer_id] /= len(capability_dataset)

    return signatures

def compute_attention_flow(model, capability_dataset):
    """
    Track how attention flows through the model for this capability
    """
    attention_matrix = torch.zeros(num_layers, num_layers)

    for batch in capability_dataset:
        with torch.no_grad():
            # Collect attention weights at each layer
            attention_weights = []
            for layer in model.layers:
                if hasattr(layer, 'attention'):
                    attn = layer.attention.weights  # [batch, heads, seq, seq]
                    attention_weights.append(attn.mean(dim=(0,1)))

            # Build transition matrix: how much layer i attends to layer j
            for i in range(len(attention_weights) - 1):
                for j in range(i+1, len(attention_weights)):
                    # Information flow from layer i to layer j
                    flow = (attention_weights[i] @ attention_weights[j]).trace()
                    attention_matrix[i,j] += flow

    return attention_matrix / len(capability_dataset)

def gradient_importance(model, capability_dataset, loss_fn):
    """
    Measure which parameters are most important via gradient magnitude
    """
    param_importance = {name: 0.0 for name, _ in model.named_parameters()}

    for batch in capability_dataset:
        model.zero_grad()
        output = model(batch['input'])
        loss = loss_fn(output, batch['target'])
        loss.backward()

        # Accumulate gradient magnitudes
        for name, param in model.named_parameters():
            if param.grad is not None:
                param_importance[name] += param.grad.abs().mean().item()

    # Normalize
    for name in param_importance:
        param_importance[name] /= len(capability_dataset)

    return param_importance
```

### 4.2 Capability Independence Test

**Definition 4.1 (Capability Independence)**
Two capabilities C_i and C_j are independent if:
```
P(M(x) ∈ C_i | M(x) ∈ C_j) = P(M(x) ∈ C_i)
```

**Algorithm: Test Independence**
```python
def test_capability_independence(model, dataset_i, dataset_j):
    """
    Measure how independent two capabilities are in the model

    Returns:
        independence_score: 0 = fully dependent, 1 = fully independent
    """
    # Get activations for both capabilities
    sig_i = extract_capability_signature(model, dataset_i)
    sig_j = extract_capability_signature(model, dataset_j)

    # Convert to vectors
    layers = sorted(set(sig_i.keys()) | set(sig_j.keys()))
    vec_i = torch.tensor([sig_i.get(l, 0) for l in layers])
    vec_j = torch.tensor([sig_j.get(l, 0) for l in layers])

    # Measure correlation
    correlation = (vec_i @ vec_j) / (vec_i.norm() * vec_j.norm())

    # Independence = 1 - |correlation|
    independence = 1 - correlation.abs().item()

    return independence
```

## 5. Pruning and Compression

### 5.1 Magnitude-Based Pruning

**Algorithm: Prune Worker for Capability**
```python
def prune_worker_for_capability(worker, capability_signature, threshold):
    """
    Remove parameters not important for the worker's capability
    """
    param_importance = {}

    # Map layer importance to parameter importance
    for name, param in worker.named_parameters():
        layer_id = extract_layer_id(name)
        importance = capability_signature.get(layer_id, 0)
        param_importance[name] = importance

    # Prune parameters below threshold
    with torch.no_grad():
        for name, param in worker.named_parameters():
            if param_importance[name] < threshold:
                # Create mask for pruning
                mask = (param.abs() > threshold * param.abs().max())
                param.mul_(mask)

    return worker
```

### 5.2 Structured Pruning

**Algorithm: Remove Entire Neurons/Heads**
```python
def structured_prune(worker, capability_signature, target_sparsity):
    """
    Remove entire neurons or attention heads that don't contribute
    """
    # Identify least important structures
    neuron_importance = compute_neuron_importance(worker, capability_signature)

    # Sort by importance
    sorted_neurons = sorted(neuron_importance.items(), key=lambda x: x[1])

    # Calculate how many to remove
    total_neurons = len(sorted_neurons)
    num_to_remove = int(total_neurons * target_sparsity)

    # Remove least important neurons
    neurons_to_remove = [n for n, _ in sorted_neurons[:num_to_remove]]

    # Reconstruct model without those neurons
    pruned_worker = rebuild_model_without_neurons(worker, neurons_to_remove)

    return pruned_worker
```

## 6. Fine-Tuning for Specialization

### 6.1 Capability-Focused Training

**Algorithm: Specialize Worker**
```python
def specialize_worker(worker, capability_dataset, num_epochs):
    """
    Fine-tune worker to excel at its assigned capability
    """
    optimizer = torch.optim.AdamW(worker.parameters(), lr=1e-5)

    for epoch in range(num_epochs):
        for batch in capability_dataset:
            # Standard supervised learning
            output = worker(batch['input'])
            loss = compute_loss(output, batch['target'])

            # Add regularization to prevent forgetting shared knowledge
            if epoch > 0:
                # Distillation from original model
                with torch.no_grad():
                    original_output = original_model(batch['input'])
                distill_loss = kl_divergence(output, original_output)
                loss += 0.1 * distill_loss

            optimizer.zero_grad()
            loss.backward()
            optimizer.step()

    return worker
```

### 6.2 Diversity Regularization

**Loss Function: Encourage Worker Diversity**
```python
def diversity_loss(worker_outputs):
    """
    Penalize workers that produce too similar outputs

    We want workers to explore different parts of solution space
    """
    # Compute pairwise similarities
    similarities = []
    for i in range(len(worker_outputs)):
        for j in range(i+1, len(worker_outputs)):
            sim = cosine_similarity(worker_outputs[i], worker_outputs[j])
            similarities.append(sim)

    # Penalize high similarity
    avg_similarity = sum(similarities) / len(similarities)
    return avg_similarity  # Minimize this

def train_workers_with_diversity(workers, dataset, num_epochs):
    """
    Train all workers simultaneously with diversity regularization
    """
    optimizers = [torch.optim.Adam(w.parameters()) for w in workers]

    for epoch in range(num_epochs):
        for batch in dataset:
            # Forward pass for all workers
            outputs = [w(batch['input']) for w in workers]

            # Individual losses
            losses = [compute_loss(out, batch['target']) for out in outputs]

            # Diversity regularization
            div_loss = diversity_loss(outputs)

            # Combined objective
            total_loss = sum(losses) + 0.1 * div_loss

            # Backward pass
            for optimizer in optimizers:
                optimizer.zero_grad()
            total_loss.backward()
            for optimizer in optimizers:
                optimizer.step()

    return workers
```

## 7. Theoretical Guarantees

### 7.1 Approximation Quality

**Theorem 7.1 (Approximation Bound)**
Let M be the original model and F(M_1, ..., M_k) be the decomposed system.
Then for any input x:
```
||M(x) - F(M_1(x), ..., M_k(x))|| ≤
  Σ_i ||M(x) - M_i(x)|| + ||F - F_optimal||
```

where F_optimal is the best possible fusion function.

**Proof:**
By triangle inequality and properties of smooth interpolation.

### 7.2 Capability Preservation

**Theorem 7.2 (Capability Retention)**
If worker M_i is trained on capability C_i with accuracy α_i, then
the fused system F(M_1, ..., M_k) achieves accuracy on C_i of at least:
```
accuracy_fused(C_i) ≥ w_i * α_i + (1-w_i) * accuracy_ensemble
```
where w_i is the fusion weight for worker i, and accuracy_ensemble
is the base ensemble accuracy.

**Corollary:**
If α_i > accuracy_ensemble, then specialization improves performance.

### 7.3 Computational Complexity

**Theorem 7.3 (Latency Reduction)**
Let T be the sequential inference time for the original model.
With k workers running in parallel, the decomposed system has latency:
```
T_parallel = max_i(T_i) + T_fusion
```
where T_i is the inference time for worker i.

If workers are balanced (T_i ≈ T/k) and fusion is fast (T_fusion << T),
then speedup is approximately k.

**Theorem 7.4 (Communication Complexity)**
The total communication cost for distributed inference is:
```
C_comm = k * |embedding_dim| * |sequence_length| * 2
```
(factor of 2 for input distribution and output collection)

This is O(k) rather than O(k^2), avoiding all-to-all communication.

---

**Status**: Formal specification complete
**Last Updated**: 2025-10-15
**Verification**: Awaiting SCTT type checking
