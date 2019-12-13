use crate::{ButtonStyle, Primitive, Renderer};
use iced_native::{button, Background, MouseCursor, Point, Rectangle};

impl button::Renderer for Renderer {
    type WidgetStyle = ButtonStyle;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        is_pressed: bool,
        style: &ButtonStyle,
        (content, _): Self::Output,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        // TODO: Render proper shadows
        let pressed_offset = if is_pressed { 1.0 } else { 0.0 };

        let background = if is_mouse_over {
            if is_pressed {
                style.get_pressed_background()
            } else {
                style.get_hovered_background()
            }
        } else {
            style.background
        };

        let border_color = if is_mouse_over {
            if is_pressed {
                style.get_pressed_border_color()
            } else {
                style.get_hovered_border_color()
            }
        } else {
            style.border_color
        };

        let pointer = if is_mouse_over {
            MouseCursor::Pointer
        } else {
            MouseCursor::OutOfBounds
        };

        let primitives = Primitive::Group {
            primitives: vec![
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds.x + pressed_offset,
                        y: bounds.y + pressed_offset,
                        width: bounds.width - pressed_offset,
                        height: bounds.height - pressed_offset,
                    },
                    background: match background {
                        Some(bg) => bg,
                        None => Background::Color([0.0, 0.0, 0.0, 0.0].into()),
                    },
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color,
                },
                content,
            ],
        };

        (primitives, pointer)
    }
}
