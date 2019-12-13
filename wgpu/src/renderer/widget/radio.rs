use crate::{Primitive, RadioStyle, Renderer};
use iced_native::{radio, Color, MouseCursor, Rectangle};

const SIZE: f32 = 28.0;
const DOT_SIZE: f32 = SIZE / 2.0;

impl radio::Renderer for Renderer {
    type WidgetStyle = RadioStyle;

    fn default_size(&self) -> u32 {
        SIZE as u32
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_selected: bool,
        is_mouse_over: bool,
        style: &Self::WidgetStyle,
        (label, _): Self::Output,
    ) -> Self::Output {
        let radio_box = Primitive::Quad {
            bounds,
            background: style.background,
            border_radius: (SIZE / 2.0) as u16,
            border_width: style.border_width,
            border_color: if is_mouse_over {
                style.get_border_hovered_color()
            } else {
                style.border_color
            },
        };

        (
            Primitive::Group {
                primitives: if is_selected {
                    let radio_circle = Primitive::Quad {
                        bounds: Rectangle {
                            x: bounds.x + DOT_SIZE / 2.0,
                            y: bounds.y + DOT_SIZE / 2.0,
                            width: bounds.width - DOT_SIZE,
                            height: bounds.height - DOT_SIZE,
                        },
                        background: style.dot_background,
                        border_radius: (DOT_SIZE / 2.0) as u16,
                        border_width: 0,
                        border_color: Color::BLACK,
                    };

                    vec![radio_box, radio_circle, label]
                } else {
                    vec![radio_box, label]
                },
            },
            if is_mouse_over {
                MouseCursor::Pointer
            } else {
                MouseCursor::OutOfBounds
            },
        )
    }
}
