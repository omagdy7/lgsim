use std::collections::HashMap;

use crate::circuit::Chip;
use crate::gate::{Gate, GateType};
use crate::pin::next_uuid;
use eframe::egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

pub fn draw_connection_dot(
    ui: &mut Ui,
    pos: Pos2,
    pin_id: usize,
    val: u8,
    is_input_pin: bool,
    dragging_wire_from: &mut Option<(usize, Pos2)>,
) -> Option<usize> {
    let radius = 6.0;
    let color = if val == 1 {
        Color32::GREEN
    } else {
        Color32::from_rgb(50, 0, 0)
    };
    let hit_rect = Rect::from_center_size(pos, Vec2::splat(20.0));
    let interact = ui.interact(hit_rect, ui.id().with("pin").with(pin_id), Sense::drag());

    if interact.drag_started() && !is_input_pin {
        *dragging_wire_from = Some((pin_id, pos));
    }

    if let Some(_) = dragging_wire_from {
        if is_input_pin
            && hit_rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO)))
        {
            ui.painter()
                .circle_stroke(pos, radius + 4.0, Stroke::new(2.0, Color32::YELLOW));
            if ui.input(|i| i.pointer.any_released()) {
                return Some(pin_id);
            }
        }
    }

    ui.painter().circle_filled(pos, radius, color);
    if !is_input_pin {
        ui.painter()
            .circle_stroke(pos, radius, Stroke::new(1.0, Color32::WHITE));
    }
    None
}

#[derive(Clone)]
pub struct VisualNode {
    pub gate_id: usize,
    pub pos: Pos2,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub label: String,
}

pub struct LogicApp {
    pub chip: Chip,
    pub nodes: Vec<VisualNode>,
    pub dragging_wire_from: Option<(usize, Pos2)>,
    pub input_count: usize,
    pub output_count: usize,
    pub global_input_ids: Vec<usize>,
    pub global_output_ids: Vec<usize>,
    pub auto_sim: bool,
    pub chip_templates: HashMap<String, Chip>,
    pub show_abstract_window: bool,
    pub abstract_name: String,
}

impl LogicApp {
    pub fn new() -> Self {
        Self {
            chip: Chip::new(0),
            nodes: Vec::new(),
            dragging_wire_from: None,
            input_count: 2,
            output_count: 1,
            global_input_ids: Vec::new(),
            global_output_ids: Vec::new(),
            auto_sim: false,
            chip_templates: HashMap::new(),
            show_abstract_window: false,
            abstract_name: String::new(),
        }
    }

    pub fn add_gate(&mut self, gtype: GateType, pos: Pos2) {
        let gate = Gate::new(gtype, vec![]);
        self.register_visual_node(gate, pos, "UNK".to_string());
    }

    pub fn add_custom_chip(&mut self, name: &str, pos: Pos2) {
        if let Some(template) = self.chip_templates.get(name) {
            let new_chip = template.deep_copy();
            let gate = Gate::Chip(new_chip);
            self.register_visual_node(gate, pos, name.to_string());
        }
    }

    pub fn register_visual_node(&mut self, gate: Gate, pos: Pos2, custom_label: String) {
        let id = gate.id();
        let inputs = gate.input().to_vec();
        let outputs = gate.output().to_vec();
        let label = match gate {
            Gate::And(_) => "AND".to_string(),
            Gate::Not(_) => "NOT".to_string(),
            Gate::Chip(_) => custom_label,
            _ => "UNK".to_string(),
        };

        self.chip.add_gate(gate);
        self.nodes.push(VisualNode {
            gate_id: id,
            pos,
            inputs,
            outputs,
            label,
        });
    }

    pub fn sync_io(&mut self) {
        while self.global_input_ids.len() < self.input_count {
            let gate = Gate::new(GateType::Source, vec![]);
            self.global_input_ids.push(self.chip.add_gate(gate));
        }
        while self.global_input_ids.len() > self.input_count {
            self.global_input_ids.pop();
        }
        while self.global_output_ids.len() < self.output_count {
            let gate = Gate::new(GateType::Output, vec![]);
            self.global_output_ids.push(self.chip.add_gate(gate));
        }
        while self.global_output_ids.len() > self.output_count {
            self.global_output_ids.pop();
        }
    }

    pub fn create_abstract_chip(&mut self) {
        // Use 0 temporarily, id assignment happens in deep_copy for components
        let mut template = Chip::new(0);
        let mut id_map: HashMap<usize, usize> = HashMap::new();

        let mut shell_input_map: HashMap<usize, usize> = HashMap::new();
        for &gid in &self.global_input_ids {
            let src_gate = self.chip.gates.get(&gid).unwrap();
            let src_pin = src_gate.output()[0];
            let new_shell_id = template.add_shell_pin(crate::pin::PinType::ChipInput);
            shell_input_map.insert(src_pin, new_shell_id);
        }

        let mut shell_output_map: HashMap<usize, usize> = HashMap::new();
        for &gid in &self.global_output_ids {
            let dest_gate = self.chip.gates.get(&gid).unwrap();
            let dest_pin = dest_gate.input()[0];
            let new_shell_id = template.add_shell_pin(crate::pin::PinType::ChipOutput);
            shell_output_map.insert(dest_pin, new_shell_id);
        }

        for node in &self.nodes {
            if let Some(gate) = self.chip.gates.get(&node.gate_id) {
                let new_gate = gate.clone_with_new_ids(&mut id_map);
                template.add_gate(new_gate);
            }
        }

        for (src, dests) in &self.chip.connections {
            if let Some(&shell_in) = shell_input_map.get(src) {
                for dst in dests {
                    if let Some(&mapped_dst) = id_map.get(dst) {
                        template.connect_pins(shell_in, mapped_dst);
                    }
                }
                continue;
            }

            if let Some(&mapped_src) = id_map.get(src) {
                for dst in dests {
                    if let Some(&shell_out) = shell_output_map.get(dst) {
                        template.connect_pins(mapped_src, shell_out);
                    } else if let Some(&mapped_dst) = id_map.get(dst) {
                        template.connect_pins(mapped_src, mapped_dst);
                    }
                }
            }
        }

        self.chip_templates
            .insert(self.abstract_name.clone(), template);
        self.abstract_name.clear();

        // Optional: Reset board after abstracting
        self.nodes.clear();
        self.chip = Chip::new(next_uuid());
        self.global_input_ids.clear();
        self.global_output_ids.clear();
    }
}

impl eframe::App for LogicApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.sync_io();
        if self.auto_sim {
            self.chip.simulate();
        }

        eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Controls:");
                if ui.button("RUN").clicked() {
                    self.chip.simulate();
                }
                ui.checkbox(&mut self.auto_sim, "Auto-Sim");
                ui.separator();
                if ui.button("ABSTRACT CIRCUIT").clicked() {
                    self.show_abstract_window = true;
                }
                ui.separator();
                ui.label("In:");
                ui.add(eframe::egui::Slider::new(&mut self.input_count, 1..=16));
                ui.label("Out:");
                ui.add(eframe::egui::Slider::new(&mut self.output_count, 1..=16));
            });
        });

        eframe::egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Tools");
            if ui.button("Add AND").clicked() {
                self.add_gate(GateType::And, eframe::egui::Pos2::new(400.0, 200.0));
            }
            if ui.button("Add NOT").clicked() {
                self.add_gate(GateType::Not, eframe::egui::Pos2::new(400.0, 300.0));
            }

            ui.separator();
            ui.heading("Custom Chips");

            let names: Vec<String> = self.chip_templates.keys().cloned().collect();
            for name in names {
                if ui.button(format!("Add {}", name)).clicked() {
                    self.add_custom_chip(&name, eframe::egui::Pos2::new(400.0, 400.0));
                }
            }

            ui.separator();
            ui.label("Drag gates to move.\nDrag Output -> Input.");
        });

        if self.show_abstract_window {
            eframe::egui::Window::new("Name Your Chip")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.abstract_name);
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            if !self.abstract_name.is_empty() {
                                self.create_abstract_chip();
                                self.show_abstract_window = false;
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_abstract_window = false;
                        }
                    });
                });
        }

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            let mut connection_made: Option<(usize, usize)> = None;

            let available_rect = ui.available_rect_before_wrap();
            let center_y = available_rect.center().y;
            let left_x = available_rect.min.x + 20.0;
            let right_x = available_rect.max.x - 40.0;

            let input_start_y = center_y - (self.global_input_ids.len() as f32 * 40.0 / 2.0);
            let output_start_y = center_y - (self.global_output_ids.len() as f32 * 40.0 / 2.0);

            // 1. DRAW GLOBAL INPUTS
            for (i, &gid) in self.global_input_ids.iter().enumerate() {
                let pos = eframe::egui::Pos2::new(left_x, input_start_y + i as f32 * 40.0);
                let gate = self.chip.gates.get_mut(&gid).unwrap();
                let out_pin = gate.output()[0];
                let val = gate.pins().get(&out_pin).unwrap().val.unwrap_or(0);

                let btn_rect = eframe::egui::Rect::from_center_size(pos, eframe::egui::Vec2::new(30.0, 20.0));
                if ui
                    .interact(btn_rect, ui.id().with("input").with(gid), eframe::egui::Sense::click())
                    .clicked()
                {
                    if let crate::gate::Gate::Source(g) = self.chip.gates.get_mut(&gid).unwrap() {
                        let new = if val == 1 { 0 } else { 1 };
                        g.set_pin(&out_pin, Some(new));
                    }
                }
                ui.painter().rect_filled(
                    btn_rect,
                    4.0,
                    if val == 1 {
                        eframe::egui::Color32::DARK_GREEN
                    } else {
                        eframe::egui::Color32::DARK_RED
                    },
                );
                ui.painter()
                    .rect_stroke(btn_rect, 4.0, eframe::egui::Stroke::new(1.0, eframe::egui::Color32::WHITE));
                draw_connection_dot(
                    ui,
                    pos + eframe::egui::Vec2::new(25.0, 0.0),
                    out_pin,
                    val,
                    false,
                    &mut self.dragging_wire_from,
                );
            }

            // 2. DRAW GLOBAL OUTPUTS
            for (i, &gid) in self.global_output_ids.iter().enumerate() {
                let pos = eframe::egui::Pos2::new(right_x, output_start_y + i as f32 * 40.0);
                let gate = self.chip.gates.get(&gid).unwrap();
                let in_pin = gate.input()[0];
                let val = gate.pins().get(&in_pin).unwrap().val.unwrap_or(0);

                if let Some(t) = draw_connection_dot(
                    ui,
                    pos - eframe::egui::Vec2::new(25.0, 0.0),
                    in_pin,
                    val,
                    true,
                    &mut self.dragging_wire_from,
                ) {
                    if let Some((src, _)) = self.dragging_wire_from {
                        connection_made = Some((src, t));
                    }
                }
                let color = if val == 1 {
                    eframe::egui::Color32::YELLOW
                } else {
                    eframe::egui::Color32::from_gray(30)
                };
                ui.painter().circle_filled(pos, 12.0, color);
            }

            // 3. DRAW NODES
            for node in &mut self.nodes {
                let rect = eframe::egui::Rect::from_center_size(node.pos, eframe::egui::Vec2::new(80.0, 50.0));
                let interact =
                    ui.interact(rect, ui.id().with("gate").with(node.gate_id), eframe::egui::Sense::drag());
                if interact.dragged() {
                    node.pos += interact.drag_delta();
                }

                ui.painter().rect_filled(rect, 5.0, eframe::egui::Color32::from_gray(60));
                ui.painter()
                    .rect_stroke(rect, 5.0, eframe::egui::Stroke::new(1.0, eframe::egui::Color32::WHITE));
                ui.painter().text(
                    node.pos,
                    eframe::egui::Align2::CENTER_CENTER,
                    &node.label,
                    eframe::egui::FontId::proportional(16.0),
                    eframe::egui::Color32::WHITE,
                );

                // Inputs
                let inputs = node.inputs.clone();
                for (j, &pid) in inputs.iter().enumerate() {
                    let y_off = (j as f32 - (inputs.len() as f32 - 1.0) / 2.0) * 15.0;
                    let val = self
                        .chip
                        .gates
                        .get(&node.gate_id)
                        .unwrap()
                        .pins()
                        .get(&pid)
                        .unwrap()
                        .val
                        .unwrap_or(0);
                    if let Some(t) = draw_connection_dot(
                        ui,
                        node.pos + eframe::egui::Vec2::new(-40.0, y_off),
                        pid,
                        val,
                        true,
                        &mut self.dragging_wire_from,
                    ) {
                        if let Some((src, _)) = self.dragging_wire_from {
                            connection_made = Some((src, t));
                        }
                    }
                }
                // Outputs
                let outputs = node.outputs.clone();
                for (j, &pid) in outputs.iter().enumerate() {
                    let y_off = (j as f32 - (outputs.len() as f32 - 1.0) / 2.0) * 15.0;
                    let val = self
                        .chip
                        .gates
                        .get(&node.gate_id)
                        .unwrap()
                        .pins()
                        .get(&pid)
                        .unwrap()
                        .val
                        .unwrap_or(0);
                    draw_connection_dot(
                        ui,
                        node.pos + eframe::egui::Vec2::new(40.0, y_off),
                        pid,
                        val,
                        false,
                        &mut self.dragging_wire_from,
                    );
                }
            }

            // 4. DRAW WIRES
            for (src, dests) in &self.chip.connections {
                let mut src_pos = eframe::egui::Pos2::ZERO;
                let mut val = 0;

                if let Some(idx) = self
                    .global_input_ids
                    .iter()
                    .position(|gid| self.chip.gates.get(gid).unwrap().output().contains(src))
                {
                    src_pos = eframe::egui::Pos2::new(left_x + 25.0, input_start_y + idx as f32 * 40.0);
                    val = self
                        .chip
                        .gates
                        .get(&self.global_input_ids[idx])
                        .unwrap()
                        .pins()
                        .get(src)
                        .unwrap()
                        .val
                        .unwrap_or(0);
                } else if let Some(node) = self.nodes.iter().find(|n| n.outputs.contains(src)) {
                    src_pos = node.pos + eframe::egui::Vec2::new(40.0, 0.0);
                    if let Some(idx) = node.outputs.iter().position(|x| x == src) {
                        src_pos.y += (idx as f32 - (node.outputs.len() as f32 - 1.0) / 2.0) * 15.0;
                    }
                    val = self
                        .chip
                        .gates
                        .get(&node.gate_id)
                        .unwrap()
                        .pins()
                        .get(src)
                        .unwrap()
                        .val
                        .unwrap_or(0);
                }

                if src_pos != eframe::egui::Pos2::ZERO {
                    let color = if val == 1 {
                        eframe::egui::Color32::GREEN
                    } else {
                        eframe::egui::Color32::from_rgb(100, 0, 0)
                    };
                    for dest in dests {
                        let mut dest_pos = eframe::egui::Pos2::ZERO;
                        if let Some(idx) = self.global_output_ids.iter().position(|gid| {
                            self.chip.gates.get(gid).unwrap().input().contains(dest)
                        }) {
                            dest_pos =
                                eframe::egui::Pos2::new(right_x - 25.0, output_start_y + idx as f32 * 40.0);
                        } else if let Some(node) =
                            self.nodes.iter().find(|n| n.inputs.contains(dest))
                        {
                            let idx = node.inputs.iter().position(|x| x == dest).unwrap_or(0);
                            let y_off =
                                (idx as f32 - (node.inputs.len() as f32 - 1.0) / 2.0) * 15.0;
                            dest_pos = node.pos + eframe::egui::Vec2::new(-40.0, y_off);
                        }

                        if dest_pos != eframe::egui::Pos2::ZERO {
                            let scale = (dest_pos.x - src_pos.x).max(20.0) / 2.0;
                            let points = [
                                src_pos,
                                src_pos + eframe::egui::Vec2::new(scale, 0.0),
                                dest_pos - eframe::egui::Vec2::new(scale, 0.0),
                                dest_pos,
                            ];
                            ui.painter().add(eframe::egui::Shape::CubicBezier(
                                eframe::epaint::CubicBezierShape::from_points_stroke(
                                    points,
                                    false,
                                    eframe::egui::Color32::TRANSPARENT,
                                    eframe::egui::Stroke::new(2.0, color),
                                ),
                            ));
                        }
                    }
                }
            }

            if let Some((_, start)) = self.dragging_wire_from {
                if let Some(curr) = ctx.pointer_hover_pos() {
                    ui.painter()
                        .line_segment([start, curr], eframe::egui::Stroke::new(2.0, eframe::egui::Color32::YELLOW));
                }
                if ui.input(|i| i.pointer.any_released()) {
                    self.dragging_wire_from = None;
                }
            }
            if let Some((src, dest)) = connection_made {
                self.chip.connect_pins(src, dest);
                self.dragging_wire_from = None;
                self.chip.simulate();
            }
        });
    }
}