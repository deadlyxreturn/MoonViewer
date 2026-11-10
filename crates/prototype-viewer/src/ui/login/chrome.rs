//! Shared login information, community links, version footer and the
//! preferences button.
use super::{LoginAction, LoginForm};
use crate::theme::Palette;
use crate::ui::{
    icons::Icons,
    news::LoginInfo,
    widgets::{self, ActionKind},
};
use egui::{Context, Id, Rect, RichText, Vec2, pos2, vec2};

fn menu_action(ui: &mut egui::Ui, label: &str, action: &mut LoginAction, next: LoginAction) {
    if ui.button(label).clicked() {
        *action = next;
        ui.close();
    }
}

fn menu_bar(ctx: &Context, p: &Palette, form: &mut LoginForm) -> LoginAction {
    let mut action = LoginAction::None;
    let screen = ctx.content_rect();
    egui::Area::new(Id::unique("moon_login_menu"))
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(if p.high_contrast { p.bar } else { p.bar.gamma_multiply(0.58) })
                .inner_margin(egui::Margin::symmetric(15, 5))
                .show(ui, |ui| {
                    ui.set_width((screen.width() - 30.0).max(0.0));
                    ui.set_min_height(29.0);
                    if !p.high_contrast {
                        let strip = Rect::from_min_size(screen.min, vec2(screen.width(), 39.0));
                        widgets::gradient_rect(
                            ui.painter(),
                            strip,
                            0.0,
                            p.indigo_light.gamma_multiply(0.23),
                            p.navy.gamma_multiply(0.12),
                        );
                        // glass sheen: a pearl highlight over the bar's upper half
                        let sheen = Rect::from_min_size(screen.min, vec2(screen.width(), 18.0));
                        widgets::gradient_rect(
                            ui.painter(),
                            sheen,
                            0.0,
                            p.ink.gamma_multiply(0.16),
                            egui::Color32::TRANSPARENT,
                        );
                    }
                    egui::MenuBar::new().ui(ui, |ui| {
                        let (mark, _) = ui.allocate_exact_size(vec2(29.0, 28.0), egui::Sense::hover());
                        if !p.high_contrast {
                            super::super::prototype_bg::radial_glow(ui.painter(), mark.center(), 19.0, p.amber, 0.27);
                        }
                        widgets::paint_glyph(ui.painter(), "moon-filled", mark.center(), 27.0, p.amber.lerp_to_gamma(p.ink, 0.36));
                        if screen.width() >= 650.0 {
                            let font = egui::FontId::new(23.0, super::super::fonts::display_serif(ui.ctx()));
                            ui.label(RichText::new("MOON VIEWER").font(font).extra_letter_spacing(1.0).color(p.ink));
                            ui.add_space(8.0);
                        }
                        ui.separator();
                        ui.menu_button(prototype_i18n::tr!("moon-menu-viewer"), |ui| {
                            menu_action(ui, prototype_i18n::tr!("common-preferences"), &mut action, LoginAction::Preferences);
                            menu_action(
                                ui,
                                prototype_i18n::tr!("login-accessibility"),
                                &mut action,
                                LoginAction::Accessibility,
                            );
                            menu_action(ui, prototype_i18n::tr!("common-quit"), &mut action, LoginAction::Quit);
                        });
                        ui.menu_button(prototype_i18n::tr!("moon-menu-world"), |ui| {
                            if ui.button(prototype_i18n::tr!("grids-manage")).clicked() {
                                super::super::grids::request_open(ctx);
                                ui.close();
                            }
                        });
                        ui.menu_button(prototype_i18n::tr!("moon-menu-help"), |ui| {
                            if ui.button(prototype_i18n::tr!("ui-login-news")).clicked() {
                                form.show_info = true;
                                ui.close();
                            }
                            menu_action(
                                ui,
                                prototype_i18n::tr!("moon-menu-check-updates"),
                                &mut action,
                                LoginAction::CheckUpdates,
                            );
                            menu_action(ui, prototype_i18n::tr!("moon-menu-report-bug"), &mut action, LoginAction::ReportBug);
                        });
                        ui.menu_button(prototype_i18n::tr!("moon-menu-debug"), |ui| {
                            menu_action(ui, prototype_i18n::tr!("moon-login-advanced"), &mut action, LoginAction::Advanced);
                            menu_action(
                                ui,
                                prototype_i18n::tr!("moon-menu-copy-version"),
                                &mut action,
                                LoginAction::CopyVersion,
                            );
                        });
                    });
                });
            let edge = ui.min_rect().bottom();
            if !p.high_contrast {
                // a soft gold glow under the bar's edge
                ui.painter().hline(screen.x_range(), edge + 1.0, egui::Stroke::new(4.0, p.amber.gamma_multiply(0.16)));
            }
            ui.painter().hline(screen.x_range(), edge, egui::Stroke::new(1.2, p.amber.gamma_multiply(0.85)));
            // the version stays in Debug › Copy version, out of the bar
        });
    action
}

fn caption_button(ui: &mut egui::Ui, p: &Palette, icon: &str, label: &str, close: bool) -> bool {
    let (rect, response) = ui.allocate_exact_size(vec2(34.0, 28.0), egui::Sense::click());
    let hot = response.hovered() || response.has_focus();
    let ink = if p.high_contrast {
        widgets::glass_pill(ui.painter(), p, rect, close && hot, hot)
    } else {
        // glossy rounded square with a gold rim, like the concept art
        let painter = ui.painter();
        let radius = 8.0;
        let face = if hot { 0.62 } else { 0.42 };
        widgets::gradient_rect(painter, rect, radius, p.indigo_light.gamma_multiply(face), p.navy.gamma_multiply(0.38));
        if close && hot {
            painter.rect_filled(rect, radius, p.rose.gamma_multiply(0.55));
        }
        let top = Rect::from_min_max(rect.min + vec2(3.0, 1.5), pos2(rect.right() - 3.0, rect.center().y));
        widgets::gradient_rect(painter, top, 5.0, p.ink.gamma_multiply(0.30), egui::Color32::TRANSPARENT);
        let rim = p.amber.lerp_to_gamma(p.ink, 0.4).gamma_multiply(if hot { 1.0 } else { 0.75 });
        painter.rect_stroke(rect, radius, egui::Stroke::new(1.2, rim), egui::StrokeKind::Inside);
        p.ink
    };
    widgets::paint_glyph(ui.painter(), icon, rect.center(), 16.0, ink);
    response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, ui.is_enabled(), label));
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(label)
        .clicked()
}

fn caption_controls(ctx: &Context, p: &Palette, action: &mut LoginAction) {
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    let (maximize_icon, maximize_label) = if maximized {
        ("copy", prototype_i18n::tr!("ui-options-restore"))
    } else {
        ("square", prototype_i18n::tr!("common-maximize"))
    };
    egui::Area::new(Id::unique("login_window_controls"))
        .anchor(egui::Align2::RIGHT_TOP, vec2(-10.0, 5.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 7.0;
                if caption_button(ui, p, "minus", prototype_i18n::tr!("region-restart-minimize"), false) {
                    *action = LoginAction::Minimize;
                }
                if caption_button(ui, p, maximize_icon, maximize_label, false) {
                    *action = LoginAction::ToggleMaximize;
                }
                if caption_button(ui, p, "x", prototype_i18n::tr!("common-close"), true) {
                    *action = LoginAction::Quit;
                }
            });
        });
}

pub fn sidebar(ctx: &Context, p: &Palette, icons: &Icons, info: &LoginInfo, card: Rect, width: f32, lowest: f32) -> bool {
    let left = ctx.content_rect().right() - 32.0 - width;
    egui::Area::new(Id::unique("login_news"))
        .fixed_pos(egui::pos2(left, card.top()))
        .show(ctx, |ui| {
            ui.set_width(width);
            super::news_panel(ui, p, icons, info, card.height().min(lowest - card.top()).max(120.0));
        });
    egui::Area::new(Id::unique("login_version"))
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(left, card.top() - 14.0))
        .show(ctx, |ui| {
            ui.set_width(width);
            super::version_panel(ui, p, icons, info)
        })
        .inner
}

pub fn footer(ctx: &Context, p: &Palette, icons: &Icons, info: &LoginInfo, sidebar_visible: bool, form: &mut LoginForm) -> LoginAction {
    let mut action = menu_bar(ctx, p, form);
    if form.safe_graphics.is_some() || form.driver_notice.is_some() {
        super::driver_notice::column(ctx, |ui, width| {
            if let Some(state) = form.safe_graphics
                && let Some(choice) = super::safe_graphics_notice::card(ui, p, icons, state, width)
            {
                action = LoginAction::SafeGraphics(choice);
            }
            if let Some(notice) = &form.driver_notice
                && super::driver_notice::card(ui, p, icons, notice, width)
            {
                action = LoginAction::DismissDriverNotice;
            }
        });
    }
    let _ = sidebar_visible;
    // Quick Preferences and Accessibility beside the caption buttons.
    egui::Area::new(Id::unique("login_settings"))
        .anchor(egui::Align2::RIGHT_TOP, vec2(-138.0, 4.0))
        .show(ctx, |ui| {
            if widgets::action_button(ui, p, "settings-filled", "", ActionKind::Flat, Vec2::splat(30.0))
                .on_hover_text(prototype_i18n::tr!("common-preferences"))
                .clicked()
            {
                action = LoginAction::Preferences;
            }
        });
    // Left of it, the way to a larger, more readable interface before the
    // first login: Preferences › Accessibility.
    egui::Area::new(Id::unique("login_accessibility"))
        .anchor(egui::Align2::RIGHT_TOP, vec2(-176.0, 4.0))
        .show(ctx, |ui| {
            if widgets::action_button(ui, p, "accessible-filled", "", ActionKind::Flat, Vec2::splat(30.0))
                .on_hover_text(prototype_i18n::tr!("login-accessibility"))
                .clicked()
            {
                action = LoginAction::Accessibility;
            }
        });
    caption_controls(ctx, p, &mut action);
    if form.show_info {
        let id = Id::unique("accounts_news_dialog");
        prototype_render::blurred_backdrop(ctx, egui::Modal::default_area(id).layer());
        let response = widgets::Modal::new(id).transparent_backdrop().show(ctx, |ui| {
            ui.set_width(380.0);
            ui.spacing_mut().item_spacing.y = 8.0;
            // Prototype: the bear and the name, like Preferences' title row
            ui.horizontal(|ui| {
                if let Some(bear) = super::super::prototype_art::texture(ctx, super::super::prototype_art::Art::Bear) {
                    let size = super::super::prototype_art::size_at_width(&bear, 30.0);
                    ui.add(egui::Image::new(&bear).fit_to_exact_size(size));
                }
                ui.label(RichText::new(prototype_i18n::tr!("ui-login-news")).size(20.0).color(p.ink));
            });
            if super::version_panel(ui, p, icons, info) {
                action = LoginAction::CheckUpdates;
            }
            ui.add_space(8.0);
            super::news_panel(ui, p, icons, info, 235.0);
            ui.add_space(4.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if widgets::action_button(
                    ui,
                    p,
                    "",
                    prototype_i18n::tr!("common-close"),
                    ActionKind::Primary,
                    Vec2::new(130.0, 40.0),
                )
                .clicked()
                {
                    form.show_info = false;
                }
            });
        });
        if response.should_close() {
            form.show_info = false;
        }
    }
    action
}
