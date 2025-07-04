use std::collections::{HashMap, VecDeque};

use common::model::{FlowSummary, TxCategory, Wei};

use crate::constants::ETH_THRESHOLDS;

#[derive(Debug)]
pub struct FlowDelta {
    pub block_number: u64,
    pub deltas: HashMap<TxCategory, Wei>,
}

#[derive(Debug)]
pub struct Alert {
    pub level: u8,
    pub reason: String,
    pub affected_category: TxCategory,
    pub delta_wei: Wei,
    pub block_number: u64,
}

impl Alert {
    pub fn report(&self) {
        let eth = self.delta_wei as f64 / 1e18;
        println!(
            "🚨 [Level {}] Block {}: {} → {eth:.2} ETH in {:?}",
            self.level, self.block_number, self.reason, self.affected_category
        );
    }
}

#[derive(Debug)]
pub struct FlowMonitor {
    window: VecDeque<FlowSummary>,
    max_blocks: usize,
    alert_thresholds: HashMap<TxCategory, Wei>,
}

impl FlowMonitor {
    pub fn new(max_blocks: usize) -> Self {
        Self {
            window: VecDeque::with_capacity(max_blocks),
            max_blocks,
            alert_thresholds: HashMap::from([(TxCategory::Foreign, ETH_THRESHOLDS)]),
        }
    }

    pub fn push(&mut self, new_summary: FlowSummary) -> (Option<FlowDelta>, Vec<Alert>) {
        let delta = self.compute_delta(&new_summary);
        let alerts = self.detect_alerts(&delta, new_summary.block_number);

        if self.window.len() == self.max_blocks {
            self.window.pop_front();
        }
        self.window.push_front(new_summary);

        (delta, alerts)
    }

    fn compute_delta(&self, current: &FlowSummary) -> Option<FlowDelta> {
        let prev = self.window.back()?;
        let mut deltas = HashMap::new();

        for (category, &curr_val) in &current.category_totals {
            let prev_val = prev.category_totals.get(category).copied().unwrap_or(0);
            let diff = curr_val.saturating_add(prev_val);
            deltas.insert(*category, diff);
        }

        Some(FlowDelta {
            block_number: current.block_number,
            deltas,
        })
    }

    fn detect_alerts(&self, delta: &Option<FlowDelta>, block_number: u64) -> Vec<Alert> {
        let mut alerts = vec![];
        let delta = match delta {
            Some(d) => d,
            None => return alerts,
        };

        for (category, &change) in &delta.deltas {
            if let Some(&threshold) = self.alert_thresholds.get(category) {
                if change >= threshold {
                    alerts.push(Alert {
                        level: 1,
                        reason: format!("High flow delta for {category:?}"),
                        affected_category: *category,
                        delta_wei: change,
                        block_number,
                    });
                }
            }
        }

        alerts
    }

    pub fn avg_flow(&self, category: &TxCategory) -> Option<Wei> {
        if self.window.is_empty() {
            return None;
        }

        let total: Wei = self
            .window
            .iter()
            .map(|summary| summary.category_totals.get(category).copied().unwrap_or(0))
            .sum();

        Some(total / self.window.len() as u128)
    }

    pub fn latest_block(&self) -> Option<u64> {
        self.window.back().map(|s| s.block_number)
    }

    pub fn print_summary(&self) {
        println!("Rolling window ({} blocks):", self.window.len());
        for summary in &self.window {
            println!(
                "Block {} | Foreign: {} | tx_count: {}",
                summary.block_number,
                summary
                    .category_totals
                    .get(&TxCategory::Foreign)
                    .unwrap_or(&0),
                summary.tx_count
            );
        }
    }
}
