# 🛡️ Netrawatch: Realtime Surveillance & Alerting Engine

> Continuous monitoring of wallet activity, capital movement, and behavioral anomalies.

---

## 🎯 Purpose

`netrawatch` is the **real-time monitoring layer** of the Garunetra observability stack. It watches Ethereum (and optionally cross-chain) activity live and raises alerts for:

- Suspicious behavior (e.g. mixer funneling, bridge hops)
- Large outbound flows from tagged wallets
- Unusual transaction patterns (low-value spam, sudden spikes)
- Known wallet interactions (e.g. CEX → unknown)

It serves security teams, forensic analysts, and on-chain compliance systems needing **live intelligence**.

---

## 🔁 Key Use Cases

- Real-time mixer tracing (e.g. Tornado → fresh wallet)
- Alerting on bridge → CEX flows
- Surveillance of high-risk wallet clusters
- Monitoring outbound behavior from flagged wallets
- Continuous data collection for netrascan/netratrace ingestion

---

## 📥 Input Sources

- Live JSON-RPC stream (via provider like Alchemy, Infura, or local node)
- Internal tags (from `netrascan`, `netratrace`, `netraflow`)
- Known wallet lists: CEXs, bridges, mixers

```bash
netrawatch start --provider wss://mainnet.infura.io/ws/v3/<KEY>

# Optional flags:
--watch-list ./watchlist.txt
--tagged-wallets ./known_cex.json
--alert-to webhook_url
```

---

## 🧠 Surveillance Features

### 🔎 1. Mixer Behavior Detection

- Funnel detection (many-to-one)
- Small, repeated tx patterns
- Unusual temporal patterns (e.g. clustered sends)

### 🌉 2. Bridge + CEX Alerts

- Watch flow between bridge contracts and CEX wallets
- Alert when wallet touches both in short time

### 💸 3. Volume Spike Watch

- Detect large sudden outflow spikes
- Trigger if over USD threshold within a time window

### 🏷️ 4. Tag Matching

- Match wallet txs against tagged addresses (CEX, mixer, suspicious clusters)
- Trace indirect interactions (2-hop, N-hop)

---

## ⚙️ Architecture Overview

```text
 [ live tx stream ]
        ↓
 ┌───────────────┐
 │  netrawatch   │
 │  (Rust CLI)   │
 └──────┬────────┘
        │
 ┌──────▼────────────┐
 │  tx classifier    │ ← heuristics & flow detection
 └──────┬────────────┘
        │
 ┌──────▼────────────┐
 │ alert engine      │ ← filters, threshold, tags
 └──────┬────────────┘
        │
 ┌──────▼─────────────────────┐
 │ sink (stdout, webhook, DB) │
 └────────────────────────────┘
```

---

## 📤 Output Format

### 🧠 Alert JSON (example)

```json
{
  "wallet": "0xabc...",
  "alert_type": "bridge_cex_flow",
  "severity": "high",
  "details": {
    "path": ["0xabc...", "0xbridge...", "0xkucoin..."],
    "total_usd": 12000.0
  },
  "timestamp": "2025-06-27T12:34:56Z"
}
```

---

## 🔋 Tech Stack

- **Language**: Rust
- **Crates**:
  - `tokio` — async runtime
  - `ethers-rs` — Ethereum client
  - `serde` — serialization
  - `clap` — CLI interface
  - `rayon` — concurrent matching
  - `notify` — (optional) hot-reload of watchlists
  - `reqwest` — webhook delivery

---

## 🚧 MVP Features to Build

- [ ] CLI: `netrawatch start --provider <url>`
- [ ] Load tagged wallets + watchlist
- [ ] Watch live mempool or confirmed txs
- [ ] Run heuristic matchers on tx stream
- [ ] Emit alert JSON to stdout or webhook

---

## 🧠 Future Features

- [ ] Multi-chain support (via RPC abstraction)
- [ ] Alert dashboard with severity filters
- [ ] SQLite or Redis-backed alert sink
- [ ] Integration with Discord, Slack, email
- [ ] Integration with `netravue` for live map view
- [ ] Actor-based rule engine for custom logic
