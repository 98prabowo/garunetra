# 🔭 Netravue: Visual Intelligence Explorer

> Visualize wallet behavior, capital flows, and trace networks — all from Garunetra output files.

---

## 🎯 Purpose

`netravue` is the visual interface of the **Garunetra** observability stack. It transforms raw analytical outputs from `netrascan`, `netraflow`, and `netratrace` into **insightful visualizations** — ideal for reports, audits, or real-time investigations.

It provides:

- Interactive trace graphs
- Outflow heatmaps and temporal trends
- Entity cluster views
- Wallet profile dashboards
- Exportable reports for investigations

---

## 📥 Input Format

Supported input files:

- `netrascan` reports (individual wallet scan `.jsonl`)
- `netraflow` rollups (aggregated capital flow data)
- `netratrace` outputs (cluster data or path traces)

Example:

```bash
netravue render ./netrascan/data/trainings/2025-06/wallet-report.jsonl
```

---

## 🖥️ Key Visual Features

### 1. Wallet Profile Viewer

- Timeline of token activity
- Flow direction: inbound vs outbound
- Score breakdown + classification tag
- Known endpoint annotations (CEX, mixer, bridge)

### 2. Entity Graph

- Graph of wallet connections
- Node coloring based on category
- Edge thickness = capital flow volume
- Click-to-expand subgraphs

### 3. Capital Flow Dashboard

- Daily/weekly/monthly outflow charts
- Token distribution pie chart
- Destination aggregation (e.g., Binance, OKX)

### 4. Trace Explorer

- Path view from source → sink
- Trace depth control
- Risk score overlays
- Export to `.png`, `.svg`, or `.dot`

## 📊 Output Snapshots

### 🧠 Wallet Profile (DOMESTIC)

| Property           | Value       |
| ------------------ | ----------- |
| Wallet             | `0xabc...`  |
| Classification     | `domestic`  |
| Score              | `0.78`      |
| Total Outflow      | `$4,300.00` |
| Distinct To        | `12`        |
| Interacts with CEX | `No`        |
| Bridge Usage       | `No`        |

### 🔗 Trace Path

```json
{
  "path": ["0xabc", "0x987", "0x321", "0xdef"],
  "risk_score": 0.82
}
```

---

## 🛠️ Architecture Overview

```text
[ netrascan / netraflow / netratrace ]
                ↓
          ┌──────────────┐
          │  netravue    │
          │ (UI backend) │
          └─────┬────────┘
                │
     ┌──────────▼──────────┐
     │ graph & timeline UI │
     └──────────┬──────────┘
                │
     ┌──────────▼──────────┐
     │ export (svg, png)   │
     └─────────────────────┘
```

---

## 🔋 Tech Stack

- **Language**: Rust (backend), TypeScript + Svelte (frontend)
- **Visualization**: D3.js, Graphviz, Plotly
- **CLI Interface**: `clap`
- **File I/O**: `serde`, `rayon`, `walkdir`

---

## 🚧 MVP Features to Build

- [ ] CLI: `netravue render <input.jsonl>`
- [ ] Parse output from `netrascan` / `netraflow`
- [ ] Render basic wallet profile timeline
- [ ] Graph viewer: edge/vertex style based on heuristics
- [ ] Export visual as `.svg`, `.png`

---

## 🧠 Future Features

- [ ] Web-based frontend (with WASM backend)
- [ ] Real-time socket feed from `netrascan`
- [ ] Timeline animation (wallet evolution over time)
- [ ] Trace heatmap viewer (frequency-based link coloring)
- [ ] "Investigate this wallet" button → run full trace
- [ ] Custom tagging + notebook mode
