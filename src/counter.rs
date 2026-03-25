use serde::{Deserialize, Serialize};
use std::fmt;

pub const SAVE_FILE: &str = "counters.json";

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Category {
    Daily,
    Weekly,
    #[default]
    Unlimited,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Category::Daily => "Daily",
            Category::Weekly => "Weekly",
            Category::Unlimited => "Unlimited",
        };
        write!(f, "{}", label)
    }
}

impl Category {
    pub fn max_count(&self) -> Option<i32> {
        match self {
            Category::Daily => Some(10),
            Category::Weekly => Some(50),
            Category::Unlimited => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub name: String,
    pub count: i32,
    pub category: Category,
}

impl Counter {
    pub fn new(name: String, category: Category) -> Self {
        Self { name, count: 0, category }
    }

    pub fn increment(&mut self) {
        let new_count = self.count.saturating_add(1);
        match self.category.max_count() {
            Some(max) => self.count = new_count.min(max),
            None => self.count = new_count,
        }
    }

    pub fn decrement(&mut self) {
        self.count = self.count.saturating_sub(1);
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let limit = match self.category.max_count() {
            Some(max) => format!("/{}", max),
            None => String::new(),
        };
        write!(f, "[{}] {}: {}{}", self.category, self.name, self.count, limit)
    }
}

pub fn save(counters: &[Counter]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(counters)?;
    std::fs::write(SAVE_FILE, json)?;
    Ok(())
}

pub fn load() -> Result<Vec<Counter>, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(SAVE_FILE)?;
    let counters = serde_json::from_str(&json)?;
    Ok(counters)
}
