#[derive(Clone, Debug, PartialEq)]
pub struct UpdateInfo {
    pub has_update: bool,
    pub latest_version: Option<String>,
    pub release_url: Option<String>,
    pub update_level: Option<String>, 
}

pub async fn check_update(current_version: &str) -> Option<UpdateInfo> {
    let user_agent = format!("freya-mid-app/{}", current_version);

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(_) => return None,
    };

    let response = match client
        .get("https://api.github.com/repos/mocikadev/freya-mid-app/releases/latest")
        .header("User-Agent", user_agent)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return None,
    };

    if !response.status().is_success() {
        return None;
    }

    let json: serde_json::Value = match response.json().await {
        Ok(j) => j,
        Err(_) => return None,
    };

    let tag = json["tag_name"].as_str().unwrap_or("");
    let latest = tag.trim_start_matches(|c: char| c == 'v' || c == 'V');
    let release_url = json["html_url"].as_str();

    let parse = |v: &str| -> Option<(u64, u64, u64)> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        Some((parts[0].parse().ok()?, parts[1].parse().ok()?, parts[2].parse().ok()?))
    };

    let (cur_maj, cur_min, cur_pat) = match parse(current_version) {
        Some(v) => v,
        None => return None,
    };
    let (lat_maj, lat_min, lat_pat) = match parse(latest) {
        Some(v) => v,
        None => return None,
    };

    let level = if lat_maj > cur_maj {
        "major"
    } else if lat_maj == cur_maj && lat_min > cur_min {
        "minor"
    } else if lat_maj == cur_maj && lat_min == cur_min && lat_pat > cur_pat {
        "patch"
    } else {
        return None;
    };

    Some(UpdateInfo {
        has_update: true,
        latest_version: Some(latest.to_string()),
        release_url: release_url.map(|s| s.to_string()),
        update_level: Some(level.to_string()),
    })
}
