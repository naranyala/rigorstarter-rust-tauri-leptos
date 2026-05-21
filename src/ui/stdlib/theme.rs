/// Design tokens for the application.
/// These match the CSS variables defined in your global styles.
pub struct Theme;

impl Theme {
    // Colors
    pub const PRIMARY: &'static str = "var(--primary)";
    pub const SECONDARY: &'static str = "var(--secondary-bg)";
    pub const BACKGROUND: &'static str = "var(--bg-color)";
    pub const TEXT_MAIN: &'static str = "var(--text-main)";
    pub const TEXT_MUTED: &'static str = "var(--text-muted)";
    pub const BORDER: &'static str = "var(--border-color)";

    // Spacing
    pub const SPACING_SM: &'static str = "0.5rem";
    pub const SPACING_MD: &'static str = "1rem";
    pub const SPACING_LG: &'static str = "2rem";

    // Shadows
    pub const SHADOW_SM: &'static str = "var(--shadow-sm)";
    pub const SHADOW_MD: &'static str = "var(--shadow-md)";
    pub const SHADOW_LG: &'static str = "var(--shadow-lg)";
}
