//! A widget with predefined size.

use crate::object::prelude::*;
use std::f64::INFINITY;

/// A widget with predefined size.
///
/// If given a child, this widget forces its child to have a specific width and/or height
/// (assuming values are permitted by this widget's parent). If either the width or height is not set,
/// this widget will size itself to match the child's size in that dimension.
///
/// If not given a child, SizedBox will try to size itself as close to the specified height
/// and width as possible given the parent's constraints. If height or width is not set,
/// it will be treated as zero.
#[derive(Debug, Default, PartialEq)]
pub struct SizedBox {
    width: Option<f64>,
    height: Option<f64>,
}

impl Properties for SizedBox {
    type Object = SizedBox;
}

impl SizedBox {
    /// Construct container with child, and both width and height not set.
    pub fn new() -> Self {
        Self::default()
    }

    #[track_caller]
    pub fn build(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let caller = Location::caller().into();
        ui.render_object(caller, self, content);
    }

    #[track_caller]
    pub fn empty(self, cx: &mut Ui) {
        let caller = Location::caller().into();
        cx.render_object(caller, self, |_| {});
    }

    /// Set container's width.
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set container's height.
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Expand container to fit the parent.
    ///
    /// Only call this method if you want your widget to occupy all available
    /// space. If you only care about expanding in one of width or height, use
    /// [`expand_width`] or [`expand_height`] instead.
    ///
    /// [`expand_height`]: #method.expand_height
    /// [`expand_width`]: #method.expand_width
    pub fn expand(mut self) -> Self {
        self.width = Some(INFINITY);
        self.height = Some(INFINITY);
        self
    }

    /// Expand the container on the x-axis.
    ///
    /// This will force the child to have maximum width.
    pub fn expand_width(mut self) -> Self {
        self.width = Some(INFINITY);
        self
    }

    /// Expand the container on the y-axis.
    ///
    /// This will force the child to have maximum height.
    pub fn expand_height(mut self) -> Self {
        self.height = Some(INFINITY);
        self
    }

    fn child_constraints(&self, bc: &BoxConstraints) -> BoxConstraints {
        // if we don't have a width/height, we don't change that axis.
        // if we have a width/height, we clamp it on that axis.
        let (min_width, max_width) = match self.width {
            Some(width) => {
                let w = width.max(bc.min().width).min(bc.max().width);
                (w, w)
            }
            None => (bc.min().width, bc.max().width),
        };

        let (min_height, max_height) = match self.height {
            Some(height) => {
                let h = height.max(bc.min().height).min(bc.max().height);
                (h, h)
            }
            None => (bc.min().height, bc.max().height),
        };

        BoxConstraints::new(
            Size::new(min_width, min_height),
            Size::new(max_width, max_height),
        )
    }
}

impl RenderObject<SizedBox> for SizedBox {
    type Action = ();

    fn create(props: SizedBox) -> Self {
        props
    }

    fn update(&mut self, ctx: &mut UpdateCtx, props: SizedBox) {
        if self != &props {
            ctx.request_layout();
            *self = props;
        }
    }
}

impl RenderObjectInterface for SizedBox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children) {
        if !children.is_empty() {
            children[0].event(ctx, event);
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle) {}

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        children: &mut Children,
    ) -> Size {
        bc.debug_check("SizedBox");

        let child_bc = self.child_constraints(bc);
        let size = match children.get_mut(0) {
            Some(inner) => inner.layout(ctx, &child_bc),
            None => bc.constrain((self.width.unwrap_or(0.0), self.height.unwrap_or(0.0))),
        };

        if size.width.is_infinite() {
            log::warn!("SizedBox is returning an infinite width.");
        }

        if size.height.is_infinite() {
            log::warn!("SizedBox is returning an infinite height.");
        }

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        if !children.is_empty() {
            children[0].paint(ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand() {
        let expand = SizedBox::new().expand();
        let bc = BoxConstraints::tight(Size::new(400., 400.)).loosen();
        let child_bc = expand.child_constraints(&bc);
        assert_eq!(child_bc.min(), Size::new(400., 400.,));
    }

    #[test]
    fn no_width() {
        let expand = SizedBox::new().height(200.);
        let bc = BoxConstraints::tight(Size::new(400., 400.)).loosen();
        let child_bc = expand.child_constraints(&bc);
        assert_eq!(child_bc.min(), Size::new(0., 200.,));
        assert_eq!(child_bc.max(), Size::new(400., 200.,));
    }
}
