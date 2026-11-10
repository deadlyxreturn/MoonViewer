//! Decorative login elements. Hit targets and actions stay in the login form.
use crate::theme::Palette;
use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, pos2, vec2};

use super::super::widgets::gradient_rect_shape;

/// A second, inset reflection gives the rounded face a visible glass
/// surface without covering the text or changing the widget's hit target.
pub(super) fn polished_pill(ctx: &egui::Context, rect: Rect, top: Color32, bottom: Color32, light: Color32, edge: Color32) -> Shape {
    let radius = rect.height() * 0.5;
    let mut face = vec![gradient_rect_shape(ctx, rect, radius, top, bottom)];
    let shine = Rect::from_min_max(
        rect.min + vec2(radius * 0.52, 2.0),
        pos2(rect.right() - radius * 0.52, rect.top() + rect.height() * 0.43),
    );
    if shine.is_positive() {
        face.push(gradient_rect_shape(
            ctx,
            shine,
            shine.height() * 0.5,
            light.gamma_multiply(0.68),
            light.gamma_multiply(0.01),
        ));
    }
    face.push(Shape::rect_stroke(rect, radius, Stroke::new(1.4, edge), StrokeKind::Inside));
    face.push(Shape::rect_stroke(
        rect.shrink(2.3),
        radius - 2.3,
        Stroke::new(0.8, light.gamma_multiply(0.65)),
        StrokeKind::Inside,
    ));
    Shape::Vec(face)
}

pub(super) fn glass_tint(ctx: &egui::Context, rect: Rect, p: &Palette) -> Shape {
    let rect = rect.shrink(4.0);
    let lavender = p.indigo_light.lerp_to_gamma(p.ink, 0.3);
    Shape::Vec(vec![
        gradient_rect_shape(ctx, rect, 52.0, lavender.gamma_multiply(0.16), lavender.gamma_multiply(0.04)),
        gradient_rect_shape(ctx, rect, 52.0, Color32::TRANSPARENT, p.violet_pale.gamma_multiply(0.17)),
    ])
}

fn star(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let points = [
        (0.0, -1.0),
        (0.16, -0.18),
        (0.8, 0.0),
        (0.16, 0.18),
        (0.0, 1.0),
        (-0.16, 0.18),
        (-0.8, 0.0),
        (-0.16, -0.18),
    ];
    painter.add(Shape::closed_line(
        points.into_iter().map(|(x, y)| center + vec2(x, y) * size).collect(),
        Stroke::new(1.0, color),
    ));
    painter.line_segment(
        [center - vec2(0.0, size * 0.65), center + vec2(0.0, size * 0.65)],
        Stroke::new(1.2, color),
    );
    painter.line_segment(
        [center - vec2(size * 0.5, 0.0), center + vec2(size * 0.5, 0.0)],
        Stroke::new(1.2, color),
    );
}

fn crystal(painter: &Painter, center: Pos2, size: f32, palette: &Palette) {
    let top = center - vec2(0.0, size);
    let bottom = center + vec2(0.0, size);
    let left = center - vec2(size * 0.53, 0.0);
    let right = center + vec2(size * 0.53, 0.0);
    painter.add(Shape::convex_polygon(
        vec![top, right, bottom, left],
        palette.indigo,
        Stroke::new(1.5, palette.amber),
    ));
    painter.add(Shape::convex_polygon(
        vec![top, right, center, left],
        palette.violet_light,
        Stroke::NONE,
    ));
    painter.line_segment([top, bottom], Stroke::new(0.8, palette.violet_pale));
    star(painter, center, size * 0.6, palette.ink);
}

pub(super) fn frame_art(ctx: &egui::Context, rect: Rect) -> Option<Shape> {
    let texture = super::super::prototype_art::texture(ctx, super::super::prototype_art::Art::Frame)?;
    // The usable panel in the source artwork spans x=122..903 and
    // y=124..1397. Scale that opening to the panel; ornaments overhang it.
    let scale = vec2(rect.width() / 781.0, rect.height() / 1273.0);
    let bounds = Rect::from_min_size(rect.min - vec2(122.0, 124.0) * scale, vec2(1024.0, 1536.0) * scale);
    Some(Shape::image(texture.id(), bounds, full_uv(), Color32::WHITE))
}

fn full_uv() -> Rect {
    Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0))
}

pub(super) fn divider(painter: &Painter, rect: Rect, p: &Palette) {
    let center = rect.center();
    let line = Stroke::new(1.0, p.amber.gamma_multiply(0.74));
    painter.line_segment([pos2(rect.left(), center.y), center - vec2(20.0, 0.0)], line);
    painter.line_segment([center + vec2(20.0, 0.0), pos2(rect.right(), center.y)], line);
    crystal(painter, center, 9.5, p);
    for sign in [-1.0, 1.0] {
        star(painter, center + vec2(sign * 32.0, 0.0), 6.0, p.amber);
    }
}

pub(super) fn connect_button(painter: &Painter, rect: Rect, p: &Palette, active: bool, hot: bool) {
    if !p.high_contrast
        && let Some(texture) = super::super::prototype_art::texture(painter.ctx(), super::super::prototype_art::Art::Connect)
    {
        let size = vec2(rect.width() + 18.0, 118.0);
        let bounds = Rect::from_center_size(rect.center() - vec2(0.0, size.y * 0.015), size);
        let opacity = if active { 255 } else { 242 };
        painter.image(texture.id(), bounds, full_uv(), Color32::from_white_alpha(opacity));
        if hot {
            let rim = rect.shrink2(vec2(rect.width() * 0.125, 4.0));
            painter.rect_stroke(rim, rim.height() * 0.5, Stroke::new(1.5, p.violet_pale), StrokeKind::Outside);
        }
        return;
    }
    let rect = rect.shrink2(vec2(18.0, 2.0));
    let fill = if active { p.violet } else { p.indigo };
    if p.high_contrast {
        painter.rect_filled(rect, rect.height() * 0.5, p.violet);
    } else {
        painter.add(polished_pill(painter.ctx(), rect, p.violet_light, fill, p.ink, p.amber));
    }
}
