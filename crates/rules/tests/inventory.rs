use resourceville_rules::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn inventory_sources_totals_and_scenario_contracts() {
    let inventory: Value =
        serde_json::from_str(include_str!("../../../docs/rules-inventory.json")).unwrap();
    let sources: Value =
        serde_json::from_str(include_str!("../../../docs/rules-sources.json")).unwrap();
    let sources: BTreeSet<_> = sources
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            assert_eq!(s["sha256"].as_str().unwrap().len(), 64);
            assert!(
                s["url"]
                    .as_str()
                    .unwrap()
                    .starts_with("https://www.catan.com/")
            );
            s["id"].as_str().unwrap()
        })
        .collect();
    let mut ids = BTreeSet::new();
    let mut totals = BTreeMap::new();
    for entry in inventory["cards"]
        .as_array()
        .unwrap()
        .iter()
        .chain(inventory["scenarios"].as_array().unwrap())
    {
        assert!(ids.insert(entry["id"].as_str().unwrap()));
        assert!(sources.contains(entry["source"].as_str().unwrap()));
        assert!((3..=17).contains(&entry["milestone"].as_u64().unwrap()));
        assert_eq!(entry["status"], "contract_only");
        assert!(!entry["pages"].as_str().unwrap().is_empty());
        if let Some(deck) = entry.get("deck") {
            *totals.entry(deck.as_str().unwrap()).or_insert(0) += entry["copies"].as_u64().unwrap();
        } else {
            let scenario: Scenario = serde_json::from_value(entry["id"].clone()).unwrap();
            let ruleset: Ruleset = serde_json::from_value(entry["ruleset"].clone()).unwrap();
            assert_eq!(scenario.ruleset(), ruleset);
            GameConfig {
                scenario,
                ruleset,
                ..Default::default()
            }
            .validate()
            .unwrap();
        }
    }
    assert_eq!(inventory["scenarios"].as_array().unwrap().len(), 21);
    assert_eq!(inventory["cards"].as_array().unwrap().len(), 38);
    assert_eq!(
        totals,
        BTreeMap::from([
            ("base", 25),
            ("science", 18),
            ("trade", 18),
            ("politics", 18),
            ("siege", 26),
            ("freight", 25)
        ])
    );
}
