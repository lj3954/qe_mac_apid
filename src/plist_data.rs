use std::collections::BTreeMap;

use plist::Value;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MacPlist {
    platform_info: PlatformInfo,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

impl MacPlist {
    pub fn debug(&self) {
        log::debug!("{:#?}", self);
    }

    pub fn get_product_name(&self) -> &str {
        &self.platform_info.generic.system_product_name
    }

    pub fn set_serial_number(&mut self, serial_number: String) {
        self.platform_info.generic.system_serial_number = serial_number;
    }

    pub fn set_mlb(&mut self, mlb: String) {
        self.platform_info.generic.mlb = mlb;
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.platform_info.generic.system_uuid = uuid.to_string();
    }

    pub fn set_rom(&mut self, rom: [u8; 12]) {
        self.platform_info.generic.rom = Value::Data(rom.to_vec());
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformInfo {
    generic: Generic,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Generic {
    #[serde(rename = "MLB")]
    mlb: String,
    #[serde(rename = "ROM")]
    rom: Value,
    system_product_name: String,
    system_serial_number: String,
    #[serde(rename = "SystemUUID")]
    system_uuid: String,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}
