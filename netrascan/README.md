# 👁️ Netrascan: Wallet Classification Engine

> Behavioral scoring and profiling for individual wallets — the first step in the Garunetra observability stack.

---

## 🎯 Purpose

`netrascan` scans individual wallets to generate behavioral classification scores.  
It powers deeper insights for tools like `netraflow`, `netratrace`, and `netrawatch` by producing rich, trusted wallet metadata.

It provides:

- Score-based wallet classification
- Lightweight address profiling
- Offline + batch analysis support
- Portable CLI for audits or research

---

## 📥 Input

Single wallet address via CLI:

```bash
netrascan 0xabc123... --etherscan-key <API_KEY>
```

---

## 📥 Output schema

expected format from Netrascan output:

```json
{
  "wallet": "0xabc...",
  "score": 0.85,
  "category": "domestic",
  "features": {
    "total_tx": 42,
    "total_out_usd": 1234.56,
    "distinct_to_count": 7,
    "interacts_with_cex": true,
    "used_bridges": false,
    "mixer_pattern_score": 0.05
  },
  "txs": [
    {
      "from": "0xabc...",
      "to": "0xdef...",
      "token_symbol": "usdt",
      "timestamp": "1719510000"
    },
    ...
  ]
}
```

---

## 🧠 Classification Categories

| Category   | Description                                 |
| ---------- | ------------------------------------------- |
| `domestic` | High volume but local token flow            |
| `foreign`  | Outbound-heavy, especially to known CEXs    |
| `bridge`   | Relays funds between categories or chains   |
| `mixer`    | Obfuscation behavior (small txs, repeatable |
| `unknown`  | Not enough data or outside heuristic scope  |

---

## ⚙️ Architecture Overview

```text
[ wallet address ]
        ↓
 ┌──────────────┐
 │  netrascan   │
 │ (rust CLI)   │
 └────┬─────────┘
      │
┌─────▼─────────────┐
│ etherscan ingest  │ ← pulls historical txs
└─────┬─────────────┘
      │
┌─────▼─────────────┐
│ heuristic scoring │ ← uses tx features, address roles
└─────┬─────────────┘
      │
┌─────▼─────────────┐
│ classification    │ ← assigns score + label
└─────┬─────────────┘
      │
┌─────▼─────────────┐
│ output (jsonl)    │
└───────────────────┘
```

---

## 🔋 Tech Stack

- **Language**: Rust
- **Crates**:
  - `reqwest` — HTTP client (Etherscan integration)
  - `serde` / `serde_json` — structured data handling
  - `chrono` — timestamp manipulation
  - `clap` — CLI interface
  - `rayon` — future parallelization

---

## 🚧 MVP Features To Build

- [x] CLI: `netrascan analyze <wallet>`
- [x] Fetch transactions from Etherscan
- [x] Score wallet via heuristics
- [x] Classify wallet type
- [x] Print output to terminal
- [ ] Write structured JSONL to disk
- [ ] Accept and scan a batch wallet list as input

---

## 🧠 Future Features

- [ ] Plug-in provider support (Alchemy, QuickNode)
- [ ] Parallel batch scanning
- [ ] Fuzzy classification confidence levels
- [ ] Local caching to avoid repeated queries
- [ ] Known address tagging (e.g., known CEX, bridge, mixer)
- [ ] API mode for streaming results to Garunetra backend
