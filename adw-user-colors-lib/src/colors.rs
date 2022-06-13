// SPDX-License-Identifier: MPL-2.0-only

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorOverrides {
    /// name
    pub name: String,
    pub accent_bg_color: Option<String>,
    pub accent_fg_color: Option<String>,
    pub accent_color: Option<String>,
    
    // destructive-action buttons
    pub destructive_bg_color: Option<String>,
    pub destructive_fg_color: Option<String>,
    pub destructive_color: Option<String>,
    
    // Levelbars, entries, labels and infobars. These don't need text colors
    pub success_color: Option<String>,
    pub warning_color: Option<String>,
    pub error_color: Option<String>,
    
    // Content areas, e.g. text views
    pub base_color: Option<String>,
    pub  text_color: Option<String>,
    
    // Main window background
    pub bg_color: Option<String>,
    pub fg_color: Option<String>,
    pub shade_color: Option<String>,
    
    // Header bar, search bar, tab bar
    pub headerbar_bg_color: Option<String>,
    pub headerbar_fg_color: Option<String>,
    pub headerbar_border_color: Option<String>,
    pub headerbar_backdrop_color: Option<String>,
    pub headerbar_shade_color: Option<String>,
    
    // Cards, boxed lists
    pub card_bg_color: Option<String>,
    pub card_fg_color: Option<String>,
    pub card_border_color: Option<String>,
    
    // Popovers
    pub popover_bg_color: Option<String>,
    pub popover_fg_color: Option<String>,
    
    // Miscellaneous
    pub scrollbar_outline_color: Option<String>,
    pub window_outline_color: Option<String>,
    pub window_border_color: Option<String>,
    pub window_border_backdrop_color: Option<String>,
}