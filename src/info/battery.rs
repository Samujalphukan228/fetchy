use std::fs;

pub fn battery_status() -> Option<String> {
    let supply_dir = fs::read_dir("/sys/class/power_supply").ok()?;
    for entry in supply_dir.flatten() {
        let path = entry.path();
        let type_path = path.join("type");
        let type_raw = fs::read_to_string(&type_path).ok()?;
        if !type_raw.trim().eq_ignore_ascii_case("Battery") {
            continue;
        }

        let capacity = fs::read_to_string(path.join("capacity"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        let status = fs::read_to_string(path.join("status"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        return Some(match (capacity, status) {
            (Some(cap), Some(st)) => format!("{}% ({})", cap, st),
            (Some(cap), None) => format!("{}%", cap),
            (None, Some(st)) => st,
            (None, None) => "Present".to_string(),
        });
    }
    None
}