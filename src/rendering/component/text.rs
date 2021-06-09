use crate::{
    component::text::{State, TextState},
    layout::{LayoutDirection, LayoutState},
    rendering::{
        consts::{DEFAULT_TEXT_SIZE, PADDING, TEXT_ALIGN_TOP},
        resource::ResourceAllocator,
        solid, RenderContext,
    },
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl ResourceAllocator>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    match &component.text {
        TextState::Center(text) => context.render_text_centered(
            text,
            PADDING,
            width - PADDING,
            [0.5 * width, TEXT_ALIGN_TOP],
            DEFAULT_TEXT_SIZE,
            solid(
                &component
                    .left_center_color
                    .unwrap_or(layout_state.text_color),
            ),
        ),
        TextState::Split(left, right) => context.render_key_value_component(
            &left,
            &[],
            &right,
            false,
            [width, height],
            component
                .left_center_color
                .unwrap_or(layout_state.text_color),
            component.right_color.unwrap_or(layout_state.text_color),
            component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
        ),
    }
}
