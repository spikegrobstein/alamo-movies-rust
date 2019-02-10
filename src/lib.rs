use serde_json;
use std::fs;
use std::error::Error;

struct Cinema {
    data: serde_json::Value
}

impl Cinema {
    pub fn from_calendar_file(path: &str) -> Result<Cinema, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;

        let v: serde_json::Value = serde_json::from_str(&contents)?;

        Ok(Cinema {
            data: v["Calendar"]["Cinemas"][0]
        })
    }

    pub fn name(&self) -> Result<String, Box<dyn Error>> {
        let field = self.data["CinemaName"];

        let s = &field.as_str()?;

        Ok(s)
    }
}
