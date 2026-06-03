mod desktop_entry;

use desktop_entry::find_apps;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).map(|s| s.as_str()).unwrap_or("");

    let apps = find_apps();
    
    if query.is_empty() {
        // Return all apps sorted by name if no query
        println!("{}", serde_json::to_string(&apps).unwrap_or_default());
        return;
    }

    let matcher = SkimMatcherV2::default();
    
    let mut results: Vec<_> = apps
        .into_iter()
        .filter_map(|app| {
            // Check both name and exec for matches
            let name_score = matcher.fuzzy_match(&app.name, query).unwrap_or(0);
            let exec_score = matcher.fuzzy_match(&app.exec, query).unwrap_or(0);
            
            let score = name_score.max(exec_score);
            if score > 0 {
                Some((score, app))
            } else {
                None
            }
        })
        .collect();

    // Sort by score descending
    results.sort_by(|(score_a, _), (score_b, _)| score_b.cmp(score_a));

    let final_apps: Vec<_> = results
        .into_iter()
        .take(10) // Limit to top 10 for performance
        .map(|(_, app)| app)
        .collect();

    println!("{}", serde_json::to_string(&final_apps).unwrap_or_default());
}