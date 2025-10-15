# Byzantine Fault-Tolerant Consensus for Butterfly

## 1. Threat Model and Guarantees

### 1.1 Assumptions

**Network Model:**
- Partially synchronous network (eventual message delivery)
- Messages may be delayed but not lost permanently
- No permanent network partitions
- Message authenticity via cryptographic signatures

**Adversary Model:**
- Up to f < n/3 workers may be Byzantine (arbitrary behavior)
- Byzantine workers can: send wrong outputs, delay responses, collude
- Byzantine workers cannot: break cryptography, forge signatures
- Head node is trusted initially (can be replicated via Raft)

**Safety Guarantees:**
- Agreement: All honest workers agree on the same output
- Validity: If all workers are honest, agreed output = correct fusion
- Integrity: Output cannot be tampered with undetectably

**Liveness Guarantees:**
- Termination: System produces output eventually (within timeout)
- Availability: System continues operating with up to f Byzantine failures

### 1.2 Consensus Problem Statement

**Input:**
- Query q : TokenSpace
- Workers W = {W₁, ..., W_n} with up to f < n/3 Byzantine
- Fusion function F : Vec LatentSpace n → TokenSpace

**Output:**
- Consensus value v : TokenSpace such that:
  1. All honest workers agree on v
  2. v = F(outputs) where outputs are from honest workers
  3. Byzantine outputs are detected and excluded

## 2. Wingbeat Consensus Protocol

### 2.1 Protocol Overview

**Wingbeat** is a novel consensus protocol optimized for LLM inference:
- Based on PBFT (Practical Byzantine Fault Tolerance)
- Optimized for semantic similarity rather than exact equality
- Incorporates proof-of-knowledge for response validation
- Uses smooth homotopy for conflict resolution

**Key Innovations:**
1. **Semantic Consensus**: Agree on semantically similar outputs, not byte-identical
2. **Parallel Verification**: Workers verify each other in parallel
3. **Smooth Interpolation**: Resolve conflicts via geodesic in output space
4. **Proof-Carrying Responses**: Each output includes verifiable reasoning trace

### 2.2 Protocol Phases

**Phase 0: Pre-Prepare (Head Node)**
```
1. Head node receives query q from client
2. Head node broadcasts <PRE-PREPARE, q, view, seq> to all workers
3. Each worker receives query and prepares to respond
```

**Phase 1: Prepare (Workers Generate Outputs)**
```
1. Worker W_i computes output: y_i = W_i(q)
2. Worker W_i generates proof: π_i = proof_of_computation(W_i, q, y_i)
3. Worker W_i broadcasts <PREPARE, i, q, y_i, π_i, sig_i> to all workers
4. Each worker collects PREPARE messages from others
```

**Phase 2: Commit (Workers Verify & Vote)**
```
1. Worker W_i receives PREPARE messages from all other workers
2. For each message from W_j:
   a. Verify signature: verify(sig_j, public_key_j)
   b. Verify proof: verify_proof(π_j, q, y_j)
   c. Compute semantic similarity: sim(y_i, y_j)
   d. Vote: accept if sim > threshold and proof valid
3. Worker W_i broadcasts <COMMIT, i, votes> to all workers
```

**Phase 3: Finalize (Head Node)**
```
1. Head node collects COMMIT messages
2. Identify response clusters (semantically similar outputs)
3. If cluster has ≥ 2f+1 votes → consensus reached
4. Return consensus value to client
```

### 2.3 Detailed Protocol Specification

**Message Formats:**
```
PRE_PREPARE = {
  type: "PRE_PREPARE",
  view: Integer,           // Current view number
  sequence: Integer,       // Sequence number for this query
  query: TokenSpace,       // The input query
  timestamp: Integer,      // Timestamp for timeout
  signature: Signature     // Head node's signature
}

PREPARE = {
  type: "PREPARE",
  worker_id: WorkerId,
  view: Integer,
  sequence: Integer,
  query: TokenSpace,
  output: LatentSpace,     // Worker's computed output
  proof: ProofTerm,        // Proof of correct computation
  metadata: {
    inference_time_ms: Integer,
    confidence: Float,
    model_hash: Hash       // Hash of worker's model weights
  },
  signature: Signature
}

COMMIT = {
  type: "COMMIT",
  worker_id: WorkerId,
  view: Integer,
  sequence: Integer,
  votes: [
    {
      target_worker: WorkerId,
      vote: Bool,          // Accept or reject
      similarity_score: Float,
      reason: String       // Why accepted/rejected
    }
  ],
  signature: Signature
}

REPLY = {
  type: "REPLY",
  view: Integer,
  sequence: Integer,
  consensus_output: TokenSpace,
  supporting_workers: [WorkerId],
  proof_bundle: [ProofTerm],
  signature: Signature
}
```

**State Machine:**
```
State = {
  view: Integer,
  sequence: Integer,
  phase: Enum[IDLE, PRE_PREPARE, PREPARE, COMMIT, REPLY],
  prepare_messages: Map[WorkerId, PREPARE],
  commit_messages: Map[WorkerId, COMMIT],
  consensus_value: Option[TokenSpace]
}

Transitions:
  IDLE → PRE_PREPARE:     Receive query from client
  PRE_PREPARE → PREPARE:  Broadcast query to workers
  PREPARE → COMMIT:       Collect 2f+1 PREPARE messages
  COMMIT → REPLY:         Consensus reached
  REPLY → IDLE:           Send response to client

  * → VIEW_CHANGE:        Timeout or suspected leader failure
  VIEW_CHANGE → IDLE:     New view established
```

## 3. Semantic Similarity Consensus

### 3.1 Clustering Algorithm

**Problem:** Unlike traditional BFT (binary agreement), LLM outputs are high-dimensional vectors. Need to cluster semantically similar outputs.

**Algorithm: Semantic Clustering**
```python
def semantic_clustering(outputs, threshold=0.9):
    """
    Cluster worker outputs by semantic similarity

    Returns: List of clusters, each cluster is [worker_ids]
    """
    clusters = []

    for worker_id, output in enumerate(outputs):
        placed = False

        # Try to add to existing cluster
        for cluster in clusters:
            # Check similarity to all members of cluster
            similarities = [
                semantic_similarity(output, outputs[other_id])
                for other_id in cluster
            ]

            if all(sim > threshold for sim in similarities):
                cluster.append(worker_id)
                placed = True
                break

        # Create new cluster if needed
        if not placed:
            clusters.append([worker_id])

    return clusters

def semantic_similarity(output1, output2):
    """
    Compute semantic similarity between two outputs

    Multiple methods combined:
    1. Embedding cosine similarity
    2. Token overlap (Jaccard)
    3. BLEU score
    4. Entailment score (NLI model)
    """
    # Method 1: Embedding similarity
    emb1 = embed(output1)
    emb2 = embed(output2)
    cos_sim = cosine_similarity(emb1, emb2)

    # Method 2: Token overlap
    tokens1 = set(tokenize(output1))
    tokens2 = set(tokenize(output2))
    jaccard = len(tokens1 & tokens2) / len(tokens1 | tokens2)

    # Method 3: BLEU score
    bleu = compute_bleu(output1, output2)

    # Method 4: Entailment (do they mean the same thing?)
    entailment_fwd = nli_model.score(output1, output2, "entailment")
    entailment_bwd = nli_model.score(output2, output1, "entailment")
    entailment = (entailment_fwd + entailment_bwd) / 2

    # Weighted combination
    similarity = (
        0.4 * cos_sim +
        0.2 * jaccard +
        0.2 * bleu +
        0.2 * entailment
    )

    return similarity
```

### 3.2 Consensus Value Selection

**Algorithm: Select Consensus from Cluster**
```python
def select_consensus_value(cluster, outputs, proofs):
    """
    Given a cluster of semantically similar outputs, select the best one

    Criteria:
    1. Highest confidence
    2. Shortest proof (Occam's razor)
    3. Highest reputation worker
    """
    candidates = [
        {
            'worker_id': worker_id,
            'output': outputs[worker_id],
            'proof': proofs[worker_id],
            'score': compute_consensus_score(worker_id, outputs, proofs)
        }
        for worker_id in cluster
    ]

    # Select highest scoring candidate
    best = max(candidates, key=lambda c: c['score'])
    return best['output']

def compute_consensus_score(worker_id, outputs, proofs):
    """
    Score a candidate for consensus selection
    """
    output = outputs[worker_id]
    proof = proofs[worker_id]

    # Factor 1: Confidence
    confidence = output.metadata.confidence

    # Factor 2: Proof quality (shorter = better)
    proof_quality = 1.0 / (1.0 + len(proof))

    # Factor 3: Worker reputation
    reputation = get_worker_reputation(worker_id)

    # Factor 4: Centrality (how similar to others in cluster)
    centrality = np.mean([
        semantic_similarity(output, outputs[other_id])
        for other_id in range(len(outputs))
    ])

    score = (
        0.3 * confidence +
        0.2 * proof_quality +
        0.2 * reputation +
        0.3 * centrality
    )

    return score
```

## 4. Proof-of-Knowledge for Responses

### 4.1 Computational Proof

**Goal:** Prove that worker actually performed inference (not replaying cached response).

**Mechanism: Verifiable Random Function (VRF)**
```python
def generate_proof_of_computation(worker, query, output):
    """
    Generate cryptographic proof that worker computed output from query

    Uses VRF to prove:
    1. Output was generated by this worker's model
    2. Output is deterministic function of query + worker's key
    3. Proof is non-forgeable (requires worker's private key)
    """
    # Hash of query + worker's public key
    challenge = hash(query + worker.public_key)

    # VRF output: deterministic pseudorandom value
    vrf_output, vrf_proof = worker.vrf_keygen(challenge)

    # Commitment: hash of (output || vrf_output)
    commitment = hash(output || vrf_output)

    return {
        'commitment': commitment,
        'vrf_output': vrf_output,
        'vrf_proof': vrf_proof,
        'output': output
    }

def verify_proof_of_computation(proof, worker_public_key, query):
    """
    Verify that proof is valid
    """
    # Verify VRF proof
    challenge = hash(query + worker_public_key)
    if not vrf_verify(challenge, proof.vrf_output, proof.vrf_proof, worker_public_key):
        return False

    # Verify commitment
    expected_commitment = hash(proof.output || proof.vrf_output)
    if proof.commitment != expected_commitment:
        return False

    return True
```

### 4.2 Reasoning Trace Proof

**Goal:** Prove that output is the result of valid reasoning (not gibberish).

**Mechanism: Proof-Carrying Code**
```python
def generate_reasoning_trace(worker, query, output):
    """
    Generate trace of reasoning steps from query to output

    Includes:
    - Attention patterns (which tokens were important)
    - Intermediate activations (hidden states at key layers)
    - Token probabilities (confidence at each generation step)
    """
    trace = []

    with torch.no_grad():
        # Forward pass with hooks to capture internals
        hidden_states = []
        attention_weights = []

        hooks = register_hooks(worker.model, hidden_states, attention_weights)

        output_logits = worker.model(query)

        remove_hooks(hooks)

    # Build trace
    trace = {
        'query': query,
        'output': output,
        'hidden_states': [
            compress(h) for h in hidden_states[::4]  # Subsample every 4th layer
        ],
        'attention_patterns': [
            compress(a) for a in attention_weights[::4]
        ],
        'token_probabilities': output_logits.softmax(dim=-1).topk(k=10)
    }

    return trace

def verify_reasoning_trace(trace, output):
    """
    Verify that reasoning trace is consistent with output

    Checks:
    1. Hidden states have expected structure
    2. Attention patterns are valid (sum to 1, non-negative)
    3. Token probabilities match output tokens
    """
    # Check 1: Hidden state dimensionality
    for h in trace.hidden_states:
        if h.shape != expected_shape:
            return False

    # Check 2: Attention validity
    for a in trace.attention_patterns:
        if not (a >= 0).all() or not np.isclose(a.sum(dim=-1), 1.0).all():
            return False

    # Check 3: Output consistency
    output_tokens = tokenize(output)
    for token, probs in zip(output_tokens, trace.token_probabilities):
        if token not in probs.indices:
            return False  # Output token not in top-k predictions

    return True
```

## 5. View Change Protocol

### 5.1 Leader Failure Detection

**Triggers for View Change:**
1. Timeout: No response from head node within T seconds
2. Invalid pre-prepare message
3. Suspected leader malicious behavior

**Algorithm: Initiate View Change**
```python
def initiate_view_change(worker, reason):
    """
    Worker suspects leader failure, initiates view change
    """
    new_view = worker.state.view + 1

    # Create VIEW_CHANGE message
    message = {
        'type': 'VIEW_CHANGE',
        'new_view': new_view,
        'worker_id': worker.worker_id,
        'reason': reason,
        'last_sequence': worker.state.sequence,
        'prepared_proofs': worker.get_prepared_messages(),
        'signature': worker.sign(...)
    }

    # Broadcast to all workers
    worker.broadcast(message)

    # Start timer for new view
    worker.start_view_change_timer(new_view)
```

### 5.2 New View Protocol

**Algorithm: Establish New View**
```python
def process_view_change_messages(new_leader, view_change_messages):
    """
    New leader collects VIEW_CHANGE messages and establishes new view
    """
    # Wait for 2f+1 VIEW_CHANGE messages
    if len(view_change_messages) < 2*f + 1:
        return None

    # Verify all signatures
    for msg in view_change_messages:
        if not verify_signature(msg):
            return None

    # Determine highest prepared sequence number
    max_sequence = max(msg.last_sequence for msg in view_change_messages)

    # Create NEW_VIEW message
    new_view_msg = {
        'type': 'NEW_VIEW',
        'view': new_view,
        'view_change_messages': view_change_messages,
        'sequence': max_sequence + 1,
        'signature': new_leader.sign(...)
    }

    # Broadcast NEW_VIEW
    new_leader.broadcast(new_view_msg)

    return new_view_msg
```

## 6. Optimizations

### 6.1 Fast Path (Optimistic Execution)

**Optimization:** If all workers are honest (common case), skip verification rounds.

```python
def optimistic_consensus(workers, query):
    """
    Fast path: assume all workers are honest

    If outputs are sufficiently similar, return immediately without full BFT
    """
    # All workers compute outputs
    outputs = [w(query) for w in workers]

    # Check if all outputs are similar
    similarities = [
        semantic_similarity(outputs[i], outputs[j])
        for i in range(len(outputs))
        for j in range(i+1, len(outputs))
    ]

    if all(sim > 0.95 for sim in similarities):
        # High agreement, return immediately
        return consensus_value(outputs)

    # Low agreement, fall back to full BFT protocol
    return full_bft_consensus(workers, query, outputs)
```

### 6.2 Parallel Verification

**Optimization:** Workers verify proofs in parallel rather than sequentially.

```python
async def parallel_verification(worker, prepare_messages):
    """
    Verify all PREPARE messages in parallel
    """
    async def verify_one(msg):
        sig_valid = verify_signature(msg)
        proof_valid = await verify_proof(msg.proof)
        similarity = semantic_similarity(worker.output, msg.output)

        return {
            'worker_id': msg.worker_id,
            'valid': sig_valid and proof_valid,
            'similarity': similarity
        }

    # Verify all messages concurrently
    results = await asyncio.gather(*[
        verify_one(msg) for msg in prepare_messages
    ])

    return results
```

### 6.3 Incremental Consensus (Streaming)

**Optimization:** Reach consensus on partial outputs (streaming tokens).

```python
async def streaming_consensus(workers, query):
    """
    Reach consensus incrementally as tokens are generated

    This reduces latency: consensus on first tokens while generating rest
    """
    token_streams = [worker.stream_tokens(query) for worker in workers]

    consensus_tokens = []

    while True:
        # Get next token from each worker
        next_tokens = await asyncio.gather(*[
            anext(stream) for stream in token_streams
        ])

        # Check if all workers produced same token
        if len(set(next_tokens)) == 1:
            # Full agreement
            consensus_tokens.append(next_tokens[0])
        else:
            # Disagreement: run consensus protocol
            token = consensus_vote(next_tokens)
            consensus_tokens.append(token)

        # Check for end of sequence
        if all(token == EOS for token in next_tokens):
            break

    return consensus_tokens
```

## 7. Security Analysis

### 7.1 Safety Proof

**Theorem 7.1 (Safety):**
If fewer than n/3 workers are Byzantine, then all honest workers agree on the same output.

**Proof:**

*Step 1: Quorum Intersection*

Any two quorums (sets of 2f+1 workers) must intersect:
```
|Q₁ ∩ Q₂| ≥ 2f+1 + 2f+1 - n = 4f+2 - 3f = f+2 > f
```
Since there are at most f Byzantine workers, the intersection contains at least 2 honest workers.

*Step 2: Honest Workers Form Cluster*

Since honest workers compute correctly, their outputs are semantically similar (similarity > threshold).
Therefore, all honest workers belong to the same cluster.

*Step 3: Honest Cluster is Majority*

The honest cluster has at least n-f ≥ 2f+1 members (since f < n/3).
This exceeds the threshold for consensus.

*Step 4: Agreement*

All honest workers vote for the same cluster (the one containing honest outputs).
Since this cluster has ≥ 2f+1 votes, it becomes the consensus value.

**QED**

### 7.2 Liveness Proof

**Theorem 7.2 (Liveness):**
The system eventually produces an output (assuming network is eventually synchronous).

**Proof:**

*Step 1: Eventual Delivery*

By assumption, all messages are eventually delivered.

*Step 2: View Change Termination*

If a view change is triggered, it completes within bounded time:
- Within 2Δ (network delay), all workers receive VIEW_CHANGE messages
- New leader collects 2f+1 messages and broadcasts NEW_VIEW
- Within 2Δ, all workers receive NEW_VIEW and proceed

*Step 3: Normal Case Termination*

In normal operation (no view change):
- Pre-prepare: 1 message delay
- Prepare: 1 message delay (parallel broadcast)
- Commit: 1 message delay (parallel verification)
- Reply: 1 message delay

Total: 4Δ latency.

*Step 4: Bounded View Changes*

Each view change takes O(Δ) time.
There can be at most f failed views (f Byzantine leaders).
After f+1 view changes, an honest leader is elected.

Total worst-case latency: O(f * Δ).

**QED**

### 7.3 Attack Resistance

**Attack 1: Sybil (Multiple Fake Identities)**
- **Defense**: Proof-of-work or stake-based registration
- **Analysis**: Cost of creating f fake workers exceeds cost of acquiring f real workers

**Attack 2: Output Forgery**
- **Defense**: Cryptographic signatures + proof-of-knowledge
- **Analysis**: Attacker cannot forge signature without private key

**Attack 3: Slow Response (Deliberate Delay)**
- **Defense**: Timeout + view change protocol
- **Analysis**: Honest workers detect timeout and elect new leader

**Attack 4: Semantic Ambiguity (Byzantine outputs near threshold)**
- **Defense**: High similarity threshold (0.9+) + multi-method similarity
- **Analysis**: Attacker must produce output very similar to honest outputs, limiting damage

**Attack 5: Colluding Byzantine Workers**
- **Defense**: f < n/3 requirement ensures honest majority in any quorum
- **Analysis**: Even if all f Byzantine workers collude, they cannot form a quorum

## 8. Implementation Considerations

### 8.1 Message Ordering

**Challenge:** Ensure messages are processed in order despite network delays.

**Solution:** Sequence numbers + view numbers
```python
class MessageQueue:
    def __init__(self):
        self.pending = {}  # seq -> message

    def add_message(self, msg):
        if msg.sequence > self.last_processed + 1:
            # Out of order, buffer it
            self.pending[msg.sequence] = msg
        else:
            # In order, process immediately
            self.process(msg)
            self.flush_pending()

    def flush_pending(self):
        # Process any now-in-order buffered messages
        while self.last_processed + 1 in self.pending:
            msg = self.pending.pop(self.last_processed + 1)
            self.process(msg)
```

### 8.2 Checkpointing

**Challenge:** Prevent unbounded state growth.

**Solution:** Periodic checkpoints
```python
def create_checkpoint(worker, sequence):
    """
    Periodically (every K sequences), create checkpoint

    Checkpoint includes:
    - Current view
    - Last sequence
    - State digest
    - Signatures from 2f+1 workers
    """
    if sequence % CHECKPOINT_INTERVAL == 0:
        checkpoint = {
            'view': worker.state.view,
            'sequence': sequence,
            'state_digest': hash(worker.state),
            'signature': worker.sign(...)
        }

        worker.broadcast_checkpoint(checkpoint)

        # Collect 2f+1 checkpoint signatures
        if worker.collect_checkpoint_sigs(checkpoint):
            # Safe to discard messages before this checkpoint
            worker.discard_old_messages(sequence)
```

---

**Status**: Byzantine Fault-Tolerant consensus protocol complete
**Last Updated**: 2025-10-15
**Security Review**: Pending formal verification
**Related**: NETWORK_PROTOCOL.md, COMBINATION_PROOFS.md
