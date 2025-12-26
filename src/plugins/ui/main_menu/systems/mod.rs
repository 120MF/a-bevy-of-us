use crate::plugins::ui::button_builder::{ButtonNavigationBuilder, ButtonSize, NavigationLayout};
use crate::plugins::ui::main_menu::components::{MainMenuButtonAction, OnMainMenuScreen};
use crate::plugins::ui::navigation::NavigationGraph;
use crate::plugins::ui::overlays::{OverlayAction, OverlayMessage};
use crate::plugins::ui::styles::{Theme, ThemeSpacing};
use crate::plugins::ui::ui_builders::{text_styled, ContainerBuilder, TextLevel};
use crate::state::{GameState, OverlayState};
use bevy::prelude::*;

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme: Res<Theme>,
    mut nav_graph: ResMut<NavigationGraph>,
) {
    commands.spawn((Camera2d, OnMainMenuScreen));

    // Clear previous navigation graph
    nav_graph.clear();

    // Create button builder
    let mut button_builder = ButtonNavigationBuilder::new(NavigationLayout::Vertical);

    ContainerBuilder::root()
        .gap_themed(ThemeSpacing::MD, &theme)
        .spawn_with(
            &mut commands,
            OnMainMenuScreen,
            |parent| {
                // 游戏标题
                parent.spawn(text_styled("Cat Blast", TextLevel::H1, &theme, &asset_server));

                // 创建按钮并添加到 builder
                let play_button = crate::plugins::ui::button_builder::spawn_button_sized(
                    parent,
                    "开始游戏",
                    MainMenuButtonAction::Play,
                    &theme,
                    &asset_server,
                    ButtonSize::Large,
                );
                button_builder.add_button(play_button);

                let settings_button = crate::plugins::ui::button_builder::spawn_button_sized(
                    parent,
                    "设置",
                    MainMenuButtonAction::Settings,
                    &theme,
                    &asset_server,
                    ButtonSize::Large,
                );
                button_builder.add_button(settings_button);

                let quit_button = crate::plugins::ui::button_builder::spawn_button_sized(
                    parent,
                    "退出",
                    MainMenuButtonAction::Quit,
                    &theme,
                    &asset_server,
                    ButtonSize::Large,
                );
                button_builder.add_button(quit_button);
            },
        );

    // Build navigation graph and set initial focus
    button_builder.build(&mut commands, &mut nav_graph, true);
}

pub fn button_interaction_system(
    q_interaction: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut ns_game: ResMut<NextState<GameState>>,
    mut mw_overlay: MessageWriter<OverlayMessage>,
    mut app_exit_mw: MessageWriter<AppExit>,
) {
    for (interaction, action) in &q_interaction {
        if *interaction == Interaction::Pressed {
            match action {
                MainMenuButtonAction::Play => {
                    info!("开始游戏按钮被点击, 切换到 Match3 状态。");
                    ns_game.set(GameState::Match3);
                }
                MainMenuButtonAction::Settings => {
                    info!("设置按钮被点击, 切换到设置界面");
                    mw_overlay.write(OverlayMessage {
                        action: OverlayAction::Push,
                        overlay: OverlayState::Settings,
                    });
                }
                MainMenuButtonAction::Quit => {
                    app_exit_mw.write(AppExit::Success);
                }
            }
        }
    }
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<OnMainMenuScreen>>,
) {
    // 查询所有带有 OnMainMenuScreen 标记的实体并销毁它们
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
