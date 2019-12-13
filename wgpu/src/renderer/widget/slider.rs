use crate::{Primitive, Renderer, SliderStyle};
use iced_native::{slider, Color, MouseCursor, Point, Rectangle};

impl slider::Renderer for Renderer {
    type WidgetStyle = SliderStyle;

    fn height(&self) -> u32 {
        30
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        is_dragging: bool,
        style: &SliderStyle,
    ) -> Self::Output {
        let (range_start, range_end) = range.into_inner();

        let handle_offset = (bounds.width - style.handle_width)
            * ((value - range_start) / (range_end - range_start).max(1.0));

        let rail_y = bounds.y + (bounds.height / 2.0).round();

        let handle_bounds = Rectangle {
            x: bounds.x + handle_offset.round(),
            y: rail_y - style.handle_height / 2.0,
            width: style.handle_width,
            height: style.handle_height,
        };

        let is_mouse_over = handle_bounds.contains(cursor_position);

        let (rail_top, rail_bottom) = (
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y - 2.0,
                    width: bounds.width,
                    height: 2.0,
                },
                background: style.rail_top_color.into(),
                border_radius: 0,
                border_width: 0,
                border_color: Color::BLACK
            },
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y,
                    width: bounds.width,
                    height: 2.0,
                },
                background: style.rail_bottom_color.into(),
                border_radius: 1,
                border_width: 0,
                border_color: Color::BLACK
            },
        );

        let handle_background = if is_dragging {
            style.get_handle_pressed_background()
        } else {
            if is_mouse_over {
                style.get_handle_hovered_background()
            } else {
                style.handle_background
            }
        };

        let handle_border_color = if is_dragging {
            style.get_handle_pressed_border_color()
        } else {
            if is_mouse_over {
                style.get_handle_hovered_border_color()
            } else {
                style.handle_border_color
            }
        };

        let handle = Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset.round(),
                y: rail_y - style.handle_height / 2.0,
                width: style.handle_width,
                height: style.handle_height,
            },
            background: handle_background,
            border_radius: style.handle_corner_radius,
            border_width: style.handle_border_width,
            border_color: handle_border_color
        };

        (
            Primitive::Group {
                primitives: vec![rail_top, rail_bottom, handle],
            },
            if is_dragging {
                MouseCursor::Grabbing
            } else if is_mouse_over {
                MouseCursor::Grab
            } else {
                MouseCursor::OutOfBounds
            },
        )
    }
}
