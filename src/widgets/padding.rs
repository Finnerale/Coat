//! A widget that just adds padding during layout.

use crate::{
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx},
    event::{Event, LifeCycle},
    kurbo::{Insets, Point, Size},
    object::{Properties, RenderObject, RenderObjectInterface},
    tree::Children,
    ui::Ui,
    BoxConstraints,
};
use std::panic::Location;

/// A widget that just adds padding around its child.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Padding {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Properties for Padding {
    type Object = Self;
}

impl Padding {
    /// Create a new widget with the specified padding. This can either be an instance
    /// of [`kurbo::Insets`], a f64 for uniform padding, a 2-tuple for axis-uniform padding
    /// or 4-tuple with (left, top, right, bottom) values.
    ///
    /// # Examples
    ///
    /// Uniform padding:
    ///
    /// ```
    /// use coat::widgets::{Label, Padding};
    /// use coat::kurbo::Insets;
    ///
    /// let _: Padding = Padding::new(10.0);
    /// let _: Padding = Padding::new(Insets::uniform(10.0));
    /// ```
    ///
    /// Uniform padding across each axis:
    ///
    /// ```
    /// use coat::widgets::{Label, Padding};
    /// use coat::kurbo::Insets;
    ///
    /// let _: Padding = Padding::new((10.0, 20.0));
    /// let _: Padding = Padding::new(Insets::uniform_xy(10.0, 20.0));
    /// ```
    ///
    /// [`kurbo::Insets`]: https://docs.rs/kurbo/0.5.3/kurbo/struct.Insets.html
    pub fn new(insets: impl Into<Insets>) -> Padding {
        let insets = insets.into();
        Padding {
            left: insets.x0,
            right: insets.x1,
            top: insets.y0,
            bottom: insets.y1,
        }
    }

    #[track_caller]
    pub fn build(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let caller = Location::caller().into();
        ui.render_object(caller, self, content);
    }
}

impl RenderObject<Padding> for Padding {
    type Action = ();

    fn create(props: Padding) -> Self {
        props
    }

    fn update(&mut self, ctx: &mut UpdateCtx, props: Padding) {
        if self != &props {
            *self = props;
            ctx.request_layout();
        }
    }
}

impl RenderObjectInterface for Padding {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children) {
        children[0].event(ctx, event)
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle) {}

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        children: &mut Children,
    ) -> Size {
        bc.debug_check("Padding");
        let child = &mut children[0];

        let hpad = self.left + self.right;
        let vpad = self.top + self.bottom;

        let child_bc = bc.shrink((hpad, vpad));
        let size = child.layout(ctx, &child_bc);
        let origin = Point::new(self.left, self.top);
        child.set_origin(ctx, origin);

        let my_size = Size::new(size.width + hpad, size.height + vpad);
        let my_insets = child.compute_parent_paint_insets(my_size);
        ctx.set_paint_insets(my_insets);
        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        children[0].paint(ctx);
    }
}
