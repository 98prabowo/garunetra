# 🌊 Netraflow: Regional Capital Flow Analyzer

> Aggregates wallet behaviors into macro-level capital movement insights — powered by `netrascan`.

---

## 🎯 Purpose

`netraflow` consumes classified transaction data from `netrascan` and generates high-level intelligence about:

- Capital inflow/outflow trends
- Regionalized wallet behaviors
- Token usage by destination
- Exchange aggregation (e.g. Binance, OKX)

Ideal for macro-level blockchain observers, economic analysts, and investigative auditors.

---

## 📥 Input Schema

Consumes `.jsonl` files from `netrascan`:

```json
{
  "wallet": "0xabc...",
  "score": 0.85,
  "category": "domestic",
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

## 📊 Core Features

### 1. 🗓️ Daily Capital Outflow Report

- Aggregate outbound token volume from `domestic` wallets
- Breakdown by destination (CEXs, foreign wallets)
- Token-specific summaries: `USDT`, `BIDR`, `IDRT`, etc.

### 2. 🌍 Region-Specific Wallet Cluster Tracking

- Group wallets by destination geography (if determinable)
- Monitor cluster activity over time ("Binance hot wallet" detection)

### 3. ⚠️ Anomaly Detection

- Identify sudden surges in outflow
- Flag suspicious behavior (e.g. mixer funneling, bridge → CEX chains)

### 4. 📈 Top Destination Analysis

- Most common foreign endpoints for Indonesian-origin tokens
- Optional cross-reference to known exchange wallets (CEXs, mixers, bridges)

---

## 🛠️ Architecture Overview

```text
[ netrascan output files (.jsonl) ]
          ↓
   ┌───────────────┐
   │   netraflow   │
   │ (rust binary) │
   └──────┬────────┘
          │
   ┌──────▼────────┐
   │ data pipeline │  ← parse, bucket, aggregate
   └──────┬────────┘
          │
  ┌───────▼─────────────┐
  │ outflow time series │
  └─────────────────────┘
          │
  ┌───────▼────────────┐
  │ top dest summary   │
  └────────────────────┘
          │
  ┌───────▼────────────┐
  │ export (csv/json)  │
  └────────────────────┘
```

---

## 🔋 Planned Tech Stack

- **Language**: Rust
- **Crates**:
  - `serde` / `serde_json` — structured input parsing
  - `chrono` — timestamp handling
  - `rayon` — parallel aggregation
  - `csv` — export to human-readable reports

---

## 🚧 MVP Features to Build

- [ ] CLI: `netraflow analyze <input_dir>`
- [ ] Input: read `.jsonl` scan result from `netrascan`
- [ ] Aggregate volume by category (domestic → foreign) and token
- [ ] Generate daily CSV reports:
  - `outflow_by_day.csv`
  - `top_dests.csv`
- [ ] Optional: outflow graph image export (via python or gnuplot)

---

## 🧠 Future Features

- [ ] Real-time input stream from `netrascan`
- [ ] GeoIP or region enrichment (based on endpoints)
- [ ] SQLite/Parquet export for dashboards
- [ ] Email/webhook alerts for sudden flow spikes
- [ ] Weekly token movement summary for newsletters or reports
