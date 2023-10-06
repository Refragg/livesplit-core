use crate::run::parser::livesplit::Version;
use core::fmt::Debug;
use livesplit_auto_splitting::{SettingValue, UserSettingKind};

#[derive(Debug, Clone, PartialEq)]
pub struct AutoSplitterSettings {
    pub version: Version,
    pub script_path: String,
    pub start: bool,
    pub reset: bool,
    pub split: bool,
    pub custom_settings: Vec<CustomSetting>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomSetting {
    pub id: String,
    pub setting_type: UserSettingKind,
    pub value: SettingValue,
}

impl AutoSplitterSettings {
    pub fn new() -> Self {
        Self {
            version: Version::default(),
            script_path: String::default(),
            start: false,
            reset: false,
            split: false,
            custom_settings: vec![],
        }
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn set_script_path(&mut self, script_path: String) {
        self.script_path = script_path;
    }

    pub fn set_start(&mut self, start: bool) {
        self.start = start;
    }

    pub fn set_reset(&mut self, reset: bool) {
        self.reset = reset;
    }

    pub fn set_split(&mut self, split: bool) {
        self.split = split;
    }

    pub fn add_custom_setting(&mut self, custom_setting: CustomSetting) {
        self.custom_settings.push(custom_setting);
    }
}

impl CustomSetting {
    pub fn new() -> Self {
        Self {
            id: String::default(),
            setting_type: UserSettingKind::Bool {
                default_value: false,
            },
            value: SettingValue::Bool(false),
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_setting_type(&mut self, setting_type: UserSettingKind) {
        self.setting_type = setting_type;
    }

    pub fn set_value(&mut self, value: SettingValue) {
        self.value = value
    }
}
