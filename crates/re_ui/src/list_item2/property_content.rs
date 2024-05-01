use crate::list_item2::{ContentContext, ListItemContent};
use crate::{Icon, ReUi};
use eframe::emath::{Align, Align2};
use eframe::epaint::text::TextWrapping;
use egui::{NumExt, Response, Ui};

/// Closure to draw an icon left of the label.
type IconFn<'a> = dyn FnOnce(&ReUi, &mut egui::Ui, egui::Rect, egui::style::WidgetVisuals) + 'a;

/// Closure to draw the right column of the property.
type PropertyValueFn<'a> =
    dyn FnOnce(&ReUi, &mut egui::Ui, egui::style::WidgetVisuals) -> Option<egui::Response> + 'a;

struct PropertyActionButton<'a> {
    icon: &'static crate::icons::Icon,
    on_click: Box<dyn FnOnce() + 'a>,
}

/// [`ListItemContent`] to display property-like, two-column content, with the left column
/// containing a label (along with an optional icon) and the right column containing some custom
/// value (which may be editable).
pub struct PropertyContent<'a> {
    label: egui::WidgetText,
    icon_fn: Option<Box<IconFn<'a>>>,
    summary_only: bool,
    value_fn: Option<Box<PropertyValueFn<'a>>>,
    //TODO(ab): in the future, that should be a `Vec`, with some auto expanding mini-toolbar
    action_buttons: Option<PropertyActionButton<'a>>,
    /**/
    //TODO(ab): icon styling? link icon right of label? clickable label?
}

impl<'a> PropertyContent<'a> {
    pub fn new(label: impl Into<egui::WidgetText>) -> Self {
        Self {
            label: label.into(),
            icon_fn: None,
            summary_only: true,
            value_fn: None,
            action_buttons: None,
        }
    }

    /// Provide an [`Icon`] to be displayed on the left of the label.
    #[inline]
    pub fn with_icon(self, icon: &'a Icon) -> Self {
        self.with_icon_fn(|_, ui, rect, visuals| {
            let tint = visuals.fg_stroke.color;
            icon.as_image().tint(tint).paint_at(ui, rect);
        })
    }

    /// Provide a custom closure to draw an icon on the left of the item.
    #[inline]
    pub fn with_icon_fn<F>(mut self, icon_fn: F) -> Self
    where
        F: FnOnce(&ReUi, &mut egui::Ui, egui::Rect, egui::style::WidgetVisuals) + 'a,
    {
        self.icon_fn = Some(Box::new(icon_fn));
        self
    }

    /// Right aligned action button.
    ///
    /// Note: for aesthetics, space is always reserved for the action button.
    // TODO(ab): accept multiple calls for this function for multiple actions. In that case, a `…´
    // button should be displayed that turns into a mini-popup with all available actions
    // TODO(ab): if ALL item in a scope have no button active, then we could skip reserving the
    // space in the right margin.
    #[inline]
    pub fn action_button(
        mut self,
        icon: &'static crate::icons::Icon,
        on_click: impl FnOnce() + 'a,
    ) -> Self {
        self.action_buttons = Some(PropertyActionButton {
            icon,
            on_click: Box::new(on_click),
        });
        self
    }

    /// Display value only for leaf or collapsed items.
    ///
    /// When enabled, the value for this item is not displayed for uncollapsed hierarchical items.
    /// This is convenient when the value serves are a summary of the child content, which doesn't
    /// need to be displayed when said content is visible.
    ///
    /// Enabled by default.
    #[inline]
    pub fn summary_only(mut self, summary_only: bool) -> Self {
        self.summary_only = summary_only;
        self
    }

    /// Provide a closure to draw the content of the right column.
    #[inline]
    pub fn value_fn<F>(mut self, value_fn: F) -> Self
    where
        F: FnOnce(&ReUi, &mut egui::Ui, egui::style::WidgetVisuals) -> Option<egui::Response> + 'a,
    {
        self.value_fn = Some(Box::new(value_fn));
        self
    }

    //
    // Bunch of helpers with concrete implementation of value fn
    //

    /// Show a read-only boolean in the value column.
    #[inline]
    pub fn value_bool(self, mut b: bool) -> Self {
        self.value_fn(move |_, ui: &mut Ui, _| {
            Some(ui.add_enabled(false, crate::toggle_switch(15.0, &mut b)))
        })
    }

    /// Show an editable boolean in the value column.
    #[inline]
    pub fn value_bool_mut(self, b: &'a mut bool) -> Self {
        self.value_fn(|_, ui: &mut Ui, _| {
            ui.visuals_mut().widgets.hovered.expansion = 0.0;
            ui.visuals_mut().widgets.active.expansion = 0.0;

            Some(ui.add(crate::toggle_switch(15.0, b)))
        })
    }

    /// Show a static text in the value column.
    #[inline]
    pub fn value_text(self, text: impl Into<egui::WidgetText> + 'a) -> Self {
        self.value_fn(move |_, ui, _| Some(ui.label(text.into())))
    }

    /// Show an editable text in the value column.
    #[inline]
    pub fn value_text_mut(self, text: &'a mut String) -> Self {
        self.value_fn(|_, ui, _| Some(ui.text_edit_singleline(text)))
    }

    /// Show a read-only color in the value column.
    #[inline]
    pub fn value_color(self, color: &'a [u8; 4]) -> Self {
        self.value_fn(|_, ui, _| {
            let [r, g, b, a] = color;
            let color = egui::Color32::from_rgba_unmultiplied(*r, *g, *b, *a);
            let response = egui::color_picker::show_color(ui, color, ui.spacing().interact_size);
            response.on_hover_text(format!("Color #{r:02x}{g:02x}{b:02x}{a:02x}"));
            None
        })
    }

    /// Show an editable color in the value column.
    #[inline]
    pub fn value_color_mut(self, color: &'a mut [u8; 4]) -> Self {
        self.value_fn(|_, ui: &mut egui::Ui, _| {
            ui.visuals_mut().widgets.hovered.expansion = 0.0;
            ui.visuals_mut().widgets.active.expansion = 0.0;
            Some(ui.color_edit_button_srgba_unmultiplied(color))
        })
    }
}

impl ListItemContent for PropertyContent<'_> {
    fn ui(
        self: Box<Self>,
        re_ui: &ReUi,
        ui: &mut Ui,
        context: &ContentContext<'_>,
    ) -> Option<Response> {
        let Self {
            label,
            icon_fn,
            summary_only,
            value_fn,
            action_buttons,
        } = *self;

        // We always reserve space for the action button(s), even if there are none.
        let action_button_rect = egui::Rect::from_center_size(
            context.rect.right_center() - egui::vec2(ReUi::small_icon_size().x / 2., 0.0),
            ReUi::small_icon_size() + egui::vec2(1.0, 1.0), // padding is needed for the buttons
        );

        let content_width =
            (context.rect.width() - action_button_rect.width() - ReUi::text_to_icon_padding())
                .at_least(0.0);

        //TODO(ab): adaptable columns
        let column_width = ((content_width - ReUi::text_to_icon_padding()) / 2.).at_least(0.0);

        let icon_rect = egui::Rect::from_center_size(
            context.rect.left_center() + egui::vec2(ReUi::small_icon_size().x / 2., 0.0),
            ReUi::small_icon_size(),
        );

        let mut label_rect = egui::Rect::from_min_size(
            context.rect.left_top(),
            egui::vec2(column_width, context.rect.height()),
        );
        if icon_fn.is_some() {
            label_rect.min.x += icon_rect.width() + ReUi::text_to_icon_padding();
        }

        let value_rect = egui::Rect::from_min_size(
            context.rect.left_top() + egui::vec2(column_width + ReUi::text_to_icon_padding(), 0.0),
            egui::vec2(column_width, context.rect.height()),
        );

        let visuals = ui
            .style()
            .interact_selectable(context.response, context.list_item.selected);

        // Draw icon
        if let Some(icon_fn) = icon_fn {
            icon_fn(re_ui, ui, icon_rect, visuals);
        }

        let button_response = if let Some(action_button) = action_buttons {
            let mut child_ui = ui.child_ui(
                action_button_rect.expand(2.0),
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            );
            let button_response = re_ui.small_icon_button(&mut child_ui, action_button.icon);
            if button_response.clicked() {
                (action_button.on_click)();
            }
            Some(button_response)
        } else {
            None
        };

        // Draw label
        let mut layout_job =
            label.into_layout_job(ui.style(), egui::FontSelection::Default, Align::LEFT);
        layout_job.wrap = TextWrapping::truncate_at_width(label_rect.width());
        let galley = ui.fonts(|fonts| fonts.layout_job(layout_job));

        // this happens here to avoid cloning the text
        context.response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                context.list_item.selected,
                galley.text(),
            )
        });

        let text_pos = Align2::LEFT_CENTER
            .align_size_within_rect(galley.size(), label_rect)
            .min;
        ui.painter().galley(text_pos, galley, visuals.text_color());

        // Draw value
        let should_show_value = context
            .list_item
            .collapse_openness
            .map_or(true, |o| o == 0.0)
            || !summary_only;
        let value_response = if let Some(value_fn) = value_fn {
            if should_show_value {
                let mut child_ui =
                    ui.child_ui(value_rect, egui::Layout::left_to_right(egui::Align::Center));
                value_fn(re_ui, &mut child_ui, visuals)
            } else {
                None
            }
        } else {
            None
        };

        // Make a union of all (possibly) interactive elements
        match (value_response, button_response) {
            (Some(a), Some(b)) => Some(a | b),
            (Some(a), None) | (None, Some(a)) => Some(a),
            (None, None) => None,
        }
    }
}
