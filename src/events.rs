use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
enum ButtonState {
    Released,
    Clicked,
    Held,
}

pub struct InputState {
    key_state: HashMap<Scancode, ButtonState>,
    mouse_button_state: HashMap<MouseButton, ButtonState>,
    mousex: i32,
    mousey: i32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            key_state: HashMap::new(),
            mouse_button_state: HashMap::new(),
            mousex: 0,
            mousey: 0,
        }
    }

    pub fn key_is_clicked(&self, key: Scancode) -> bool {
        matches!(self.key_state.get(&key), Some(ButtonState::Clicked))
    }

    pub fn mouse_button_is_clicked(&self, button: MouseButton) -> bool {
        matches!(
            self.mouse_button_state.get(&button),
            Some(ButtonState::Clicked)
        )
    }

    pub fn key_is_held(&self, key: Scancode) -> bool {
        matches!(
            self.key_state.get(&key),
            Some(ButtonState::Clicked) | Some(ButtonState::Held)
        )
    }

    pub fn mouse_button_is_held(&self, button: MouseButton) -> bool {
        matches!(
            self.mouse_button_state.get(&button),
            Some(ButtonState::Clicked) | Some(ButtonState::Held)
        )
    }

    pub fn update(&mut self, event_pump: &EventPump) {
        let keyboard = event_pump.keyboard_state();
        let mouse = event_pump.mouse_state();

        //Check for any new keys that were pressed, set those to be "clicked"
        for scancode in keyboard.pressed_scancodes() {
            match self.key_state.get(&scancode) {
                Some(ButtonState::Released) | None => {
                    self.key_state.insert(scancode, ButtonState::Clicked);
                }
                Some(ButtonState::Clicked) => {
                    //If it was "clicked" in the previous frame, set it to be held
                    self.key_state.insert(scancode, ButtonState::Held);
                }
                _ => {}
            }
        }

        //Check for any new buttons that were pressed, set those to be "clicked"
        for button in mouse.pressed_mouse_buttons() {
            match self.mouse_button_state.get(&button) {
                Some(ButtonState::Released) | None => {
                    self.mouse_button_state.insert(button, ButtonState::Clicked);
                }
                Some(ButtonState::Clicked) => {
                    //If it was "clicked" in the previous frame, set it to be held
                    self.mouse_button_state.insert(button, ButtonState::Held);
                }
                _ => {}
            }
        }

        //Check for keys that got released
        let mut released_keys = vec![];

        for (scancode, button_state) in self.key_state.iter() {
            if (*button_state == ButtonState::Held || *button_state == ButtonState::Clicked)
                && !keyboard.is_scancode_pressed(*scancode)
            {
                released_keys.push(*scancode);
            }
        }

        //Check for mouse buttons that got released
        let mut released_buttons = vec![];

        for (button, button_state) in self.mouse_button_state.iter() {
            if (*button_state == ButtonState::Held || *button_state == ButtonState::Clicked)
                && !mouse.is_mouse_button_pressed(*button)
            {
                released_buttons.push(*button);
            }
        }

        released_keys.into_iter().for_each(|scancode| {
            self.key_state.insert(scancode, ButtonState::Released);
        });

        released_buttons.into_iter().for_each(|button| {
            self.mouse_button_state
                .insert(button, ButtonState::Released);
        });

        //Update mouse position
        self.mousex = mouse.x();
        self.mousey = mouse.y();
    }

    pub fn mouse_pos(&self) -> (i32, i32) {
        (self.mousex, self.mousey)
    }
}

pub fn can_quit(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        if let Event::Quit { .. } = event {
            return true;
        }
    }

    false
}
