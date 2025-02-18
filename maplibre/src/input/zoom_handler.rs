use super::UpdateState;

use crate::coords::Zoom;
use crate::map_state::{MapState, ViewState};

use crate::render::camera::Camera;
use crate::MapWindow;
use cgmath::{Vector2, Vector3};
use std::time::Duration;

pub struct ZoomHandler {
    window_position: Option<Vector2<f64>>,
    zoom_delta: Option<Zoom>,
    sensitivity: f64,
}

impl UpdateState for ZoomHandler {
    fn update_state(&mut self, state: &mut ViewState, dt: Duration) {
        if let Some(zoom_delta) = self.zoom_delta {
            if let Some(window_position) = self.window_position {
                let current_zoom = state.zoom();
                let next_zoom = current_zoom + zoom_delta;

                state.update_zoom(next_zoom);
                self.zoom_delta = None;

                let view_proj = state.view_projection();
                let inverted_view_proj = view_proj.invert();

                if let Some(cursor_position) = state
                    .camera
                    .window_to_world_at_ground(&window_position, &inverted_view_proj)
                {
                    let scale = current_zoom.scale_delta(&next_zoom);

                    let delta = Vector3::new(
                        cursor_position.x * scale,
                        cursor_position.y * scale,
                        cursor_position.z,
                    ) - cursor_position;

                    state.camera.position += delta;
                }
            }
        }
    }
}

impl ZoomHandler {
    pub fn new(sensitivity: f64) -> Self {
        Self {
            window_position: None,
            zoom_delta: None,
            sensitivity,
        }
    }

    pub fn process_window_position(
        &mut self,
        window_position: &Vector2<f64>,
        _touch: bool,
    ) -> bool {
        self.window_position = Some(*window_position);
        true
    }

    pub fn update_zoom(&mut self, delta: f64) {
        self.zoom_delta = Some(self.zoom_delta.unwrap_or_default() + Zoom::new(delta));
    }

    pub fn process_scroll(&mut self, delta: &winit::event::MouseScrollDelta) {
        self.update_zoom(
            match delta {
                winit::event::MouseScrollDelta::LineDelta(_horizontal, vertical) => {
                    *vertical as f64
                }
                winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
                    y: scroll,
                    ..
                }) => *scroll / 100.0,
            } * self.sensitivity,
        );
    }

    pub fn process_key_press(
        &mut self,
        key: winit::event::VirtualKeyCode,
        state: winit::event::ElementState,
    ) -> bool {
        let amount = if state == winit::event::ElementState::Pressed {
            0.1
        } else {
            0.0
        };

        match key {
            winit::event::VirtualKeyCode::Plus | winit::event::VirtualKeyCode::I => {
                self.update_zoom(amount);
                true
            }
            winit::event::VirtualKeyCode::Minus | winit::event::VirtualKeyCode::K => {
                self.update_zoom(-amount);
                true
            }
            _ => false,
        }
    }
}
