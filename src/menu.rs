use crate::InputState;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use sdl2::mouse::MouseButton;

pub struct Text {
    pub text: String,
    pub color: Color,
    pub x: i32,
    pub y: i32,
    pub char_size: u32,
}

impl Text {
    // Creates a text object
    pub fn new(text_str: String, text_color: Color, posx: i32, posy: i32, ch_sz: u32) -> Self {
        Self {
            text: text_str,
            color: text_color,
            x: posx,
            y: posy,
            char_size: ch_sz,
        }
    } 

    // Width of the text in pixels
    pub fn width(&self) -> u32 {
        self.char_size * self.text.len() as u32
    }

    // Displays the text, pass in a texture creator so that
    // we can render the text onto that texture and then copy
    // it onto the screen
    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let surface = font
            .render(self.text.as_str())
            .blended(self.color)
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;

        let text_rect = Rect::new(self.x, self.y, self.width(), self.char_size * 2);

        canvas.copy(&texture, None, Some(text_rect))?;

        Ok(())
    }

    fn display_with_offset(
        &self,
        offset: &Point,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let surface = font
            .render(self.text.as_str())
            .blended(self.color)
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;

        let text_rect = Rect::new(
            self.x + offset.x,
            self.y + offset.y,
            self.width(),
            self.char_size * 2,
        );

        canvas.copy(&texture, None, Some(text_rect))?;

        Ok(())
    }
}

pub struct Bitmap {
    image_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Bitmap {}

pub struct MenuElement {
    bounding_rect: Rect,
    pub id: Option<String>,
    pub normal_color: Color,
    pub hover_color: Color,
    pub text: Vec<Text>,
    pub children: Vec<MenuElement>
}

impl MenuElement {
    //x, y is the center of the element
    //w, h is the width and height    
    pub fn new(x: i32, y: i32, w: u32, h: u32, normal: Color, hover: Color) -> Self {
        Self {
            bounding_rect: Rect::from_center(Point::new(x, y), w, h),
            normal_color: normal,
            hover_color: hover,
            text: Vec::new(),
            children: Vec::new(),
            id: None
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_owned()); 
    }

    pub fn mouse_hovering(&self, input_state: &InputState) -> bool {
        let (mousex, mousey) = input_state.mouse_pos();
        self.bounding_rect
            .contains_point(Point::new(mousex, mousey))
    }

    pub fn mouse_hovering_with_offset(&self, input_state: &InputState, offset: &Point) -> bool { 
        let (mousex, mousey) = input_state.mouse_pos();
        
        let bounding_rect = Rect::new(
            self.x() + offset.x,
            self.y() + offset.y,
            self.bounding_rect.width(),
            self.bounding_rect.height()
        );

        bounding_rect
            .contains_point(Point::new(mousex, mousey)) 
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        input_state: &InputState,
    ) -> Result<(), String> {
        // Draw background rect,
        // changes colors based on whether
        // the mouse is hovering over the
        // button so that way the user has
        // some amount of visual feedback
        if self.mouse_hovering(input_state) {
            canvas.set_draw_color(self.hover_color);
        } else {
            canvas.set_draw_color(self.normal_color);
        }

        canvas.fill_rect(self.bounding_rect)?;

        Ok(())
    }

    fn display_with_offset(
        &self,
        offset: &Point,
        canvas: &mut Canvas<Window>,
        input_state: &InputState,
    ) -> Result<(), String> {
        // Draw background rect,
        // changes colors based on whether
        // the mouse is hovering over the
        // button so that way the user has
        // some amount of visual feedback
        if self.mouse_hovering_with_offset(input_state, offset) {
            canvas.set_draw_color(self.hover_color);
        } else {
            canvas.set_draw_color(self.normal_color);
        }

        let bounding_rect = Rect::new(
            self.bounding_rect.x + offset.x,
            self.bounding_rect.y + offset.y,
            self.bounding_rect.width(),
            self.bounding_rect.height(),
        );

        canvas.fill_rect(bounding_rect)?;

        Ok(())
    }

    // Getter functions to make it nicer to get the x and y coordinates
    // of the element
    pub fn x(&self) -> i32 {
        self.bounding_rect.x
    }

    pub fn y(&self) -> i32 {
        self.bounding_rect.y
    }

    pub fn xy(&self) -> Point {
        Point::new(self.x(), self.y()) 
    }

    pub fn display_with_children(
        &self,
        canvas: &mut Canvas<Window>,
        input_state: &InputState,
    ) -> Result<(), String> {
        // Start with this element
        let mut all_children = vec![self];
        let mut child_offset = vec![Point::new(0, 0)];

        // Keep going until the list of menu elements to draw is 0
        // (we have drawn all the elements)
        while !all_children.is_empty() {
            // Get the top element
            let top = all_children[all_children.len() - 1];
            all_children.pop();
            // Get the offset of that element
            let offset = child_offset[child_offset.len() - 1];
            child_offset.pop();
            
            top.display_with_offset(&offset, canvas, input_state)?;

            // Draw all of the element's children
            for child in &top.children {
                all_children.push(child);
                child_offset.push(Point::new(offset.x + top.x(), offset.y + top.y()));
            }
        }

        Ok(())
    }

    pub fn display_text(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font
    ) -> Result<(), String> {
        for text in &self.text {
            text.display_with_offset(&self.xy(), canvas, texture_creator, font)?;
        }

        Ok(())
    }

    pub fn display_text_with_children(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font
    ) -> Result<(), String> {
        // Start with this element
        let mut all_children = vec![self];
        let mut child_offset = vec![self.xy()];

        // Keep going until the list of menu elements to draw is 0
        // (we have drawn all the elements)
        while !all_children.is_empty() {
            // Get the top element
            let top = all_children[all_children.len() - 1];
            all_children.pop();
            // Get the offset of that element
            let offset = child_offset[child_offset.len() - 1];
            child_offset.pop();

            for text in &top.text {
                text.display_with_offset(&offset, canvas, texture_creator, font)?;
            }

            // Draw all of the element's children
            for child in &top.children {
                all_children.push(child);
                child_offset.push(Point::new(offset.x + child.x(), offset.y + child.y()));
            }
        }

        Ok(())
    }

    // Returns the id of an element that has been clicked
    pub fn get_clicked(&self, input_state: &InputState, button: MouseButton) -> Option<String> {
         // Start with this element
        let mut all_children = vec![self];
        let mut child_offset = vec![Point::new(0, 0)]; 

        let mut id = None;

        // Keep going until the list of menu elements to draw is 0
        // (we have drawn all the elements)
        while !all_children.is_empty() {
            // Get the top element
            let top = all_children[all_children.len() - 1];
            all_children.pop();
            // Get the offset of that element
            let offset = child_offset[child_offset.len() - 1];
            child_offset.pop();

            if top.mouse_hovering_with_offset(input_state, &offset) &&
               input_state.mouse_button_is_clicked(button) {
                id = top.id.clone();
            }

            for child in &top.children {
                all_children.push(child);
                child_offset.push(Point::new(top.x() + offset.x, top.y() + offset.y));
            }
        } 

        id
    }
}
