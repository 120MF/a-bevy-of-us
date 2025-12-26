use bevy::prelude::*;

use crate::plugins::ui::button_builder::ButtonStyle;

/// 主题资源 - 全局UI样式配置
#[derive(Resource, Clone)]
pub struct Theme {
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub button: ButtonStyle,
}

/// 颜色调色板
#[derive(Clone, Debug)]
pub struct ColorPalette {
    // 主要颜色
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub surface: Color,

    // 文本颜色
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_disabled: Color,

    // 按钮颜色 (与ButtonStyle同步)
    pub button_normal: Color,
    pub button_hovered: Color,
    pub button_pressed: Color,
    pub button_focused: Color,
    pub button_selected: Color,
}

/// 排版系统
#[derive(Clone, Debug)]
pub struct Typography {
    pub font_path: String,

    // 字体大小
    pub size_h1: f32,
    pub size_h2: f32,
    pub size_h3: f32,
    pub size_body: f32,
    pub size_small: f32,

    // 行高
    pub line_height: f32,
}

/// 间距系统 - 使用8px基准的间距尺度
#[derive(Clone, Debug)]
pub struct Spacing {
    pub xs: f32,  // 4px
    pub sm: f32,  // 8px
    pub md: f32,  // 16px
    pub lg: f32,  // 24px
    pub xl: f32,  // 32px
    pub xxl: f32, // 48px
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// 深色主题 (默认)
    pub fn dark() -> Self {
        let colors = ColorPalette {
            primary: Color::srgb(0.3, 0.5, 0.8),
            secondary: Color::srgb(0.5, 0.3, 0.8),
            background: Color::srgb(0.1, 0.1, 0.1),
            surface: Color::srgb(0.15, 0.15, 0.15),

            text_primary: Color::WHITE,
            text_secondary: Color::srgb(0.7, 0.7, 0.7),
            text_disabled: Color::srgb(0.4, 0.4, 0.4),

            button_normal: Color::srgb(0.15, 0.15, 0.15),
            button_hovered: Color::srgb(0.25, 0.25, 0.25),
            button_pressed: Color::srgb(0.35, 0.75, 0.35),
            button_focused: Color::srgb(0.3, 0.3, 0.5),
            button_selected: Color::srgb(0.2, 0.5, 0.2),
        };

        Self {
            button: ButtonStyle {
                normal: colors.button_normal,
                hovered: colors.button_hovered,
                pressed: colors.button_pressed,
                focused: colors.button_focused,
                selected: colors.button_selected,
            },
            colors,
            typography: Typography {
                font_path: "fonts/AlibabaPuHuiTi-3-65-Medium.ttf".to_string(),
                size_h1: 48.0,
                size_h2: 36.0,
                size_h3: 28.0,
                size_body: 20.0,
                size_small: 16.0,
                line_height: 1.5,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
            },
        }
    }

    /// 浅色主题 (可选)
    pub fn light() -> Self {
        let colors = ColorPalette {
            primary: Color::srgb(0.2, 0.4, 0.7),
            secondary: Color::srgb(0.4, 0.2, 0.7),
            background: Color::srgb(0.95, 0.95, 0.95),
            surface: Color::WHITE,

            text_primary: Color::srgb(0.1, 0.1, 0.1),
            text_secondary: Color::srgb(0.3, 0.3, 0.3),
            text_disabled: Color::srgb(0.6, 0.6, 0.6),

            button_normal: Color::srgb(0.85, 0.85, 0.85),
            button_hovered: Color::srgb(0.75, 0.75, 0.75),
            button_pressed: Color::srgb(0.35, 0.75, 0.35),
            button_focused: Color::srgb(0.7, 0.7, 0.9),
            button_selected: Color::srgb(0.6, 0.9, 0.6),
        };

        Self {
            button: ButtonStyle {
                normal: colors.button_normal,
                hovered: colors.button_hovered,
                pressed: colors.button_pressed,
                focused: colors.button_focused,
                selected: colors.button_selected,
            },
            colors,
            typography: Typography {
                font_path: "fonts/AlibabaPuHuiTi-3-65-Medium.ttf".to_string(),
                size_h1: 48.0,
                size_h2: 36.0,
                size_h3: 28.0,
                size_body: 20.0,
                size_small: 16.0,
                line_height: 1.5,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
            },
        }
    }
}

/// 主题间距枚举 - 用于从Theme获取间距值
#[derive(Clone, Copy, Debug)]
pub enum ThemeSpacing {
    XS,
    SM,
    MD,
    LG,
    XL,
    XXL,
}

impl ThemeSpacing {
    pub fn get_value(&self, spacing: &Spacing) -> f32 {
        match self {
            ThemeSpacing::XS => spacing.xs,
            ThemeSpacing::SM => spacing.sm,
            ThemeSpacing::MD => spacing.md,
            ThemeSpacing::LG => spacing.lg,
            ThemeSpacing::XL => spacing.xl,
            ThemeSpacing::XXL => spacing.xxl,
        }
    }
}

/// 主题颜色枚举 - 用于从Theme获取主题颜色
#[derive(Clone, Copy, Debug)]
pub enum ThemedColor {
    Primary,
    Secondary,
    Background,
    Surface,
    TextPrimary,
    TextSecondary,
}

impl ThemedColor {
    pub fn get_color(&self, palette: &ColorPalette) -> Color {
        match self {
            ThemedColor::Primary => palette.primary,
            ThemedColor::Secondary => palette.secondary,
            ThemedColor::Background => palette.background,
            ThemedColor::Surface => palette.surface,
            ThemedColor::TextPrimary => palette.text_primary,
            ThemedColor::TextSecondary => palette.text_secondary,
        }
    }
}
