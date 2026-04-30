//! UI language preference (D13).
//!
//! Disk shape: top-level TOML key `language = "zh-CN" | "en" | "system"`. Missing value
//! is mapped to `Language::default() == Zh` per fork policy.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp_core::settings::{
    RespectUserSyncSetting, SupportedPlatforms, SyncToCloud, macros::define_settings_group,
};
use warpui::SingletonEntity;

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    settings_value::SettingsValue,
)]
pub enum Language {
    #[default]
    #[serde(rename = "zh-CN")]
    Zh,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "system")]
    System,
}

impl Language {
    pub fn resolve_locale(self) -> warp_i18n::Locale {
        match self {
            Language::Zh => warp_i18n::Locale::ZhCn,
            Language::En => warp_i18n::Locale::En,
            Language::System => warp_i18n::detect_system_locale()
                .as_deref()
                .and_then(warp_i18n::Locale::parse_bcp47)
                .unwrap_or(warp_i18n::Locale::En),
        }
    }
}

define_settings_group!(LanguageSettings, settings: [
    language: LanguageSetting {
        type: Language,
        default: Language::Zh,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "language",
        description: "UI language preference. \"zh-CN\" or \"en\" forces a locale; \"system\" follows the host OS.",
    },
]);

/// Apply the current [`Language`] setting to [`warp_i18n`] and re-apply on every
/// settings change. On macOS, also triggers a main menu rebuild so AppKit labels
/// reflect the new locale.
pub fn bind_to_warp_i18n(ctx: &mut warpui::AppContext) {
    let initial = LanguageSettings::as_ref(ctx).language.resolve_locale();
    warp_i18n::set_locale(initial);

    ctx.subscribe_to_model(&LanguageSettings::handle(ctx), |_, _event, ctx| {
        let locale = LanguageSettings::as_ref(ctx).language.resolve_locale();
        warp_i18n::set_locale(locale);
        ctx.invalidate_all_views();
        #[cfg(target_os = "macos")]
        rebuild_main_menu_if_ready();
    });
}

#[cfg(target_os = "macos")]
fn rebuild_main_menu_if_ready() {
    if warp_i18n::try_global().is_some() {
        warpui::platform::mac::rebuild_main_menu();
    }
}
