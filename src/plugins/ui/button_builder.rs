use crate::plugins::ui::components::{Focusable, Focused};
use crate::plugins::ui::navigation::{NavigationGraph, NavigationNeighbors};
use crate::plugins::ui::styles::Theme;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

/// Button style constants
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub const FOCUSED_BUTTON: Color = Color::srgb(0.3, 0.3, 0.5);
pub const SELECTED_BUTTON: Color = Color::srgb(0.2, 0.5, 0.2);

/// Button style configuration
#[derive(Clone, Debug)]
pub struct ButtonStyle {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub focused: Color,
    pub selected: Color,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            normal: NORMAL_BUTTON,
            hovered: HOVERED_BUTTON,
            pressed: PRESSED_BUTTON,
            focused: FOCUSED_BUTTON,
            selected: SELECTED_BUTTON,
        }
    }
}

/// Navigation layout type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationLayout {
    Vertical,
    Horizontal,
    Grid { columns: usize },
}

/// Builder for creating navigable buttons
pub struct ButtonNavigationBuilder {
    buttons: Vec<Entity>,
    layout: NavigationLayout,
}

impl ButtonNavigationBuilder {
    pub fn new(layout: NavigationLayout) -> Self {
        Self {
            buttons: Vec::new(),
            layout,
        }
    }

    /// Add a button to the builder
    pub fn add_button(&mut self, entity: Entity) {
        self.buttons.push(entity);
    }

    /// Build the navigation graph and set up initial focus
    pub fn build(
        self,
        commands: &mut Commands,
        nav_graph: &mut NavigationGraph,
        set_initial_focus: bool,
    ) {
        if self.buttons.is_empty() {
            return;
        }

        // Add Focusable to all buttons
        for &entity in &self.buttons {
            commands.entity(entity).insert(Focusable);
        }

        // Create navigation relationships based on layout
        match self.layout {
            NavigationLayout::Vertical => {
                for (i, &entity) in self.buttons.iter().enumerate() {
                    let neighbors = NavigationNeighbors::new()
                        .with_up(if i > 0 {
                            Some(self.buttons[i - 1])
                        } else {
                            None
                        })
                        .with_down(if i < self.buttons.len() - 1 {
                            Some(self.buttons[i + 1])
                        } else {
                            None
                        });
                    nav_graph.register_button(entity, neighbors);
                }
            }
            NavigationLayout::Horizontal => {
                for (i, &entity) in self.buttons.iter().enumerate() {
                    let neighbors = NavigationNeighbors::new()
                        .with_left(if i > 0 {
                            Some(self.buttons[i - 1])
                        } else {
                            None
                        })
                        .with_right(if i < self.buttons.len() - 1 {
                            Some(self.buttons[i + 1])
                        } else {
                            None
                        });
                    nav_graph.register_button(entity, neighbors);
                }
            }
            NavigationLayout::Grid { columns } => {
                for (i, &entity) in self.buttons.iter().enumerate() {
                    let row = i / columns;
                    let col = i % columns;

                    let up = if row > 0 {
                        let up_idx = i - columns;
                        if up_idx < self.buttons.len() {
                            Some(self.buttons[up_idx])
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    let down = if (row + 1) * columns < self.buttons.len() {
                        let down_idx = i + columns;
                        if down_idx < self.buttons.len() {
                            Some(self.buttons[down_idx])
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    let left = if col > 0 {
                        Some(self.buttons[i - 1])
                    } else {
                        None
                    };
                    let right = if col < columns - 1 && i + 1 < self.buttons.len() {
                        Some(self.buttons[i + 1])
                    } else {
                        None
                    };

                    let neighbors = NavigationNeighbors::new()
                        .with_up(up)
                        .with_down(down)
                        .with_left(left)
                        .with_right(right);
                    nav_graph.register_button(entity, neighbors);
                }
            }
        }

        // Set initial focus
        if set_initial_focus {
            if let Some(&first_button) = self.buttons.first() {
                commands.entity(first_button).insert(Focused);
                nav_graph.set_focus(first_button);
            }
        }
    }
}

/// Creates a standard button node
pub fn create_button_node(width: f32, height: f32, margin: f32) -> Node {
    Node {
        width: px(width),
        height: px(height),
        margin: UiRect::all(px(margin)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

/// Helper function to spawn a button with standard components
#[deprecated(note = "Use spawn_button_sized or spawn_themed_button instead")]
pub fn spawn_button<T: Component>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    text: &str,
    action: T,
    font: &TextFont,
    node: Node,
) -> Entity {
    let button_entity = parent
        .spawn((Button, node, BackgroundColor(NORMAL_BUTTON), action))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                font.clone(),
                TextLayout::new_with_justify(Justify::Center),
            ));
        })
        .id();
    button_entity
}

// === 主题化按钮系统 ===

/// 按钮尺寸预设
#[derive(Clone, Copy, Debug)]
pub enum ButtonSize {
    Small,  // 200x40
    Medium, // 250x50
    Large,  // 250x65
    Custom(f32, f32),
}

impl ButtonSize {
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            ButtonSize::Small => (200.0, 40.0),
            ButtonSize::Medium => (250.0, 50.0),
            ButtonSize::Large => (250.0, 65.0),
            ButtonSize::Custom(w, h) => (*w, *h),
        }
    }
}

/// 增强的按钮spawn函数，从主题获取样式
pub fn spawn_themed_button<T: Component>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    text: &str,
    action: T,
    theme: &Theme,
    asset_server: &AssetServer,
    width: f32,
    height: f32,
) -> Entity {
    let font = TextFont {
        font: asset_server.load(&theme.typography.font_path),
        font_size: theme.typography.size_body,
        ..default()
    };

    let node = Node {
        width: Val::Px(width),
        height: Val::Px(height),
        margin: UiRect::all(Val::Px(theme.spacing.sm)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    parent
        .spawn((
            Button,
            node,
            BackgroundColor(theme.button.normal),
            action,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                font,
                TextColor(theme.colors.text_primary),
                TextLayout::new_with_justify(Justify::Center),
            ));
        })
        .id()
}

/// 使用预设尺寸spawn按钮
pub fn spawn_button_sized<T: Component>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    text: &str,
    action: T,
    theme: &Theme,
    asset_server: &AssetServer,
    size: ButtonSize,
) -> Entity {
    let (width, height) = size.dimensions();
    spawn_themed_button(parent, text, action, theme, asset_server, width, height)
}
