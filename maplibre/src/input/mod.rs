//! Handles the user input which is dispatched by the main event loop.

use std::time::Duration;

use crate::coords::Zoom;
use cgmath::Vector2;
use winit::event::{DeviceEvent, KeyboardInput, TouchPhase, WindowEvent};

use crate::input::pan_handler::PanHandler;
use crate::input::pinch_handler::PinchHandler;
use crate::input::query_handler::QueryHandler;
use crate::input::shift_handler::ShiftHandler;
use crate::input::tilt_handler::TiltHandler;
use crate::input::zoom_handler::ZoomHandler;
use crate::map_state::{MapState, ViewState};
use crate::render::camera::Camera;
use crate::MapWindow;

mod pan_handler;
mod pinch_handler;
mod query_handler;
mod shift_handler;
mod tilt_handler;
mod zoom_handler;

pub struct InputController {
    pinch_handler: PinchHandler,
    pan_handler: PanHandler,
    zoom_handler: ZoomHandler,
    tilt_handler: TiltHandler,
    shift_handler: ShiftHandler,
    query_handler: QueryHandler,
}

impl InputController {
    /// Creates a new input controller.
    ///
    /// # Arguments
    ///
    /// * `speed`: How fast animation should go. Default is 1.0. 2.0 is a speedup of 2.
    /// * `sensitivity`: How much impact an action has. Default is 10px for pressing the forward
    /// key for example.
    ///
    /// returns: InputController
    ///
    pub fn new(speed: f64, sensitivity: f64, zoom_sensitivity: f64) -> Self {
        Self {
            pinch_handler: PinchHandler::new(),
            pan_handler: PanHandler::new(),
            zoom_handler: ZoomHandler::new(zoom_sensitivity),
            tilt_handler: TiltHandler::new(speed, sensitivity),
            shift_handler: ShiftHandler::new(speed, sensitivity),
            query_handler: QueryHandler::new(),
        }
    }

    pub fn device_input(&mut self, _event: &DeviceEvent) -> bool {
        false
    }

    pub fn window_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let position: (f64, f64) = position.to_owned().into();
                self.pan_handler
                    .process_window_position(&Vector2::from(position), false);
                self.query_handler
                    .process_window_position(&Vector2::from(position), false);
                self.zoom_handler
                    .process_window_position(&Vector2::from(position), false);
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => {
                if !self.shift_handler.process_key_press(*key, *state) {
                    if !self.tilt_handler.process_key_press(*key, *state) {
                        self.zoom_handler.process_key_press(*key, *state)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            WindowEvent::Touch(touch) => match touch.phase {
                TouchPhase::Started => {
                    self.pan_handler.process_touch_start();
                    self.query_handler.process_touch_start();
                    true
                }
                TouchPhase::Ended => {
                    self.pan_handler.process_touch_end();
                    self.query_handler.process_touch_end();
                    true
                }
                TouchPhase::Moved => {
                    let position: (f64, f64) = touch.location.to_owned().into();
                    self.pan_handler
                        .process_window_position(&Vector2::from(position), true);
                    self.query_handler
                        .process_window_position(&Vector2::from(position), true);
                    self.zoom_handler
                        .process_window_position(&Vector2::from(position), true);
                    true
                }
                TouchPhase::Cancelled => false,
            },
            WindowEvent::MouseWheel { delta, .. } => {
                self.shift_handler.process_scroll(delta);
                self.zoom_handler.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.pan_handler.process_mouse_key_press(button, state);
                self.query_handler.process_mouse_key_press(button, state)
            }
            _ => false,
        }
    }
}

pub trait UpdateState {
    fn update_state(&mut self, state: &mut ViewState, dt: Duration);
}

impl UpdateState for InputController {
    #[tracing::instrument(skip_all)]
    fn update_state(&mut self, state: &mut ViewState, dt: Duration) {
        self.pan_handler.update_state(state, dt);
        self.pinch_handler.update_state(state, dt);
        self.zoom_handler.update_state(state, dt);
        self.tilt_handler.update_state(state, dt);
        self.shift_handler.update_state(state, dt);
        self.query_handler.update_state(state, dt);
    }
}
