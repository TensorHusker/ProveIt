# Butterfly Terminal Interface & Accessibility

## 1. Design Philosophy

### 1.1 Accessibility-First Principles

**Core Tenets:**
1. **Non-Visual Primary**: Terminal interface is primary, not secondary
2. **Screen Reader Native**: All output optimized for screen readers (NVDA, JAWS, VoiceOver)
3. **Cognitive Accessibility**: Clear, consistent command structure for neurodivergent users
4. **Progressive Disclosure**: Simple commands for beginners, power features for experts
5. **Multi-Modal**: Audio, tactile, and textual feedback channels

**Anti-Patterns to Avoid:**
- ASCII art or visual-only representations
- Assuming sighted users
- Complex nested menus
- Unclear command names
- Silent failures

### 1.2 Interaction Modalities

**Primary: Terminal (Text-Based)**
- All functionality accessible via text commands
- Consistent command structure (verb-noun pattern)
- Tab completion and help at every level
- Parseable output (JSON mode for scripting)

**Secondary: Audio**
- Text-to-speech for all output
- Sonification of metrics (pitch = latency, volume = load)
- Spatial audio for worker locations in distributed system

**Tertiary: Tactile (Future)**
- Haptic feedback for inference progress
- Vibration patterns for different event types
- Braille displays for compact status

## 2. Command-Line Interface

### 2.1 Command Structure

**Pattern: `butterfly <command> <subcommand> [options] [arguments]`**

**Command Categories:**
```
butterfly query          - Submit inference queries
butterfly worker         - Manage worker nodes
butterfly model          - Model splitting and management
butterfly network        - Network configuration and monitoring
butterfly consensus      - Consensus protocol operations
butterfly verify         - Formal verification
butterfly status         - System status and metrics
butterfly config         - Configuration management
butterfly help           - Help and documentation
```

### 2.2 Query Commands

**Submit Query**
```bash
butterfly query submit "What is the proof that sqrt(2) is irrational?" \
  --workers logic,formal \
  --fusion sctt-smooth \
  --timeout 5000 \
  --stream

# Output:
# [Query ID: 7f8e2d4a-3b1c-4f8e-9d2a-1c3e5f7g9h1i]
# [Routing to: worker-logic-001, worker-formal-001]
# [Fusion: SCTT Smooth Combination]
#
# Response:
# The proof proceeds by contradiction. Assume sqrt(2) = p/q where...
# [continues streaming...]
#
# [Consensus: 2/2 workers agreed (100%)]
# [Latency: 47ms]
# [Verification: PASSED]
```

**Query with Debate**
```bash
butterfly query submit "Is P=NP?" \
  --mode debate \
  --rounds 3 \
  --workers all

# Output:
# [Query ID: ...]
# [Initiating 3-round debate among 4 workers]
#
# Round 1: Initial Proposals
# - worker-logic-001: "Likely no, due to..."
# - worker-formal-001: "Current evidence suggests..."
# - worker-pattern-001: "The complexity classes..."
# - worker-creative-001: "From an intuitive perspective..."
#
# Round 2: Critiques
# - worker-logic-001 → worker-creative-001: "Lacks formal rigor..."
# - worker-formal-001 → worker-pattern-001: "Missing key theorem..."
# [continues...]
#
# Round 3: Synthesis
# [Final consensus with merged reasoning]
```

**Stream Tokens**
```bash
butterfly query stream "Explain quantum entanglement" --audio

# Audio output (with pitch variation for confidence):
# [High pitch, confident]: "Quantum entanglement is a phenomenon..."
# [Medium pitch]: "where particles become correlated..."
# [Low pitch, uncertain]: "Some interpretations suggest..."
```

### 2.3 Worker Management Commands

**List Workers**
```bash
butterfly worker list

# Output:
# ID                   Name              Status  Load  Capability        Reputation
# worker-logic-001     Logic Worker 1    UP      23%   logical_reasoning 0.94
# worker-formal-001    Formal Worker 1   UP      18%   formal_verify     0.91
# worker-pattern-001   Pattern Worker 1  UP      35%   pattern_recog     0.88
# worker-creative-001  Creative Worker 1 DOWN    --    creative_gen      0.76
#
# Total: 4 workers (3 up, 1 down)
```

**Worker Details**
```bash
butterfly worker info worker-logic-001

# Output:
# Worker: worker-logic-001
# Name: Logic Worker 1
# Status: UP (last heartbeat 2s ago)
# Address: 10.0.1.23:8000
# Capabilities:
#   - logical_reasoning (confidence: 0.95)
#     Benchmarks: arc_challenge=0.87, math_qa=0.91
#   - proof_verification (confidence: 0.89)
# Hardware:
#   Device: CUDA (NVIDIA A100, 80GB)
#   Compute: 8.0
# Network:
#   Protocol: gRPC + TLS
#   Latency: p50=23ms, p95=45ms, p99=78ms
# Load:
#   Current queries: 3/16
#   CPU: 45%, Memory: 62%, GPU: 71%
# Reputation: 0.94 (1,247 queries, 1,172 correct)
# Registered: 2025-10-15 08:23:41 UTC
```

**Register Worker**
```bash
butterfly worker register \
  --name "logic-worker-002" \
  --address "10.0.1.24:8000" \
  --capability logical_reasoning:0.95 \
  --hardware cuda:A100:80gb \
  --model-path "/models/llama-logic-variant.pt"

# Output:
# Registering worker...
# [✓] Connection test passed
# [✓] Model loaded successfully
# [✓] Capability verification passed
# [✓] Proof-of-work challenge completed
#
# Worker registered: worker-logic-002
# Worker ID: a8f3e7d2-9c4b-4a1f-8e7d-2c5f9g3h6i9j
```

**Deregister Worker**
```bash
butterfly worker deregister worker-logic-002 --reason "maintenance"

# Output:
# Deregistering worker-logic-002...
# [✓] Active queries migrated to other workers
# [✓] State synchronized
# [✓] Worker removed from cluster
#
# Worker deregistered successfully
```

### 2.4 Model Management Commands

**Split Model**
```bash
butterfly model split \
  --model llama-3-70b \
  --strategy hybrid_functional \
  --workers 4 \
  --output-dir /models/split/

# Output:
# Splitting llama-3-70b into 4 workers...
#
# Phase 1: Capability Analysis
# [██████████████████████████████] 100% (157 layers analyzed)
# Identified capabilities: logical_reasoning, pattern_recognition, formal_verification, creative_generation
#
# Phase 2: Layer Assignment
# Worker 1 (logical): layers [0-3, 12-18, 22-24] (7.3B params)
# Worker 2 (pattern): layers [0-3, 4-10, 11-13] (6.8B params)
# Worker 3 (formal): layers [0-3, 16-21, 23-24] (7.1B params)
# Worker 4 (creative): layers [0-3, 14-20, 22-24] (7.5B params)
#
# Phase 3: Pruning
# [██████████████████████████████] 100% (pruned 23% of parameters)
#
# Phase 4: Fine-Tuning
# Worker 1: [██████████████████████████████] epoch 5/5 (acc: 0.94)
# Worker 2: [██████████████████████████████] epoch 5/5 (acc: 0.91)
# Worker 3: [██████████████████████████████] epoch 5/5 (acc: 0.92)
# Worker 4: [██████████████████████████████] epoch 5/5 (acc: 0.89)
#
# Phase 5: Saving
# [✓] /models/split/worker-1-logic.pt (7.3GB)
# [✓] /models/split/worker-2-pattern.pt (6.8GB)
# [✓] /models/split/worker-3-formal.pt (7.1GB)
# [✓] /models/split/worker-4-creative.pt (7.5GB)
# [✓] /models/split/metadata.json
#
# Model splitting complete!
# Total time: 3h 24m 17s
```

**Load Model to Worker**
```bash
butterfly model load \
  --worker worker-logic-001 \
  --model-path /models/split/worker-1-logic.pt

# Output:
# Loading model to worker-logic-001...
# [✓] Model file validated (checksum: a7f3e2d9...)
# [✓] Model loaded to GPU (7.3GB allocated)
# [✓] Capability verification passed
# [✓] Test query successful
#
# Model loaded successfully
# Ready for inference
```

**Benchmark Model**
```bash
butterfly model benchmark \
  --worker worker-logic-001 \
  --dataset arc_challenge \
  --metrics accuracy,latency

# Output:
# Benchmarking worker-logic-001 on arc_challenge...
# [██████████████████████████████] 100% (500 samples)
#
# Results:
# Accuracy: 0.87 (435/500 correct)
# Latency:
#   Mean: 23.4ms
#   Median: 21.7ms
#   P95: 45.2ms
#   P99: 78.9ms
# Throughput: 42.7 queries/sec
# Token/sec: 847.3
#
# Breakdown by difficulty:
# Easy:   0.95 (100/105)
# Medium: 0.88 (220/250)
# Hard:   0.79 (115/145)
```

### 2.5 Network Commands

**Network Status**
```bash
butterfly network status

# Output:
# Network Status
# ==============
#
# Topology: Mesh (1 head + 4 workers)
# Protocol: gRPC over QUIC
# Encryption: TLS 1.3
#
# Head Node:
#   Address: 10.0.1.10:8000
#   Status: UP
#   Load: 15% CPU, 32% Memory
#
# Workers:
#   4 registered, 3 active, 1 down
#   Average latency: 28ms
#   Packet loss: 0.02%
#
# Consensus:
#   Protocol: Wingbeat (PBFT variant)
#   Byzantine tolerance: f < n/3 (supports 1 Byzantine worker)
#   Consensus rounds (avg): 1.2
#
# Throughput:
#   Queries/sec: 34.7
#   Tokens/sec: 1,847
#   Bytes/sec: 847KB sent, 2.1MB received
```

**Network Topology**
```bash
butterfly network topology --format audio

# Audio output (spatial positioning):
# [Center]: Head node at 10.0.1.10
# [Front-left]: Worker 1 (logic) at 10.0.1.23 - 23ms latency
# [Front-right]: Worker 2 (pattern) at 10.0.1.24 - 28ms latency
# [Back-left]: Worker 3 (formal) at 10.0.1.25 - 19ms latency
# [Back-right]: Worker 4 (creative) - OFFLINE
#
# [Sound: Ping tones at each location, pitch indicates latency]
```

**Test Connection**
```bash
butterfly network ping worker-logic-001 --count 10

# Output:
# PING worker-logic-001 (10.0.1.23:8000)
# Reply from worker-logic-001: time=22ms signature_verified=yes
# Reply from worker-logic-001: time=24ms signature_verified=yes
# Reply from worker-logic-001: time=21ms signature_verified=yes
# Reply from worker-logic-001: time=23ms signature_verified=yes
# Reply from worker-logic-001: time=25ms signature_verified=yes
# Reply from worker-logic-001: time=22ms signature_verified=yes
# Reply from worker-logic-001: time=24ms signature_verified=yes
# Reply from worker-logic-001: time=21ms signature_verified=yes
# Reply from worker-logic-001: time=23ms signature_verified=yes
# Reply from worker-logic-001: time=22ms signature_verified=yes
#
# --- worker-logic-001 ping statistics ---
# 10 packets transmitted, 10 received, 0% packet loss
# rtt min/avg/max/mdev = 21/22.7/25/1.3 ms
```

### 2.6 Consensus Commands

**View Consensus State**
```bash
butterfly consensus status

# Output:
# Consensus State
# ===============
#
# Protocol: Wingbeat v1.0
# View: 42
# Sequence: 1,847
# Phase: IDLE (ready for queries)
#
# Last Consensus:
#   Query ID: 7f8e2d4a-3b1c-4f8e-9d2a-1c3e5f7g9h1i
#   Rounds: 1
#   Agreement: 3/3 workers (100%)
#   Latency: 47ms
#   Status: FINALIZED
#
# Byzantine Detections (last 24h): 0
# Failed Consensus (last 24h): 2 (timeout)
# View Changes (last 24h): 1
```

**Force View Change**
```bash
butterfly consensus view-change --reason "suspected-leader-failure"

# Output:
# Initiating view change...
# [✓] VIEW_CHANGE broadcast to all workers
# [✓] Collected 3/3 VIEW_CHANGE messages
# [✓] New leader elected: worker-pattern-001
# [✓] NEW_VIEW broadcast
#
# View change complete
# Old view: 42
# New view: 43
# Time: 124ms
```

**Consensus History**
```bash
butterfly consensus history --limit 10

# Output:
# Recent Consensus History
# ========================
#
# 1. [2025-10-15 14:32:18] Query: "What is sqrt(2)?"
#    Rounds: 1, Agreement: 3/3 (100%), Latency: 47ms
#    Status: FINALIZED
#
# 2. [2025-10-15 14:32:03] Query: "Prove Fermat's Last Theorem"
#    Rounds: 3, Agreement: 2/3 (67%), Latency: 283ms
#    Status: FINALIZED (worker-creative-001 dissented)
#
# 3. [2025-10-15 14:31:45] Query: "Is P=NP?"
#    Rounds: TIMEOUT, Agreement: --, Latency: 5000ms
#    Status: FAILED (timeout)
#
# [continues...]
```

### 2.7 Verification Commands

**Verify Fusion**
```bash
butterfly verify fusion \
  --query "What is 2+2?" \
  --workers worker-logic-001,worker-pattern-001 \
  --expected "4"

# Output:
# Verifying fusion correctness...
#
# Step 1: Individual Worker Outputs
# - worker-logic-001: "4" (confidence: 0.99)
# - worker-pattern-001: "4" (confidence: 0.98)
#
# Step 2: Fusion Result
# - Fusion output: "4" (confidence: 0.99)
#
# Step 3: Correctness Checks
# [✓] Type validity: output in TokenSpace
# [✓] Quality preservation: fused ≥ max individual
# [✓] Smoothness: Lipschitz constant = 0.87 < 1.0
# [✓] Agreement: all workers agree
#
# Step 4: SCTT Proof Generation
# [✓] Proof term generated
# [✓] Proof type-checks
# [✓] Proof verifies in SCTT
#
# Verification PASSED
# Certificate: /tmp/verify-7f8e2d4a.proof
```

**Verify Worker**
```bash
butterfly verify worker worker-logic-001 \
  --test-dataset /data/arc_samples.json

# Output:
# Verifying worker-logic-001...
#
# Test 1: Canary Queries
# [✓] "What is 2+2?" → "4" (correct)
# [✓] "Capital of France?" → "Paris" (correct)
# [✓] "sqrt(-1)?" → "i" (correct)
#
# Test 2: Proof-of-Knowledge
# [✓] VRF signature valid
# [✓] Computational proof valid
# [✓] Reasoning trace consistent
#
# Test 3: Dataset Accuracy
# [██████████████████████████████] 100% (100 samples)
# Accuracy: 0.91 (91/100 correct)
# Expected: 0.87-0.95 (within range)
#
# Test 4: Byzantine Resistance
# [✓] Worker does not accept malicious inputs
# [✓] Worker signatures valid
# [✓] Worker reputation consistent
#
# Verification PASSED
# Worker is TRUSTED
```

### 2.8 Status and Monitoring

**System Status**
```bash
butterfly status

# Output:
# Butterfly Distributed Inference System
# ======================================
#
# System: UP (uptime: 7d 14h 23m)
# Version: 1.0.0-beta
#
# Workers: 3/4 active
# Queries: 1,847 processed (34.7/sec)
# Errors: 23 (1.2% error rate)
# Latency: p50=47ms, p95=124ms, p99=283ms
#
# Network: OK
#   Bandwidth: 847KB/s sent, 2.1MB/s received
#   Packet loss: 0.02%
#
# Consensus: HEALTHY
#   Agreement: 98.4% (last 100 queries)
#   Byzantine detections: 0 (last 24h)
#
# Resources:
#   Head node: 15% CPU, 32% Memory
#   Workers: 41% CPU avg, 58% Memory avg, 67% GPU avg
#
# Status: OPTIMAL
```

**Live Monitoring**
```bash
butterfly status watch --interval 1 --audio

# Output (updates every 1 second):
# [14:32:18] Queries: 1847 (+1) | Latency: 47ms | Workers: 3/4 UP
# [Audio: Low tone = healthy, pitch rises with latency]
#
# [14:32:19] Queries: 1848 (+1) | Latency: 52ms | Workers: 3/4 UP
# [Audio: Slightly higher pitch]
#
# [14:32:20] Queries: 1849 (+1) | Latency: 124ms | Workers: 3/4 UP
# [Audio: Higher pitch + warning beep]
#
# [14:32:21] Queries: 1849 (+0) | Latency: -- | Workers: 3/4 UP
# [Audio: Silence (no query)]
```

**Metrics Export**
```bash
butterfly status export \
  --format prometheus \
  --output /var/metrics/butterfly.prom

# Output: Prometheus metrics file
# Includes:
# - butterfly_queries_total counter
# - butterfly_latency_seconds histogram
# - butterfly_workers_active gauge
# - butterfly_errors_total counter
# - butterfly_consensus_agreement gauge
# [etc.]
```

### 2.9 Configuration Commands

**View Config**
```bash
butterfly config show

# Output:
# Configuration
# =============
#
# Network:
#   head_node: "10.0.1.10:8000"
#   protocol: "grpc"
#   encryption: "tls1.3"
#
# Consensus:
#   protocol: "wingbeat"
#   timeout_ms: 5000
#   similarity_threshold: 0.9
#
# Fusion:
#   strategy: "sctt-smooth"
#   meta_learning: true
#   confidence_weighting: true
#
# Workers:
#   min_workers: 2
#   max_workers: 10
#   health_check_interval: 10
#
# Logging:
#   level: "info"
#   output: "/var/log/butterfly.log"
#   format: "json"
#
# Accessibility:
#   audio_enabled: true
#   speech_rate: 1.2
#   spatial_audio: true
```

**Set Config**
```bash
butterfly config set consensus.timeout_ms 10000

# Output:
# Updated configuration:
# consensus.timeout_ms: 5000 → 10000
#
# [!] Configuration change will take effect after restart
```

**Reset Config**
```bash
butterfly config reset --confirm

# Output:
# Resetting configuration to defaults...
# [✓] Configuration reset
#
# Please restart Butterfly for changes to take effect:
#   butterfly restart
```

## 3. Accessibility Features

### 3.1 Screen Reader Optimization

**Semantic HTML Output (for terminal multiplexers)**
```bash
butterfly query submit "What is sqrt(2)?" --format semantic

# Output (with ARIA labels):
# <section role="region" aria-label="Query Result">
#   <h1>Query Response</h1>
#   <p aria-label="Response text">The square root of 2 is approximately 1.414...</p>
#   <footer aria-label="Metadata">
#     <span>Consensus: 3/3 workers</span>
#     <span>Latency: 47ms</span>
#   </footer>
# </section>
```

**Verbose Mode**
```bash
butterfly worker list --verbose

# Output (extra context for screen readers):
# Worker listing. Total: 4 workers. 3 are up, 1 is down.
#
# Worker 1 of 4.
# ID: worker-logic-001
# Name: Logic Worker 1
# Status: Up. Last heartbeat 2 seconds ago.
# Load: 23 percent.
# Capability: logical reasoning. Reputation: 0.94.
#
# Worker 2 of 4.
# ID: worker-formal-001
# [continues...]
```

**Phonetic Identifiers**
```bash
butterfly config set accessibility.phonetic_ids true

butterfly worker list

# Output:
# ID                   Name              Status
# whiskey-lima-001     Logic Worker 1    UP
# whiskey-foxtrot-001  Formal Worker 1   UP
# whiskey-papa-001     Pattern Worker 1  UP
# whiskey-charlie-001  Creative Worker 1 DOWN
```

### 3.2 Audio Output

**Text-to-Speech**
```bash
butterfly query submit "What is quantum entanglement?" --tts

# Audio output:
# [Voice synthesis]: "Quantum entanglement is a phenomenon where particles become correlated such that the state of one particle cannot be described independently of the others..."
# [Rate: adjustable via config, default 1.2x]
# [Voice: system default, configurable]
```

**Sonification**
```bash
butterfly status watch --sonify

# Audio output:
# [Base tone at 440 Hz = system healthy]
# [Pitch increases with latency: 440 Hz (fast) → 660 Hz (slow)]
# [Volume increases with load: quiet (low load) → loud (high load)]
# [Harmonics indicate number of active workers: more harmonics = more workers]
# [Dissonance indicates errors or disagreement]
```

**Spatial Audio**
```bash
butterfly network topology --spatial-audio

# Audio output (3D positioned sound):
# [Center]: Head node heartbeat (steady pulse)
# [Front-left]: Worker 1 activity (intermittent clicks)
# [Front-right]: Worker 2 activity
# [Back-left]: Worker 3 activity
# [Back-right]: Worker 4 silence (offline)
#
# [User can "hear" network topology in 3D space]
```

### 3.3 Haptic Feedback (Future)

**Vibration Patterns**
```bash
butterfly config set accessibility.haptics true

# Patterns:
# - Short pulse: query received
# - Double pulse: query completed successfully
# - Long pulse: error occurred
# - Rapid pulses: high load warning
# - Pattern variation: different event types
```

## 4. Scripting and Automation

### 4.1 JSON Output Mode

**Machine-Readable Output**
```bash
butterfly worker list --format json

# Output:
{
  "workers": [
    {
      "id": "worker-logic-001",
      "name": "Logic Worker 1",
      "status": "up",
      "load": 0.23,
      "capabilities": [
        {"name": "logical_reasoning", "confidence": 0.95}
      ],
      "reputation": 0.94
    }
  ],
  "total": 4,
  "active": 3,
  "down": 1
}
```

### 4.2 Scripting Examples

**Health Check Script**
```bash
#!/bin/bash
# health_check.sh - Monitor Butterfly system health

STATUS=$(butterfly status --format json)
WORKERS_UP=$(echo "$STATUS" | jq '.workers_active')
MIN_WORKERS=2

if [ "$WORKERS_UP" -lt "$MIN_WORKERS" ]; then
  echo "CRITICAL: Only $WORKERS_UP workers active (minimum: $MIN_WORKERS)"
  # Send alert
  butterfly alert send --level critical --message "Insufficient workers"
  exit 2
fi

LATENCY_P95=$(echo "$STATUS" | jq '.latency.p95')
MAX_LATENCY=100

if (( $(echo "$LATENCY_P95 > $MAX_LATENCY" | bc -l) )); then
  echo "WARNING: High latency ($LATENCY_P95 ms > $MAX_LATENCY ms)"
  # Send warning
  butterfly alert send --level warning --message "High latency detected"
  exit 1
fi

echo "OK: System healthy ($WORKERS_UP workers, ${LATENCY_P95}ms latency)"
exit 0
```

**Automated Worker Scaling**
```bash
#!/bin/bash
# scale_workers.sh - Auto-scale workers based on load

LOAD=$(butterfly status --format json | jq '.load_avg')
THRESHOLD=0.7

if (( $(echo "$LOAD > $THRESHOLD" | bc -l) )); then
  echo "High load detected ($LOAD > $THRESHOLD), scaling up..."

  # Start new worker
  butterfly worker register \
    --name "auto-worker-$(date +%s)" \
    --address "10.0.1.$(shuf -i 50-100 -n 1):8000" \
    --capability logical_reasoning:0.9 \
    --auto-deregister-on-low-load

  echo "New worker registered"
fi
```

### 4.3 Event Streaming

**Watch for Events**
```bash
butterfly events stream --filter "query.completed,worker.failed"

# Output (streaming):
# [14:32:18] event=query.completed id=7f8e2d4a latency=47ms
# [14:32:19] event=query.completed id=8g9f3e5b latency=52ms
# [14:32:20] event=worker.failed id=worker-creative-001 reason=timeout
# [14:32:21] event=query.completed id=9h1g4f6c latency=124ms
# [continues streaming...]
```

**Webhooks**
```bash
butterfly events webhook add \
  --url "https://example.com/webhook" \
  --events "worker.failed,consensus.disagreement" \
  --secret "webhook_secret_key"

# Output:
# Webhook registered
# ID: webhook-001
# URL: https://example.com/webhook
# Events: worker.failed, consensus.disagreement
# Secret: [hidden]
#
# Test webhook:
#   butterfly events webhook test webhook-001
```

## 5. Help System

### 5.1 Context-Sensitive Help

**Top-Level Help**
```bash
butterfly --help

# Output:
# Butterfly - Distributed LLM Inference with Strategic Model Splitting
#
# USAGE:
#   butterfly <command> [subcommand] [options] [arguments]
#
# COMMANDS:
#   query       Submit and manage inference queries
#   worker      Manage worker nodes
#   model       Model splitting and management
#   network     Network configuration and monitoring
#   consensus   Consensus protocol operations
#   verify      Formal verification
#   status      System status and metrics
#   config      Configuration management
#   help        Show help information
#
# GLOBAL OPTIONS:
#   --format <json|text|semantic>  Output format (default: text)
#   --audio                        Enable audio output
#   --verbose                      Verbose output for screen readers
#   --config <path>                Config file path
#
# EXAMPLES:
#   butterfly query submit "What is sqrt(2)?"
#   butterfly worker list
#   butterfly status watch
#
# For detailed help on a command:
#   butterfly <command> --help
#
# Documentation: https://butterfly.proveit.org/docs
# Accessibility: https://butterfly.proveit.org/accessibility
```

**Command-Specific Help**
```bash
butterfly query --help

# Output:
# butterfly query - Submit and manage inference queries
#
# USAGE:
#   butterfly query <subcommand> [options] [arguments]
#
# SUBCOMMANDS:
#   submit      Submit a new query
#   stream      Submit query with token streaming
#   status      Check query status
#   cancel      Cancel ongoing query
#   history     View query history
#
# For detailed help on a subcommand:
#   butterfly query <subcommand> --help
```

### 5.2 Interactive Help

**Example Mode**
```bash
butterfly query submit --examples

# Output:
# Examples for 'butterfly query submit':
#
# 1. Basic query:
#    butterfly query submit "What is 2+2?"
#
# 2. With specific workers:
#    butterfly query submit "Prove P≠NP" --workers logic,formal
#
# 3. With streaming:
#    butterfly query submit "Explain quantum mechanics" --stream --audio
#
# 4. Debate mode:
#    butterfly query submit "Is P=NP?" --mode debate --rounds 3
#
# 5. Custom fusion:
#    butterfly query submit "..." --fusion sctt-smooth --weights 0.5,0.3,0.2
```

---

**Status**: Terminal interface specification complete
**Last Updated**: 2025-10-15
**Accessibility Compliance**: WCAG 2.1 Level AAA (target)
**Related**: NETWORK_PROTOCOL.md, FORMAL_SPEC.md
