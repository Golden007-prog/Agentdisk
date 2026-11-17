//! `agentdisk check`: a fast health check for hooks (no deep scan).

use crate::adapters::Kind;
use crate::json::Value;
use crate::paths;
use crate::scan;
use crate::size::human;
use crate::sys;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Health {
    pub free: Option<u64>,
    pub total: Option<u64>,
    pub worktrees: Vec<(String, usize)>,
    pub warnings: Vec<String>,
}

pub fn run(min_free_gb: u64, max_worktrees: usize, adapter_dirs: &[PathBuf]) -> Health {
    let (adapters, _) = scan::load_adapters(adapter_dirs);
    let disk = sys::disk_space(&paths::home());
    let mut per_agent = Vec::new();
    for a in &adapters {
        let n: usize = a
            .paths
            .iter()
            .filter(|p| p.kind == Kind::Worktree && p.applies_here())
            .map(|p| paths::expand(&p.template).len())
            .sum();
        if n > 0 {
            per_agent.push((a.name.clone(), n));
        }
    }
    let mut warnings = Vec::new();
    if let Some((free, _)) = disk {
        if let Some(warning) = low_space_warning(free, min_free_gb) {
            warnings.push(warning);
        }
    }
    let total: usize = per_agent.iter().map(|x| x.1).sum();
    if total > max_worktrees {
        warnings.push(format!(
            "{total} agent worktrees on disk. Run `agentdisk plan` to see which are safe to clean."
        ));
    }
    Health {
        free: disk.map(|d| d.0),
        total: disk.map(|d| d.1),
        worktrees: per_agent,
        warnings,
    }
}

fn low_space_warning(free: u64, min_free_gb: u64) -> Option<String> {
    (free < min_free_gb.saturating_mul(1024 * 1024 * 1024)).then(|| {
        format!(
            "Low disk space: {} free. Run `agentdisk scan` to see what agents left behind.",
            human(free)
        )
    })
}

pub fn render(h: &Health) -> String {
    let mut out = String::new();
    if h.warnings.is_empty() {
        out.push_str("agentdisk: OK");
        if let Some(f) = h.free {
            out.push_str(&format!(" ({} free)", human(f)));
        }
        out.push('\n');
    } else {
        for w in &h.warnings {
            out.push_str(&format!("agentdisk: {w}\n"));
        }
    }
    out
}

pub fn to_json(h: &Health) -> Value {
    Value::obj(vec![
        ("free_bytes", h.free.into()),
        ("total_bytes", h.total.into()),
        (
            "agent_worktrees",
            Value::Obj(
                h.worktrees
                    .iter()
                    .map(|(n, c)| (n.clone(), Value::from(*c)))
                    .collect(),
            ),
        ),
        (
            "warnings",
            h.warnings
                .iter()
                .map(|w| Value::from(w.as_str()))
                .collect::<Vec<_>>()
                .into(),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huge_free_space_thresholds_still_warn() {
        for min_free_gb in [1 << 34, u64::MAX] {
            assert_eq!(
                low_space_warning(10 * 1024 * 1024 * 1024, min_free_gb),
                Some("Low disk space: 10.00 GiB free. Run `agentdisk scan` to see what agents left behind.".into())
            );
        }
    }

    #[test]
    fn ordinary_free_space_thresholds_keep_their_boundary() {
        let gib = 1024 * 1024 * 1024;
        assert!(low_space_warning(19 * gib, 20).is_some());
        assert_eq!(low_space_warning(20 * gib, 20), None);
        assert_eq!(low_space_warning(21 * gib, 20), None);
        assert_eq!(low_space_warning(0, 0), None);
    }
}
