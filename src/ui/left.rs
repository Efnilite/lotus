use crate::app::App;
use crate::ui::icon::Icon;
use crate::ui::{large_icon_button, ActiveBottomTab, ActiveLeftTab};
use egui::{Align, Frame, Layout, Margin, Panel, Ui};

const LEFT_NAME: &str = "left";

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::left(LEFT_NAME)
        .resizable(false)
        .min_size(0.)
        .exact_size(45.)
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 8,
            right: 8,
        }))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                if large_icon_button(
                    app,
                    ui,
                    Icon::Folder,
                    app.settings.locale.project.as_str(),
                    Some(app.settings.keybinds.project),
                )
                .clicked()
                {
                    if let Some(ActiveLeftTab::Project) = app.view_state.active_left_tab {
                        app.view_state.active_left_tab = None;
                    } else {
                        app.view_state.active_left_tab = Some(ActiveLeftTab::Project);
                    }
                };

                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    if large_icon_button(
                        app,
                        ui,
                        Icon::Analytics,
                        app.settings.locale.analytics.as_str(),
                        Some(app.settings.keybinds.analytics),
                    )
                    .clicked()
                    {
                        handle_click(
                            ActiveBottomTab::Analytics,
                            &mut app.view_state.active_bottom_tab,
                        );
                    }

                    ui.add_space(6.);

                    if large_icon_button(
                        app,
                        ui,
                        Icon::Terminal,
                        app.settings.locale.terminal.as_str(),
                        Some(app.settings.keybinds.terminal),
                    )
                    .clicked()
                    {
                        handle_click(
                            ActiveBottomTab::Terminal,
                            &mut app.view_state.active_bottom_tab,
                        );
                    };

                    ui.add_space(6.);

                    if large_icon_button(
                        app,
                        ui,
                        Icon::Error,
                        app.settings.locale.problems.as_str(),
                        Some(app.settings.keybinds.problems),
                    )
                    .clicked()
                    {
                        handle_click(
                            ActiveBottomTab::Problems,
                            &mut app.view_state.active_bottom_tab,
                        );
                    };
                });
            });
        });
}

fn handle_click(tab: ActiveBottomTab, active: &mut Option<ActiveBottomTab>) {
    if let Some(active_tab) = active
        && *active_tab == tab
    {
        *active = None;
    } else {
        *active = Some(tab);
    }
}
