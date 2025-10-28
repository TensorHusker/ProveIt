# Butterfly Network Protocols

## 1. Network Architecture

### 1.1 Topology

**Gossip-Based Mesh Network**
```
                    ┌─────────────┐
                    │  Head Node  │
                    │ (Orchestr.) │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼────┐        ┌────▼────┐        ┌────▼────┐
   │Worker 1 │◄──────►│Worker 2 │◄──────►│Worker 3 │
   │(Logic)  │        │(Pattern)│        │(Formal) │
   └────┬────┘        └────┬────┘        └────┬────┘
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                    ┌──────▼──────┐
                    │Verification │
                    │   Layer     │
                    └─────────────┘
```

**Properties:**
- **Star Topology**: Head node coordinates workers (low latency)
- **Peer Mesh**: Workers can communicate directly (fault tolerance)
- **Gossip Layer**: Workers share state updates via epidemic protocol
- **Verification Ring**: Workers form verification consensus ring

### 1.2 Communication Patterns

**Pattern 1: Scatter-Gather (Primary)**
```
1. Client → Head: Query
2. Head → Workers: Broadcast query
3. Workers → Head: Stream results
4. Head → Client: Fused response
```

**Pattern 2: Worker-to-Worker Debate**
```
1. Worker A → Worker B: "My answer is X because..."
2. Worker B → Worker A: "I disagree because..."
3. Workers → Head: Consensus or divergence report
```

**Pattern 3: Gossip State Synchronization**
```
Every T seconds:
  Each worker sends state summary to random peers
  Peers reconcile state differences
  Eventual consistency achieved
```

## 2. Message Format Specification

### 2.1 Base Protocol (Binary, Zero-Copy)

**Message Header (32 bytes)**
```
Offset | Size | Field              | Description
-------|------|--------------------|-----------------------
0      | 4    | magic              | 0x42555454 ("BUTT")
4      | 2    | version            | Protocol version (0x0001)
6      | 2    | message_type       | Type enum (see below)
8      | 8    | message_id         | UUID (64-bit hash)
16     | 8    | timestamp_ns       | Nanosecond timestamp
24     | 4    | payload_length     | Length of payload in bytes
28     | 2    | compression        | None=0, Zstd=1, LZ4=2
30     | 2    | checksum           | CRC16 of payload
```

**Message Types**
```
0x0001: QUERY_REQUEST
0x0002: QUERY_RESPONSE
0x0003: WORKER_REGISTRATION
0x0004: HEARTBEAT
0x0005: GOSSIP_STATE_UPDATE
0x0006: DEBATE_PROPOSAL
0x0007: DEBATE_CRITIQUE
0x0008: CONSENSUS_VOTE
0x0009: VERIFICATION_REQUEST
0x000A: VERIFICATION_RESPONSE
0x000B: ERROR_REPORT
0x000C: METRICS_UPDATE
```

### 2.2 Query Messages

**QUERY_REQUEST Payload**
```json
{
  "query_id": "uuid-v4",
  "input_text": "What is the proof that sqrt(2) is irrational?",
  "input_tokens": [token_ids],
  "context": {
    "conversation_history": [...],
    "user_preferences": {
      "formality": "academic",
      "detail_level": "high"
    }
  },
  "routing_hint": {
    "preferred_capabilities": ["logical_reasoning", "formal_verification"],
    "exclude_workers": [],
    "timeout_ms": 5000
  },
  "streaming": true
}
```

**QUERY_RESPONSE Payload**
```json
{
  "query_id": "uuid-v4",
  "worker_id": "worker-logic-001",
  "response": {
    "text": "The proof proceeds by contradiction...",
    "tokens": [token_ids],
    "logits": [...],  // Optional: raw logits for fusion
    "hidden_states": [...],  // Optional: last layer activations
    "attention_weights": [...]  // Optional: for interpretability
  },
  "metadata": {
    "inference_time_ms": 23,
    "tokens_generated": 157,
    "confidence_score": 0.94,
    "capability_used": "logical_reasoning"
  },
  "streaming_chunk": {
    "chunk_index": 5,
    "is_final": false
  }
}
```

### 2.3 Worker Registration

**WORKER_REGISTRATION Payload**
```json
{
  "worker_id": "uuid-v4",
  "worker_name": "worker-logic-001",
  "capabilities": [
    {
      "name": "logical_reasoning",
      "confidence": 0.95,
      "benchmark_scores": {
        "arc_challenge": 0.87,
        "math_qa": 0.91,
        "proof_verification": 0.94
      }
    }
  ],
  "hardware": {
    "device_type": "cuda",
    "gpu_model": "A100",
    "memory_gb": 80,
    "compute_capability": "8.0"
  },
  "network": {
    "address": "10.0.1.23:8000",
    "protocol": "grpc",
    "tls_enabled": true,
    "certificate_fingerprint": "sha256:..."
  },
  "load": {
    "current_queries": 3,
    "max_concurrent": 16,
    "avg_latency_ms": 25
  }
}
```

### 2.4 Gossip Protocol

**GOSSIP_STATE_UPDATE Payload**
```json
{
  "gossip_round": 12045,
  "sender_id": "worker-2",
  "state_vector": {
    "worker-1": {"version": 128, "hash": "abc123"},
    "worker-2": {"version": 131, "hash": "def456"},
    "worker-3": {"version": 127, "hash": "ghi789"}
  },
  "deltas": [
    {
      "worker_id": "worker-2",
      "updates": [
        {"key": "load.current_queries", "value": 5, "timestamp": 1697234567890},
        {"key": "metrics.avg_latency_ms", "value": 28, "timestamp": 1697234567891}
      ]
    }
  ],
  "request_sync": ["worker-3"]  // Request full state from worker-3
}
```

### 2.5 Debate Protocol

**DEBATE_PROPOSAL Payload**
```json
{
  "debate_id": "uuid-v4",
  "query_id": "uuid-v4",
  "round": 1,
  "proposer": "worker-logic-001",
  "proposal": {
    "answer": "The proof requires assuming sqrt(2) = p/q in lowest terms...",
    "reasoning": [
      "Step 1: Assume sqrt(2) = p/q where gcd(p,q) = 1",
      "Step 2: Square both sides: 2 = p²/q²",
      "Step 3: Therefore p² = 2q²",
      "Step 4: This means p² is even, so p is even",
      "Step 5: Let p = 2k, then (2k)² = 2q²",
      "Step 6: 4k² = 2q², so 2k² = q²",
      "Step 7: Therefore q² is even, so q is even",
      "Step 8: But if p and q are both even, gcd(p,q) ≥ 2",
      "Step 9: This contradicts our assumption. QED."
    ],
    "confidence": 0.96,
    "formal_proof": "..." // Optional: SCTT proof term
  }
}
```

**DEBATE_CRITIQUE Payload**
```json
{
  "debate_id": "uuid-v4",
  "query_id": "uuid-v4",
  "round": 1,
  "critic": "worker-formal-001",
  "target_proposal": "worker-logic-001",
  "critique": {
    "agreement_score": 0.92,
    "disagreements": [
      {
        "step": 4,
        "issue": "Justification for 'p² even implies p even' not explicit",
        "severity": "minor",
        "suggestion": "Add lemma: If n² is even, then n is even"
      }
    ],
    "improvements": [
      "Formalize in type theory for machine verification",
      "Add constructive proof variant"
    ],
    "overall_assessment": "Correct but could be more rigorous"
  }
}
```

### 2.6 Consensus Protocol

**CONSENSUS_VOTE Payload**
```json
{
  "consensus_id": "uuid-v4",
  "query_id": "uuid-v4",
  "voter": "worker-pattern-001",
  "vote": {
    "preferred_answer": "worker-logic-001",
    "justification": "Most rigorous formal structure",
    "alternative_rankings": [
      {"worker": "worker-formal-001", "score": 0.89},
      {"worker": "worker-logic-001", "score": 0.96},
      {"worker": "worker-pattern-001", "score": 0.78}
    ]
  },
  "signature": "ed25519:...",  // Cryptographic signature for BFT
  "timestamp_ns": 1697234567890123456
}
```

## 3. Transport Layer

### 3.1 Protocol Stack

**Layer 4: Application (Butterfly Protocol)**
- Message serialization (Protocol Buffers or Cap'n Proto)
- Compression (Zstd for cold data, LZ4 for hot path)
- Encryption (TLS 1.3 mandatory)

**Layer 3: RPC Framework (gRPC)**
- Bidirectional streaming for low-latency
- HTTP/2 multiplexing
- Automatic retry and load balancing

**Layer 2: Transport (QUIC/TCP)**
- Primary: QUIC for low latency, 0-RTT reconnection
- Fallback: TCP for firewall compatibility
- UDP multicast for gossip (local network only)

**Layer 1: Network (IPv6 preferred, IPv4 compat)**

### 3.2 gRPC Service Definition

```protobuf
syntax = "proto3";

package butterfly;

// Main orchestration service (Head Node)
service ButterflyOrchestrator {
  // Client submits query, streams back results
  rpc Query(QueryRequest) returns (stream QueryResponse);

  // Worker registers with head node
  rpc RegisterWorker(WorkerRegistration) returns (RegistrationAck);

  // Heartbeat to maintain worker liveness
  rpc Heartbeat(HeartbeatRequest) returns (HeartbeatResponse);

  // Get system status
  rpc GetStatus(StatusRequest) returns (StatusResponse);
}

// Worker-to-worker communication
service ButterflyWorker {
  // Forward subquery to another worker
  rpc ForwardQuery(QueryRequest) returns (QueryResponse);

  // Debate protocol
  rpc ProposeAnswer(DebateProposal) returns (DebateCritiqueStream);
  rpc CritiqueAnswer(DebateCritique) returns (Empty);

  // Gossip state synchronization
  rpc GossipUpdate(GossipStateUpdate) returns (GossipAck);
  rpc RequestState(StateRequest) returns (StateResponse);
}

// Formal verification service
service ButterflyVerifier {
  // Verify a proof using SCTT
  rpc VerifyProof(VerificationRequest) returns (VerificationResponse);

  // Check combination correctness
  rpc VerifyFusion(FusionVerificationRequest) returns (FusionVerificationResponse);
}

message QueryRequest {
  string query_id = 1;
  string input_text = 2;
  repeated int32 input_tokens = 3;
  Context context = 4;
  RoutingHint routing_hint = 5;
  bool streaming = 6;
}

message QueryResponse {
  string query_id = 1;
  string worker_id = 2;
  Response response = 3;
  Metadata metadata = 4;
  StreamingChunk streaming_chunk = 5;
}

// ... (additional message definitions)
```

## 4. Network Optimization

### 4.1 Compression Strategy

**Token Compression**
- Tokens are integers, use variable-length encoding (VarInt)
- Consecutive tokens often form patterns, use run-length encoding
- Expected compression ratio: 40-60%

**Logits Compression**
- Float32 logits have high precision but low entropy
- Quantize to Float16 or BFloat16 (50% size reduction)
- Sparse top-k logits: only send top 100 candidates (95% size reduction)
- Expected compression ratio: 90-95%

**Hidden States Compression**
- Apply PCA to reduce dimensionality
- Quantize to Int8 with learned scale factors
- Expected compression ratio: 75%

### 4.2 Batching Strategy

**Dynamic Micro-Batching**
```python
class MicroBatcher:
    def __init__(self, max_batch_size=32, max_wait_ms=10):
        self.max_batch_size = max_batch_size
        self.max_wait_ms = max_wait_ms
        self.buffer = []
        self.timer = None

    async def add_request(self, request):
        self.buffer.append(request)

        # Start timer on first request
        if len(self.buffer) == 1:
            self.timer = asyncio.create_task(
                self.wait_and_flush()
            )

        # Flush immediately if batch is full
        if len(self.buffer) >= self.max_batch_size:
            await self.flush()

    async def wait_and_flush(self):
        await asyncio.sleep(self.max_wait_ms / 1000)
        await self.flush()

    async def flush(self):
        if not self.buffer:
            return

        batch = self.buffer
        self.buffer = []

        # Process batch in parallel
        results = await process_batch(batch)

        # Return results to individual requests
        for request, result in zip(batch, results):
            request.set_result(result)
```

### 4.3 Connection Pooling

**Pool Configuration**
```python
connection_pool_config = {
    'min_connections': 2,          # Keep warm for low latency
    'max_connections': 16,         # Limit concurrent requests
    'idle_timeout_seconds': 300,   # Close idle after 5 min
    'max_lifetime_seconds': 3600,  # Rotate connections hourly
    'connection_test_query': 'PING',
    'retry_strategy': {
        'max_attempts': 3,
        'backoff': 'exponential',
        'initial_delay_ms': 10,
        'max_delay_ms': 1000
    }
}
```

## 5. Byzantine Fault Tolerance

### 5.1 Threat Model

**Assumptions:**
- Up to f < n/3 workers may be Byzantine (malicious or faulty)
- Network may experience delays but not permanent partitions
- Head node is trusted (can be replicated with Raft/Paxos)

**Attacks to Defend Against:**
1. **Malicious Responses**: Worker returns incorrect answers
2. **Slowloris**: Worker deliberately delays responses
3. **Sybil**: Attacker creates multiple fake worker identities
4. **Eclipse**: Attacker isolates victim from honest workers

### 5.2 Defense Mechanisms

**Mechanism 1: Cryptographic Signatures**
```python
class SignedResponse:
    def __init__(self, worker_id, response, private_key):
        self.worker_id = worker_id
        self.response = response
        self.timestamp = time.time_ns()

        # Sign: hash(worker_id || response || timestamp)
        message = f"{worker_id}|{response}|{self.timestamp}".encode()
        self.signature = private_key.sign(message)

    def verify(self, public_key):
        message = f"{self.worker_id}|{self.response}|{self.timestamp}".encode()
        try:
            public_key.verify(self.signature, message)
            return True
        except Exception:
            return False
```

**Mechanism 2: Response Verification**
```python
def verify_worker_response(query, response, worker_id):
    """
    Check if response is plausible for the query

    Uses multiple heuristics:
    1. Semantic similarity to expected answer
    2. Consistency with other workers
    3. Format validation
    4. Timing analysis (too fast = suspicious)
    """
    checks = []

    # Check 1: Response format is valid
    checks.append(validate_format(response))

    # Check 2: Response time is reasonable
    if response.metadata.inference_time_ms < 10:
        checks.append(False)  # Suspiciously fast

    # Check 3: Semantic similarity to consensus
    if consensus_exists():
        similarity = compute_similarity(response, get_consensus())
        checks.append(similarity > 0.7)

    # Check 4: Worker signature is valid
    checks.append(verify_signature(response, worker_id))

    return all(checks)
```

**Mechanism 3: Reputation System**
```python
class WorkerReputation:
    def __init__(self):
        self.scores = {}  # worker_id -> reputation_score

    def update_score(self, worker_id, correct):
        if worker_id not in self.scores:
            self.scores[worker_id] = 1.0

        # EWMA update: new_score = α * correct + (1-α) * old_score
        alpha = 0.1
        self.scores[worker_id] = (
            alpha * (1.0 if correct else 0.0) +
            (1 - alpha) * self.scores[worker_id]
        )

    def get_score(self, worker_id):
        return self.scores.get(worker_id, 0.5)  # Neutral default

    def should_trust(self, worker_id, threshold=0.7):
        return self.get_score(worker_id) >= threshold
```

**Mechanism 4: Consensus Protocol**
```python
def byzantine_resistant_consensus(responses):
    """
    Achieve consensus among responses even with Byzantine workers

    Uses a variant of PBFT (Practical Byzantine Fault Tolerance)
    """
    n = len(responses)
    f = (n - 1) // 3  # Maximum Byzantine workers

    # Phase 1: Pre-prepare
    # Head node broadcasts query to all workers

    # Phase 2: Prepare
    # Each worker signs and broadcasts its response
    signed_responses = [sign_response(r) for r in responses]

    # Phase 3: Commit
    # If worker receives 2f+1 matching responses, it commits
    response_groups = group_by_similarity(signed_responses)

    for group in response_groups:
        if len(group) >= 2*f + 1:
            # We have a quorum on this response
            return consensus_from_group(group)

    # No consensus reached
    return None

def group_by_similarity(responses, threshold=0.9):
    """
    Group responses that are semantically similar
    """
    groups = []
    for response in responses:
        placed = False
        for group in groups:
            if all(similarity(response, r) > threshold for r in group):
                group.append(response)
                placed = True
                break
        if not placed:
            groups.append([response])
    return groups
```

### 5.3 Proof-of-Work for Sybil Resistance

**Optional: Computational Puzzle**
```python
def generate_proof_of_work_challenge():
    """
    Generate a computational puzzle that workers must solve to register

    This prevents Sybil attacks where attacker creates many fake workers
    """
    challenge = os.urandom(32)
    difficulty = 20  # Number of leading zero bits required

    return {
        'challenge': challenge.hex(),
        'difficulty': difficulty,
        'algorithm': 'sha256'
    }

def verify_proof_of_work(challenge, nonce, difficulty):
    """
    Verify that worker solved the puzzle
    """
    data = bytes.fromhex(challenge) + nonce.to_bytes(8, 'big')
    hash_result = hashlib.sha256(data).digest()

    # Check if hash has enough leading zero bits
    leading_zeros = count_leading_zero_bits(hash_result)
    return leading_zeros >= difficulty

def count_leading_zero_bits(data):
    count = 0
    for byte in data:
        if byte == 0:
            count += 8
        else:
            count += (7 - byte.bit_length())
            break
    return count
```

## 6. Network Monitoring

### 6.1 Metrics Collection

**Key Metrics**
```python
@dataclass
class NetworkMetrics:
    # Latency metrics
    query_latency_p50: float  # ms
    query_latency_p95: float
    query_latency_p99: float

    # Throughput metrics
    queries_per_second: float
    tokens_per_second: float

    # Network metrics
    bytes_sent_per_second: float
    bytes_received_per_second: float
    packet_loss_rate: float

    # Worker metrics
    active_workers: int
    worker_utilization: Dict[str, float]  # worker_id -> utilization

    # Error metrics
    error_rate: float
    timeout_rate: float
    byzantine_detections: int

    # Consensus metrics
    consensus_rounds_avg: float
    debate_duration_avg: float
```

### 6.2 Health Checks

**Worker Health Check**
```python
async def health_check_worker(worker):
    """
    Comprehensive health check for a worker
    """
    checks = {}

    # 1. Network reachability
    try:
        await asyncio.wait_for(
            worker.ping(),
            timeout=1.0
        )
        checks['reachable'] = True
    except asyncio.TimeoutError:
        checks['reachable'] = False

    # 2. Response correctness (canary query)
    canary_query = "What is 2+2?"
    expected = "4"
    response = await worker.query(canary_query)
    checks['correct'] = (expected in response)

    # 3. Latency within bounds
    latency = response.metadata.inference_time_ms
    checks['latency_ok'] = (latency < 100)

    # 4. Resource usage healthy
    metrics = await worker.get_metrics()
    checks['cpu_ok'] = (metrics.cpu_usage < 0.9)
    checks['memory_ok'] = (metrics.memory_usage < 0.9)
    checks['gpu_ok'] = (metrics.gpu_usage < 0.95)

    # Overall health
    return all(checks.values()), checks
```

## 7. Load Balancing

### 7.1 Routing Strategy

**Capability-Aware Routing**
```python
class IntelligentRouter:
    def __init__(self, workers):
        self.workers = workers
        self.query_classifier = QueryClassifier()
        self.load_balancer = LoadBalancer()

    async def route_query(self, query):
        # 1. Classify query to identify needed capabilities
        capabilities = self.query_classifier.classify(query)

        # 2. Find workers with those capabilities
        candidate_workers = [
            w for w in self.workers
            if any(c in w.capabilities for c in capabilities)
        ]

        # 3. Filter by health and reputation
        healthy_workers = [
            w for w in candidate_workers
            if w.is_healthy() and w.reputation > 0.7
        ]

        # 4. Load balance among healthy workers
        selected_workers = self.load_balancer.select(
            healthy_workers,
            strategy='least_loaded'
        )

        return selected_workers
```

**Load Balancing Strategies**
```python
class LoadBalancer:
    def select(self, workers, strategy='least_loaded'):
        if strategy == 'round_robin':
            return self.round_robin(workers)
        elif strategy == 'least_loaded':
            return self.least_loaded(workers)
        elif strategy == 'latency_weighted':
            return self.latency_weighted(workers)
        elif strategy == 'capability_optimal':
            return self.capability_optimal(workers)

    def least_loaded(self, workers):
        """Select worker with lowest current load"""
        return min(workers, key=lambda w: w.current_load)

    def latency_weighted(self, workers):
        """Probabilistic selection weighted by inverse latency"""
        weights = [1.0 / w.avg_latency for w in workers]
        return random.choices(workers, weights=weights)[0]

    def capability_optimal(self, workers):
        """Select worker with best capability match"""
        return max(workers, key=lambda w: w.capability_score)
```

---

**Status**: Network protocol specification complete
**Last Updated**: 2025-10-15
**Implementation**: Requires gRPC, Protocol Buffers, cryptography libraries
