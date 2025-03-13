use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use std::{fs, process::Command};
use std::hash::{Hash, Hasher, DefaultHasher};
use urlencoding::encode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Engine {
    Google,
    Custom { name: String, url: String, secondary_prefix: String },
}

impl Engine {
    fn value(&self) -> &str {
        match self {
            Self::Google => "google.com/search?q={}",
            Self::Custom { url, .. } => url,
        }
    }
    fn secondary_prefix(&self) -> &str {
        match self {
            Self::Google => "",
            Self::Custom { secondary_prefix, .. } => secondary_prefix,
        }
    }
    fn name(&self) -> &str {
        match self {
            Self::Google => "Google",
            Self::Custom { name, .. } => name,
        }
    }
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.name().hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    prefix: String,
    engines: Vec<Engine>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: "?".to_string(),
            engines: vec![Engine::Google],
        }
    }
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/websearch-plus.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Websearch+".into(),
        icon: "distributor-logo-netrunner".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, config: &Config) -> RVec<Match> {
    if !input.starts_with(&config.prefix) {
        RVec::new()
    } else {
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().smart_case();

        config
            .engines
            .iter()
            .filter(|engine| {
                let query = input.strip_prefix(&config.prefix).expect("Unable to strip prefix from input lines");
                let default = query.starts_with(' ');

                // Not using a secondary prefix (default engine)
                (default && engine == &&Engine::Google)
                // Matches one of the engines
                || (matcher.fuzzy_match(
                    engine.secondary_prefix().to_lowercase().as_str(),
                    query.split_whitespace().next().unwrap_or("").to_lowercase().as_str()
                ).is_some() && !default)
            })
            .map(|engine| Match {
                title: input
                    .trim_start_matches(&config.prefix) // trim main prefix
                    .trim_start_matches(|c: char| !c.is_whitespace()) // trim secondary prefix
                    .trim_start() // trim whitespace left from previous trim
                    .into(),
                description: ROption::RSome(format!("Search with {}{}", engine.name(),
                    {
                        let s_prefix = engine.secondary_prefix();
                        if s_prefix.is_empty() {
                            String::new()
                        } else {
                            format!(" ({}{})", config.prefix, s_prefix)
                        }
                    }
                ).into()),
                use_pango: false,
                icon: ROption::RNone,
                id: ROption::RSome(engine.id()),
            })
            .collect()
    }
}

#[handler]
fn handler(selection: Match, config: &Config) -> HandleResult {

    let engine = config
        .engines
        .iter()
        .find(|engine| engine.id() == selection.id.unwrap())
        .unwrap();

    if let Err(why) = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "xdg-open https://{}",
            engine
                .value()
                .replace("{}", &encode(&selection.title.to_string()))
        ))
        .spawn()
    {
        println!("Failed to perform websearch-plus: {}", why);
    }

    HandleResult::Close
}
