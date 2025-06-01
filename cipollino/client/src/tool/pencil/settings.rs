
use project::StrokeBrush;

use crate::{AppSystems, EditorState, RendererState, BUILTIN_BRUSHES};
use super::{PencilStrokeWidthPref, PencilTool, PencilUsePressure, StabilizationRadius};

impl PencilTool {

    pub(super) fn settings(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, systems: &mut AppSystems, renderer: &mut Option<RendererState>) {
        pierro::scroll_area(ui, |ui| {
            pierro::margin(ui, pierro::Margin::same(3.0), |ui| {
                pierro::key_value_layout(ui, |builder| {
                    self.settings_contents(builder, editor, systems, renderer);
                });
            });
        });
    }

    pub(super) fn settings_contents(&mut self, builder: &mut pierro::KeyValueBuilder, editor: &mut EditorState, systems: &mut AppSystems, renderer: &mut Option<RendererState>) {
        builder.labeled("Stroke Width:", |ui| {
            let mut stroke_width = systems.prefs.get::<PencilStrokeWidthPref>();
            let prev_stroke_width = stroke_width;
            pierro::DragValue::new(&mut stroke_width) 
                .with_min(0.75)
                .with_max(100.0)
                .render(ui);
            if stroke_width != prev_stroke_width {
                systems.prefs.set::<PencilStrokeWidthPref>(&stroke_width);
            }
        });
        builder.labeled("Stabilization Radius:", |ui| {
            let mut radius = systems.prefs.get::<StabilizationRadius>();
            let prev_radius = radius;
            pierro::DragValue::new(&mut radius) 
                .with_max(200)
                .render(ui);
            if radius != prev_radius {
                systems.prefs.set::<StabilizationRadius>(&radius);
            }
        });
        builder.labeled("Use Pen Pressure:", |ui| {
            let mut use_pen_pressure = systems.prefs.get::<PencilUsePressure>();
            let prev_use_pen_pressure = use_pen_pressure;
            pierro::checkbox(ui, &mut use_pen_pressure);
            if use_pen_pressure != prev_use_pen_pressure {
                systems.prefs.set::<PencilUsePressure>(&use_pen_pressure);
            }
        });
        builder.labeled("Draw Fill:", |ui| {
            pierro::checkbox(ui, &mut self.draw_fill);
        });
        builder.labeled("Brush:", |ui| {

            let brush_name = match editor.brush {
                StrokeBrush::Builtin(idx) => BUILTIN_BRUSHES[idx].name,
            };
            
            pierro::dropdown(ui, brush_name, |ui| {
                for (brush_idx, brush) in BUILTIN_BRUSHES.iter().enumerate() {
                    let resp = pierro::menu_button_with_content(ui, |ui| {
                        pierro::label(ui, brush.name);
                        if let Some(renderer) = renderer {
                            pierro::scaled_image(ui, 0.5, renderer.builtin_brushes.previews[brush_idx].clone());
                        }
                    });
                    if resp.mouse_clicked() {
                        editor.brush = StrokeBrush::Builtin(brush_idx);
                    }
                }
            });
        });
    }

}
