use bevy::ecs::relationship::{RelatedSpawnerCommands};
use bevy::ecs::hierarchy::ChildOf;
use bevy::prelude::*;

use crate::plugins::ui::styles::{Theme, ThemedColor, ThemeSpacing};

/// 容器Builder - 用于构建布局容器
pub struct ContainerBuilder {
    node: Node,
    background_color: Option<Color>,
}

impl ContainerBuilder {
    /// 创建新的容器Builder
    pub fn new() -> Self {
        Self {
            node: Node::default(),
            background_color: None,
        }
    }

    /// 创建全屏根容器 (100% 宽高，垂直居中)
    pub fn root() -> Self {
        Self {
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: None,
        }
    }

    // === 布局方向 ===

    pub fn vertical(mut self) -> Self {
        self.node.flex_direction = FlexDirection::Column;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.node.flex_direction = FlexDirection::Row;
        self
    }

    // === 对齐方式 ===

    pub fn centered(mut self) -> Self {
        self.node.align_items = AlignItems::Center;
        self.node.justify_content = JustifyContent::Center;
        self
    }

    pub fn align_start(mut self) -> Self {
        self.node.align_items = AlignItems::FlexStart;
        self
    }

    pub fn align_end(mut self) -> Self {
        self.node.align_items = AlignItems::FlexEnd;
        self
    }

    pub fn justify_start(mut self) -> Self {
        self.node.justify_content = JustifyContent::FlexStart;
        self
    }

    pub fn justify_end(mut self) -> Self {
        self.node.justify_content = JustifyContent::FlexEnd;
        self
    }

    pub fn justify_space_between(mut self) -> Self {
        self.node.justify_content = JustifyContent::SpaceBetween;
        self
    }

    // === 尺寸设置 ===

    pub fn width(mut self, width: Val) -> Self {
        self.node.width = width;
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.node.height = height;
        self
    }

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }

    // === 间距设置 (使用主题间距) ===

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.node.padding = padding;
        self
    }

    pub fn padding_all(mut self, value: f32) -> Self {
        self.node.padding = UiRect::all(Val::Px(value));
        self
    }

    pub fn padding_themed(mut self, theme_spacing: ThemeSpacing, theme: &Theme) -> Self {
        let value = theme_spacing.get_value(&theme.spacing);
        self.node.padding = UiRect::all(Val::Px(value));
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.node.margin = margin;
        self
    }

    pub fn margin_all(mut self, value: f32) -> Self {
        self.node.margin = UiRect::all(Val::Px(value));
        self
    }

    pub fn margin_themed(mut self, theme_spacing: ThemeSpacing, theme: &Theme) -> Self {
        let value = theme_spacing.get_value(&theme.spacing);
        self.node.margin = UiRect::all(Val::Px(value));
        self
    }

    pub fn gap(mut self, gap: Val) -> Self {
        self.node.row_gap = gap;
        self.node.column_gap = gap;
        self
    }

    pub fn gap_themed(mut self, theme_spacing: ThemeSpacing, theme: &Theme) -> Self {
        let value = theme_spacing.get_value(&theme.spacing);
        self.node.row_gap = Val::Px(value);
        self.node.column_gap = Val::Px(value);
        self
    }

    // === 背景颜色 ===

    pub fn background(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn background_themed(mut self, themed_color: ThemedColor, theme: &Theme) -> Self {
        self.background_color = Some(themed_color.get_color(&theme.colors));
        self
    }

    // === 构建方法 ===

    /// Spawn容器并返回Entity，支持闭包添加子元素
    pub fn spawn(self, commands: &mut Commands, build_children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>)) -> Entity
    {
        let mut entity_commands = commands.spawn(self.node);

        if let Some(bg_color) = self.background_color {
            entity_commands.insert(BackgroundColor(bg_color));
        }

        entity_commands.with_children(build_children);

        entity_commands.id()
    }

    /// Spawn容器并添加额外组件
    pub fn spawn_with<B: Bundle>(
        self,
        commands: &mut Commands,
        bundle: B,
        build_children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
    ) -> Entity
    {
        let mut entity_commands = commands.spawn((self.node, bundle));

        if let Some(bg_color) = self.background_color {
            entity_commands.insert(BackgroundColor(bg_color));
        }

        entity_commands.with_children(build_children);

        entity_commands.id()
    }

    /// Spawn容器但不添加子元素
    pub fn spawn_empty(self, commands: &mut Commands) -> Entity {
        let mut entity_commands = commands.spawn(self.node);

        if let Some(bg_color) = self.background_color {
            entity_commands.insert(BackgroundColor(bg_color));
        }

        entity_commands.id()
    }

    /// Spawn容器和额外组件，但不添加子元素
    pub fn spawn_empty_with<B: Bundle>(self, commands: &mut Commands, bundle: B) -> Entity {
        let mut entity_commands = commands.spawn((self.node, bundle));

        if let Some(bg_color) = self.background_color {
            entity_commands.insert(BackgroundColor(bg_color));
        }

        entity_commands.id()
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// === 文本系统 ===

/// 文本级别枚举
#[derive(Clone, Copy, Debug)]
pub enum TextLevel {
    H1,    // 大标题
    H2,    // 次标题
    H3,    // 小标题
    Body,  // 正文
    Small, // 小文本
}

/// 创建文本组件元组
pub fn text_styled(
    text: impl Into<String>,
    level: TextLevel,
    theme: &Theme,
    asset_server: &AssetServer,
) -> (Text, TextFont, TextColor, TextLayout) {
    let font_size = match level {
        TextLevel::H1 => theme.typography.size_h1,
        TextLevel::H2 => theme.typography.size_h2,
        TextLevel::H3 => theme.typography.size_h3,
        TextLevel::Body => theme.typography.size_body,
        TextLevel::Small => theme.typography.size_small,
    };

    (
        Text::new(text.into()),
        TextFont {
            font: asset_server.load(&theme.typography.font_path),
            font_size,
            ..default()
        },
        TextColor(theme.colors.text_primary),
        TextLayout::new_with_justify(Justify::Center),
    )
}

/// 创建带自定义颜色的文本
pub fn text_styled_colored(
    text: impl Into<String>,
    level: TextLevel,
    color: Color,
    theme: &Theme,
    asset_server: &AssetServer,
) -> (Text, TextFont, TextColor, TextLayout) {
    let font_size = match level {
        TextLevel::H1 => theme.typography.size_h1,
        TextLevel::H2 => theme.typography.size_h2,
        TextLevel::H3 => theme.typography.size_h3,
        TextLevel::Body => theme.typography.size_body,
        TextLevel::Small => theme.typography.size_small,
    };

    (
        Text::new(text.into()),
        TextFont {
            font: asset_server.load(&theme.typography.font_path),
            font_size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Center),
    )
}

// === 便捷工具 ===

/// 创建固定高度的分隔符
pub fn spacer(height: f32) -> Node {
    Node {
        height: Val::Px(height),
        ..default()
    }
}

/// 创建使用主题间距的分隔符
pub fn spacer_themed(theme: &Theme, spacing: ThemeSpacing) -> Node {
    Node {
        height: Val::Px(spacing.get_value(&theme.spacing)),
        ..default()
    }
}

/// 创建固定宽度的水平分隔符
pub fn h_spacer(width: f32) -> Node {
    Node {
        width: Val::Px(width),
        ..default()
    }
}

/// 创建使用主题间距的水平分隔符
pub fn h_spacer_themed(theme: &Theme, spacing: ThemeSpacing) -> Node {
    Node {
        width: Val::Px(spacing.get_value(&theme.spacing)),
        ..default()
    }
}
