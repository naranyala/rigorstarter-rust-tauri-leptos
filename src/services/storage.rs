use serde::{Deserialize, Serialize};
use web_sys::window;

#[allow(dead_code)]
pub struct BrowserStorage;

#[allow(dead_code)]
impl BrowserStorage {
    pub fn set<T: Serialize>(key: &str, value: &T) -> Result<(), String> {
        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten())
            .ok_or("LocalStorage not available")?;

        let val_str = serde_json::to_string(value).map_err(|e| e.to_string())?;
        storage
            .set_item(key, &val_str)
            .map_err(|_| "Failed to set item".to_string())?;
        Ok(())
    }

    pub fn get<T: for<'de> Deserialize<'de>>(key: &str) -> Result<Option<T>, String> {
        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten())
            .ok_or("LocalStorage not available")?;

        match storage
            .get_item(key)
            .map_err(|_| "Failed to get item".to_string())?
        {
            Some(val_str) => {
                let val = serde_json::from_str(&val_str).map_err(|e| e.to_string())?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    pub fn remove(key: &str) -> Result<(), String> {
        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten())
            .ok_or("LocalStorage not available")?;
        storage
            .remove_item(key)
            .map_err(|_| "Failed to remove item".to_string())?;
        Ok(())
    }
}
