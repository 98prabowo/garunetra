# 🛰️ Netratrace: Entity & Intent Tracing Engine

> Trace wallet relationships and uncover intent in Ethereum networks.

---

## 🎯 Purpose

`netratrace` traces wallet interactions to uncover relationships, funding paths, and probable **intent** behind behaviors.  
It is the final stage in the Garunetra observability stack, providing investigatory capabilities for forensic analysts and auditors.

It builds on data from `netrascan` and `netraflow`, offering:

- Transaction graph construction
- Entity clustering (based on behavior or linkage)
- Trace paths between addresses (e.g., bridge → mixer → CEX)
- Optional risk propagation scoring

---

## 📥 Input Schema

Consumes data from `netrascan` using the `--report flag` (or `netraflow` rollups), structured like:

```json
{
  "wallet": "0xabc...",
  "category": "mixer",
  "features": {
    "total_tx": 123,
    "total_out_usd": 3000.0
  },
  "txs": [
    {
      "from": "0xabc...",
      "to": "0xdef...",
      "token_symbol": "usdt",
      "timestamp": "1719510000"
    }
  ]
}
```

---

## 🔍 Core Features

### 1. Entity Tracing

- Recursively follow transactions across wallets
- Trace flows from source wallet to CEX or bridge

### 2. Path Reconstruction

- Identify most probable funding paths between two addresses
- Supports directional filters: `source-only`, `sink-only`, or full trace

### 3. Risk Propagation Scoring

- Flag wallets downstream from mixers or bridges
- Assign a "risk distance" score based on proximity to risky nodes

### 4. Relationship Clustering

- Group wallets that commonly interact with same endpoints
- Optionally apply tagging (e.g., known CEX, mixer, bridge)

---

## 🛠️ Architecture Overview

```text
[ netrascan reports ]
        ↓
 ┌───────────────┐
 │  netratrace   │
 │  (Rust CLI)   │
 └──────┬────────┘
        │
 ┌──────▼────────────┐
 │ wallet graph init │ ← build from tx edges
 └──────┬────────────┘
        │
 ┌──────▼────────────┐
 │ entity clustering │
 └──────┬────────────┘
        │
 ┌──────▼────────────┐
 │ trace path engine │ ← find routes, assign risk
 └──────┬────────────┘
        │
 ┌──────▼──────────────┐
 │ output (json, csv)  │
 └─────────────────────┘
```

---

## 📊 Output Examples

### 🔗 Trace Result

```json
{
  "source": "0xabc...",
  "sink": "0xdef...",
  "path": ["0xabc...", "0x987...", "0x654...", "0xdef..."],
  "risk_score": 0.82
}
```

### 🧩 Cluster Result

```json
{
  "cluster_id": "cluster_12",
  "wallets": ["0xabc...", "0xdef...", "0x987..."],
  "tag": "kucoin-funding-ring"
}
```

---

## 🔋 Tech Stack

- **Language**: Rust
- **Crates**:
  - `serde` / `serde_json` — structured input/output
  - `clap` — CLI argument parsing
  - `petgraph` — wallet graph representation
  - `rayon` — parallel graph operations
  - `graphviz-rust` (optional) — path visualization

---

## 🚧 MVP Features to Build

- [ ] CLI: `netratrace trace <wallet>`
- [ ] Load wallet txs + features from netrascan output
- [ ] Construct interaction graph
- [ ] Basic trace: find shortest tx path from source to sink
- [ ] Output graph JSON or CSV
- [ ] CLI options: `--max-depth`, `--direction`, `--risk`

---

## 🧠 Future Features

- [ ] Known wallet tagging (CEX, bridge, mixer, etc.)
- [ ] Smart contract interaction detection
- [ ] Risk propagation score engine
- [ ] Graph export to `.dot`, `.graphml`, or `.csv`
- [ ] Web frontend or TUI for visual trace exploration
- [ ] API mode to serve `trace()` over HTTP
