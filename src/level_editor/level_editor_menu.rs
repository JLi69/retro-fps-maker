use crate::menu::{MenuElement, Text};
use crate::InputState;
use sdl2::image::LoadTexture;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

pub struct LevelEditorMenu {
    pub menu: MenuElement,
    pub selected: u8,
    pub icon_sz: u32,
}

impl LevelEditorMenu {
    pub fn new() -> Self {
        let mut level_editor_menu = MenuElement::new(
            800,
            320,
            320,
            640,
            Color::RGB(32, 32, 32),
            Color::RGB(32, 32, 32),
        );

        // Play level button
        {
            let mut play_button = MenuElement::new(
                16 + 60,
                640 - 16 - 25,
                120,
                50,
                Color::RGB(0, 200, 0),
                Color::RGB(0, 180, 0)
            );

            play_button.set_id("play_button");
            play_button.text.push(Text::new("Play Level", Color::WHITE, 8, 12, 10));

            level_editor_menu.children.push(play_button);
        }

        {
            let mut play_button = MenuElement::new(
                16 + 60,
                640 - 16 - 25 - 50 - 16,
                120,
                50,
                Color::RGB(0, 200, 0),
                Color::RGB(0, 180, 0)
            );

            play_button.set_id("save_button");
            play_button.text.push(Text::new("Save Level", Color::WHITE, 8, 12, 10));

            level_editor_menu.children.push(play_button);
        }

        {
            let mut play_button = MenuElement::new(
                16 + 60 + 120 + 16,
                640 - 16 - 25 - 50 - 16,
                120,
                50,
                Color::RGB(0, 200, 0),
                Color::RGB(0, 180, 0)
            );

            play_button.set_id("load_button");
            play_button.text.push(Text::new("Load Level", Color::WHITE, 8, 12, 10));

            level_editor_menu.children.push(play_button);
        }

        level_editor_menu
            .text
            .push(Text::new("Level Editor", Color::WHITE, 16, 16, 12)); 

        Self {
            menu: level_editor_menu,
            selected: 1,
            icon_sz: 32,
        }
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        input_state: &InputState,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
        textures: &[Texture],
    ) -> Result<(), String> {
        self.menu.display_with_children(canvas, input_state)?;
        self.menu.display_text_with_children(canvas, texture_creator, font)?;

        let (mousex, mousey) = input_state.mouse_pos();

        let mut x = 16;
        let mut y = 48;
        for (i, texture) in textures.iter().enumerate() {
            let icon_rect = Rect::new(
                x + self.menu.x(),
                y + self.menu.y(),
                self.icon_sz,
                self.icon_sz,
            );

            canvas.copy(texture, None, icon_rect)?;

            if i + 1 == self.selected as usize {
                canvas.set_draw_color(Color::WHITE);
                canvas.draw_rect(icon_rect)?;
            }

            if icon_rect.contains_point(Point::new(mousex, mousey)) {
                canvas.set_draw_color(Color::YELLOW);
                canvas.draw_rect(icon_rect)?;
            }

            x += self.icon_sz as i32;
            if x >= (self.menu.width() - self.icon_sz) as i32 {
                y += self.icon_sz as i32;
                x = 16;
            }
        }

        Ok(())
    }

    pub fn handle_mouse_input(&mut self, input_state: &InputState, tile_count: u8) {
        let mut x = 16;
        let mut y = 48;

        let (mousex, mousey) = input_state.mouse_pos();
        for tile in 0..tile_count {
            let icon_rect = Rect::new(
                x + self.menu.x(),
                y + self.menu.y(),
                self.icon_sz,
                self.icon_sz,
            );

            if icon_rect.contains_point(Point::new(mousex, mousey))
                && input_state.mouse_button_is_clicked(MouseButton::Left)
            {
                self.selected = tile + 1;
                return;
            }

            x += self.icon_sz as i32;
            if x >= (self.menu.width() - self.icon_sz) as i32 {
                y += self.icon_sz as i32;
                x = 16;
            }
        }
    }
}

pub fn load_default_assets(texture_creator: &TextureCreator<WindowContext>) -> Vec<Texture> {
    let default_texture_path = vec![
        "assets/images/textures/test-texture.png",
        "assets/images/textures/bricks.png",
        "assets/images/textures/wall1.png",
        "assets/images/textures/wall2.png",
        "assets/images/textures/wall3.png",
        "assets/images/textures/wall4.png",
        "assets/images/textures/whitewall.png",
        "assets/images/textures/box.png",
        "assets/images/textures/stripeblock.png",
    ];

    let mut textures = vec![];

    for path in default_texture_path {
        let res = texture_creator.load_texture(path);

        match res {
            Ok(tex) => textures.push(tex),
            Err(msg) => {
                eprintln!("{msg}")
            }
        }
    }

    textures
}
