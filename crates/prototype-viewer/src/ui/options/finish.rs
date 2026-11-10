use crate::theme::Palette;
use crate::ui::{prototype_bg, widgets};
use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, pos2, vec2};

/// Gold and pearl around the glass, with a crest above the draggable title bar.
pub(in crate::ui) fn paint_window_trim(painter: &Painter, p: &Palette, rect: Rect, radius: f32, minimized: bool) {
    if p.high_contrast {
        return;
    }
    if !minimized && let Some(frame) = crate::ui::prototype_art::texture(painter.ctx(), crate::ui::prototype_art::Art::PreferencesFrame) {
        // Artwork has transparent bleed. Its four straight gold rules sit at
        // x = 96, 1384 and y = 119, 939 in the 1479 × 1064 source.
        let x = rect.width() / (1384.0 - 96.0);
        let y = rect.height() / (939.0 - 119.0);
        let art = Rect::from_min_max(rect.min - vec2(96.0 * x, 119.0 * y), rect.max + vec2(95.0 * x, 125.0 * y));
        painter.image(
            frame.id(),
            art,
            Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
            widgets::PICTURE_TINT,
        );
        return;
    }
    let pale = p.amber.lerp_to_gamma(p.ink, 0.65);
    painter.rect_stroke(rect.shrink(0.5), radius, Stroke::new(1.25, p.amber), StrokeKind::Inside);
    painter.rect_stroke(
        rect.shrink(3.0),
        (radius - 2.5).max(0.0),
        Stroke::new(1.0, pale.gamma_multiply(0.42)),
        StrokeKind::Inside,
    );
    if minimized {
        return;
    }

    let center = rect.center_top();
    let crest = center - vec2(0.0, 10.0);
    prototype_bg::radial_glow(painter, crest, 35.0, p.rose, 0.28);
    for side in [-1.0, 1.0] {
        let at = |x, y| center + vec2(side * x, y);
        let control = [at(8.0, 2.5), at(22.0, -2.0), at(30.0, -19.0), at(62.0, 0.2)];
        painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
            control,
            false,
            Color32::TRANSPARENT,
            Stroke::new(2.0, p.amber),
        ));
        let highlight = control.map(|point| point - vec2(0.0, 1.25));
        painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
            highlight,
            false,
            Color32::TRANSPARENT,
            Stroke::new(1.0, pale),
        ));
        sparkle(painter, at(29.0, -10.0), 3.6, pale, p.amber);

        let x = rect.center().x + side * (rect.width() * 0.5 - 19.0);
        sparkle(painter, pos2(x, rect.top() + 5.0), 4.2, pale, p.amber);
        sparkle(painter, pos2(x, rect.bottom() - 5.0), 4.2, pale, p.amber);
    }
    // Tabler's solid crescent keeps the background visible through its cutout.
    widgets::paint_glyph(painter, "moon-filled", crest + vec2(0.0, 1.0), 36.0, p.amber);
    widgets::paint_glyph(painter, "moon-filled", crest - vec2(0.0, 0.7), 32.0, pale);
    sparkle(painter, crest + vec2(12.0, -7.0), 4.4, pale, p.amber);
}

pub(super) fn glass_wash(painter: &Painter, p: &Palette, rect: Rect, radius: f32) {
    if !p.high_contrast {
        widgets::gradient_rect(
            painter,
            rect,
            radius,
            p.navy.gamma_multiply(0.20),
            p.indigo_light.gamma_multiply(0.27),
        );
    }
}

pub(super) fn sparkle(painter: &Painter, center: Pos2, radius: f32, face: Color32, edge: Color32) {
    let mut mesh = egui::epaint::Mesh::default();
    mesh.colored_vertex(center, face);
    for i in 0..10 {
        let theta = std::f32::consts::TAU * i as f32 / 10.0 - std::f32::consts::FRAC_PI_2;
        let r = if i % 2 == 0 { radius } else { radius * 0.43 };
        mesh.colored_vertex(center + vec2(theta.cos(), theta.sin()) * r, edge);
        mesh.add_triangle(0, i + 1, (i + 1) % 10 + 1);
    }
    painter.add(Shape::mesh(mesh));
}

/// A small, purely decorative moon-phase strip below the navigation.
pub(super) fn moon_phases(painter: &Painter, p: &Palette, center: Pos2) {
    let pearl = p.ink.lerp_to_gamma(p.amber, 0.35);
    for x in [-42.0, 42.0] {
        sparkle(painter, center + vec2(x, 0.0), 3.4, pearl, p.amber);
    }
    for (x, direction, terminator) in [(-22.0, 1.0, 0.43), (0.0, 1.0, -0.66), (22.0, -1.0, 0.43)] {
        let mut moon = egui::epaint::Mesh::default();
        let at = center + vec2(x, 0.0);
        for i in 0..=16 {
            let theta = std::f32::consts::PI * i as f32 / 16.0;
            let edge = direction * 6.3 * theta.sin();
            let y = -6.3 * theta.cos();
            moon.colored_vertex(at + vec2(edge, y), pearl);
            moon.colored_vertex(at + vec2(edge * terminator, y), p.amber);
            if i > 0 {
                let n = i * 2;
                moon.add_triangle(n - 2, n - 1, n);
                moon.add_triangle(n - 1, n + 1, n);
            }
        }
        painter.add(Shape::mesh(moon));
    }
}

/// Native Slider retains dragging, keyboard focus and AccessKit. This paints
/// the rail and star over its exact interaction rectangle.
pub(super) fn volume_rail(ui: &egui::Ui, p: &Palette, response: &egui::Response, position: f32) {
    if !ui.is_rect_visible(response.rect) || p.high_contrast {
        return;
    }
    let painter = ui.painter();
    let dim = if response.enabled() { 1.0 } else { 0.42 };
    let rail = Rect::from_center_size(response.rect.center(), vec2(response.rect.width(), 5.0));
    painter.rect_filled(rail, 2.5, p.indigo_light.gamma_multiply(0.48 * dim));
    // egui's circular handle centers travel inset by height / 2.5.
    let reach = response.rect.x_range().shrink(response.rect.height() / 2.5);
    let center = pos2(egui::lerp(reach.min..=reach.max, position), rail.center().y);
    let filled = Rect::from_min_max(rail.min, pos2(center.x, rail.bottom()));
    widgets::gradient_rect(painter, filled, 2.5, p.violet_pale.gamma_multiply(dim), p.rose.gamma_multiply(dim));
    let pearl = p.ink.lerp_to_gamma(p.amber, 0.22);
    let hot = response.hovered() || response.dragged() || response.has_focus();
    prototype_bg::radial_glow(
        painter,
        center,
        if hot { 17.0 } else { 12.0 },
        p.amber,
        if hot { 0.58 } else { 0.32 } * dim,
    );
    sparkle(
        painter,
        center,
        if hot { 8.0 } else { 7.0 },
        pearl.gamma_multiply(dim),
        p.amber.gamma_multiply(dim),
    );
    if response.has_focus() {
        painter.rect_stroke(
            response.rect,
            response.rect.height() * 0.5,
            Stroke::new(1.0, p.ink.gamma_multiply(0.72)),
            StrokeKind::Inside,
        );
    }
}
