use std::fmt;

use woothee::parser::{Parser, WootheeResult};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Platform {
    FirefoxDesktop,
    Fenix,
    FirefoxIOS,
    #[default]
    Other,
}

impl fmt::Display for Platform {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        write!(fmt, "{}", name)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum DeviceFamily {
    Desktop,
    Mobile,
    Tablet,
    #[default]
    Other,
}

impl fmt::Display for DeviceFamily {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        write!(fmt, "{}", name)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum OsFamily {
    Windows,
    MacOs,
    Linux,
    IOS,
    Android,
    #[default]
    Other,
}

impl fmt::Display for OsFamily {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        write!(fmt, "{}", name)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct DeviceInfo {
    pub platform: Platform,
    pub device_family: DeviceFamily,
    pub os_family: OsFamily,
    pub firefox_version: u32,
}

impl DeviceInfo {
    /// Determine if the device is a desktop device based on either the form factor or OS.
    pub fn is_desktop(&self) -> bool {
        matches!(&self.device_family, DeviceFamily::Desktop)
            || matches!(
                &self.os_family,
                OsFamily::MacOs | OsFamily::Windows | OsFamily::Linux
            )
    }

    /// Determine if the device is a mobile phone based on either the form factor or OS.
    pub fn is_mobile(&self) -> bool {
        matches!(&self.device_family, DeviceFamily::Mobile)
            && matches!(&self.os_family, OsFamily::Android | OsFamily::IOS)
    }

    /// Determine if the device is iOS based on either the form factor or OS.
    pub fn is_ios(&self) -> bool {
        matches!(&self.device_family, DeviceFamily::Mobile)
            && matches!(&self.os_family, OsFamily::IOS)
    }

    /// Determine if the device is an android (Fenix) device based on either the form factor or OS.
    pub fn is_fenix(&self) -> bool {
        matches!(&self.device_family, DeviceFamily::Mobile)
            && matches!(&self.os_family, OsFamily::Android)
    }
}

/// Parses user agents from headers and returns a DeviceInfo struct containing
/// DeviceFamily, OsFamily, Platform, and Firefox Version.
///
/// Intended to handle standard user agent strings but also accomodates the non-standard,
/// Firefox-specific user agents for iOS and desktop.
///
/// It is theoretically possible to have an invalid user agent that is non-Firefox in the
/// case of an invalid UA, bot, or scraper.
/// There is a check for this to return an empty result as opposed to failing.
///
/// Parsing logic for non-standard iOS strings are in the form Firefox-iOS-FxA/24 and
/// manually modifies WootheeResult to match with correct enums for iOS platform and OS.
/// FxSync/<...>.desktop result still parses natively with Woothee and doesn't require intervention.
pub fn get_device_info(user_agent: &str) -> DeviceInfo {
    let mut w_result: WootheeResult<'_> = Parser::new().parse(user_agent).unwrap_or_default();

    // Current Firefox-iOS logic outputs the `user_agent` in the following formats:
    // Firefox-iOS-Sync/108.1b24234 (iPad; iPhone OS 16.4.1) (Firefox)
    // OR
    // Firefox-iOS-FxA/24
    // Both contain prefix `Firefox-iOS` and are not successfully parsed by Woothee.
    // This custom logic accomodates the current state (Q4 - 2024)
    // This may be a discussion point for future client-side adjustment to have a more standardized
    // user_agent string.
    if user_agent.to_lowercase().starts_with("firefox-ios") {
        w_result.name = "firefox";
        w_result.category = "smartphone";
        w_result.os = "iphone";
    }

    // NOTE: Firefox on iPads report back the Safari "desktop" UA
    // (e.g. `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/605.1.15
    //        (KHTML, like Gecko) Version/13.1 Safari/605.1.15)`
    // therefore we have to accept that one. This does mean that we may presume
    // that a mac safari UA is an iPad.
    if w_result.name.to_lowercase() == "safari" && !user_agent.to_lowercase().contains("firefox/") {
        w_result.name = "firefox";
        w_result.category = "smartphone";
        w_result.os = "ipad";
    }

    // Check if the user agent is not Firefox and return empty.
    if !["firefox"].contains(&w_result.name.to_lowercase().as_str()) {
        return DeviceInfo::default();
    }

    let os = w_result.os.to_lowercase();
    let os_family = match os.as_str() {
        _ if os.starts_with("windows") => OsFamily::Windows,
        "mac osx" => OsFamily::MacOs,
        "linux" => OsFamily::Linux,
        "iphone" | "ipad" => OsFamily::IOS,
        "android" => OsFamily::Android,
        _ => OsFamily::Other,
    };

    let device_family = match w_result.category {
        "pc" => DeviceFamily::Desktop,
        "smartphone" if os.as_str() == "ipad" => DeviceFamily::Tablet,
        "smartphone" => DeviceFamily::Mobile,
        _ => DeviceFamily::Other,
    };

    let platform = match device_family {
        DeviceFamily::Desktop => Platform::FirefoxDesktop,
        DeviceFamily::Mobile => match os_family {
            OsFamily::IOS => Platform::FirefoxIOS,
            OsFamily::Android => Platform::Fenix,
            _ => Platform::Other,
        },
        DeviceFamily::Tablet => match os_family {
            OsFamily::IOS => Platform::FirefoxIOS,
            _ => Platform::Other,
        },
        DeviceFamily::Other => Platform::Other,
    };

    let firefox_version = w_result
        .version
        .split('.')
        .next()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    DeviceInfo {
        platform,
        device_family,
        os_family,
        firefox_version,
    }
}
