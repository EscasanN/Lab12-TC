use std::collections::HashMap;

fn map_of(make: &str, model: i64, color: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("make".into(), make.into());
    m.insert("model".into(), model.to_string());
    m.insert("color".into(), color.into());
    m
}

fn sort_by_key(mut items: Vec<HashMap<String, String>>, key: &str) -> Vec<HashMap<String, String>> {
    items.sort_by(|a, b| {
        let va = a.get(key).map(|s| s.as_str()).unwrap_or("");
        let vb = b.get(key).map(|s| s.as_str()).unwrap_or("");
        match (va.parse::<i64>(), vb.parse::<i64>()) {
            (Ok(na), Ok(nb)) => na.cmp(&nb),
            _ => va.cmp(vb),
        }
    });
    items
}

fn main() {
    let data = vec![
        map_of("Nokia", 216, "Black"),
        map_of("Mi Max", 2, "Gold"),
        map_of("Samsung", 7, "Blue"),
    ];

    let sorted = sort_by_key(data, "model");
    for d in &sorted {
        println!("{{ make: {}, model: {}, color: {} }}",
            d.get("make").unwrap(),
            d.get("model").unwrap(),
            d.get("color").unwrap()
        );
    }
}
