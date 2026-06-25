use gpui::{
    div, prelude::*, px, size, App, Application, Bounds, Context, FontWeight, Render, SharedString,
    Window, WindowBounds, WindowOptions,
};

use rui::{
    h_flex, prelude::*, v_flex, ActiveTheme, AlertModal, Assets, Avatar, Button, ButtonVariant,
    Card, Checkbox, Color, Divider, DividerTitle, Headline, HeadlineSize, Icon, IconName, IconSize,
    Indicator, Label, Link, RadioGroup, Root, Size, Switch, Text, Theme, ToggleState,
};

// ── Component Page Enum ──────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum ComponentPage {
    Button,
    Icon,
    Label,
    Link,
    Divider,
    Avatar,
    Card,
    Checkbox,
    Radio,
    Switch,
    Text,
    Tooltip,
    Headline,
    Indicator,
    Layout,
    AlertModal,
}

impl ComponentPage {
    fn name(&self) -> &'static str {
        match self {
            ComponentPage::Button => "Button",
            ComponentPage::Icon => "Icon",
            ComponentPage::Label => "Label",
            ComponentPage::Link => "Link",
            ComponentPage::Divider => "Divider",
            ComponentPage::Avatar => "Avatar",
            ComponentPage::Card => "Card",
            ComponentPage::Checkbox => "Checkbox",
            ComponentPage::Radio => "Radio",
            ComponentPage::Switch => "Switch",
            ComponentPage::Text => "Text",
            ComponentPage::Tooltip => "Tooltip",
            ComponentPage::Headline => "Headline",
            ComponentPage::Indicator => "Indicator",
            ComponentPage::Layout => "Layout",
            ComponentPage::AlertModal => "AlertModal",
        }
    }

    fn group(&self) -> &'static str {
        match self {
            ComponentPage::Button
            | ComponentPage::Icon
            | ComponentPage::Label
            | ComponentPage::Link
            | ComponentPage::Divider => "基础组件",
            ComponentPage::Avatar | ComponentPage::Card | ComponentPage::Text => "数据展示",
            ComponentPage::Checkbox | ComponentPage::Radio | ComponentPage::Switch => "表单组件",
            ComponentPage::Tooltip | ComponentPage::AlertModal => "反馈组件",
            ComponentPage::Headline | ComponentPage::Indicator | ComponentPage::Layout => "布局",
        }
    }

    fn all() -> Vec<ComponentPage> {
        vec![
            ComponentPage::Button,
            ComponentPage::Icon,
            ComponentPage::Label,
            ComponentPage::Link,
            ComponentPage::Divider,
            ComponentPage::Avatar,
            ComponentPage::Card,
            ComponentPage::Checkbox,
            ComponentPage::Radio,
            ComponentPage::Switch,
            ComponentPage::Text,
            ComponentPage::Tooltip,
            ComponentPage::Headline,
            ComponentPage::Indicator,
            ComponentPage::Layout,
            ComponentPage::AlertModal,
        ]
    }

    fn groups() -> Vec<(&'static str, Vec<ComponentPage>)> {
        let all = Self::all();
        let mut groups: Vec<(&'static str, Vec<ComponentPage>)> = Vec::new();
        for page in all {
            let group_name = page.group();
            if let Some(group) = groups.iter_mut().find(|g| g.0 == group_name) {
                group.1.push(page);
            } else {
                groups.push((group_name, vec![page]));
            }
        }
        groups
    }

    fn icon(&self) -> IconName {
        match self {
            ComponentPage::Button => IconName::SquareDot,
            ComponentPage::Icon => IconName::Sparkle,
            ComponentPage::Label => IconName::TextSnippet,
            ComponentPage::Link => IconName::Link,
            ComponentPage::Divider => IconName::Split,
            ComponentPage::Avatar => IconName::Person,
            ComponentPage::Card => IconName::Blocks,
            ComponentPage::Checkbox => IconName::Check,
            ComponentPage::Radio => IconName::Circle,
            ComponentPage::Switch => IconName::Sliders,
            ComponentPage::Text => IconName::Font,
            ComponentPage::Tooltip => IconName::Info,
            ComponentPage::Headline => IconName::Book,
            ComponentPage::Indicator => IconName::Indicator,
            ComponentPage::Layout => IconName::PanelLeft,
            ComponentPage::AlertModal => IconName::Warning,
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────────────────

struct Gallery {
    active_page: ComponentPage,
    checkbox_checked: bool,
    switch_checked: bool,
    radio_selected: usize,
}

impl Gallery {
    fn new() -> Self {
        Self {
            active_page: ComponentPage::Button,
            checkbox_checked: true,
            switch_checked: true,
            radio_selected: 1,
        }
    }

    fn render_sidebar(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let bg = cx.theme().colors.bg;
        let border = cx.theme().colors.border_variant;
        let muted_color = cx.theme().colors.text_muted;
        let primary = cx.theme().colors.primary;

        v_flex()
            .id("sidebar")
            .w(px(240.))
            .h_full()
            .bg(bg)
            .border_r_1()
            .border_color(border)
            .flex_shrink_0()
            .child(
                // Sidebar Header
                v_flex()
                    .px_3()
                    .py_4()
                    .gap_2()
                    .border_b_1()
                    .border_color(border)
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::ZedAssistant)
                                    .size(IconSize::Large)
                                    .color(primary),
                            )
                            .child(
                                Text::new("Rui Components")
                                    .font_weight(FontWeight::BOLD)
                                    .text_size(rems_from_px(16.)),
                            ),
                    ),
            )
            .child(
                // Sidebar Content
                div()
                    .flex_1()
                    .py_2()
                    .map(|container| {
                        let mut c = container;
                        for (group_name, pages) in &ComponentPage::groups() {
                            c = c.child(
                                v_flex()
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .text_size(rems_from_px(11.))
                                            .text_color(muted_color)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child(group_name.to_string()),
                                    )
                                    .children(pages.iter().map(|page| {
                                        let is_active = self.active_page == *page;
                                        let active_page = *page;
                                        Button::new(SharedString::from(format!(
                                            "nav-{}",
                                            page.name()
                                        )))
                                        .ghost()
                                        .child(
                                            h_flex()
                                                .items_center()
                                                .gap_2()
                                                .child(
                                                    Icon::new(page.icon())
                                                        .size(IconSize::Small),
                                                )
                                                .child(page.name().to_string()),
                                        )
                                        .when(is_active, |this| {
                                            this.text_color(primary)
                                        })
                                        .w_full()
                                        .on_click(
                                            cx.listener(move |gallery, _, _, _| {
                                                gallery.active_page = active_page;
                                            }),
                                        )
                                    })),
                            );
                        }
                        c
                    }),
            )
            // Sidebar Footer
            .child(
                h_flex()
                    .px_3()
                    .py_2()
                    .border_t_1()
                    .border_color(border)
                    .gap_2()
                    .items_center()
                    .child(
                        Icon::new(IconName::Settings)
                            .size(IconSize::Small)
                            .color(muted_color),
                    )
                    .child(
                        Text::new(format!(
                            "{} components",
                            ComponentPage::all().len()
                        ))
                        .text_size(rems_from_px(12.))
                        .color(muted_color),
                    ),
            )
    }

    fn render_content(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let page = self.active_page;
        v_flex()
            .flex_1()
            .h_full()
            .bg(cx.theme().colors.bg)
            .child(
                // Content Header
                h_flex()
                    .px_6()
                    .py_4()
                    .border_b_1()
                    .border_color(cx.theme().colors.border_variant)
                    .items_center()
                    .justify_between()
                    .child(
                        v_flex().gap_1().child(
                            Text::new(page.name())
                                .text_size(rems_from_px(20.))
                                .font_weight(FontWeight::BOLD),
                        ),
                    )
                    .child(
                        Text::new(format!("Group: {}", page.group()))
                            .text_size(rems_from_px(12.))
                            .color(cx.theme().colors.text_muted),
                    ),
            )
            .child(
                // Content Body
                div()
                    .flex_1()
                    .p_6()
                    .child(match page {
                        ComponentPage::Button => self.render_button_story(cx).into_any_element(),
                        ComponentPage::Icon => self.render_icon_story(cx).into_any_element(),
                        ComponentPage::Label => self.render_label_story(cx).into_any_element(),
                        ComponentPage::Link => self.render_link_story(cx).into_any_element(),
                        ComponentPage::Divider => self.render_divider_story(cx).into_any_element(),
                        ComponentPage::Avatar => self.render_avatar_story(cx).into_any_element(),
                        ComponentPage::Card => self.render_card_story(cx).into_any_element(),
                        ComponentPage::Checkbox => self.render_checkbox_story(cx).into_any_element(),
                        ComponentPage::Radio => self.render_radio_story(cx).into_any_element(),
                        ComponentPage::Switch => self.render_switch_story(cx).into_any_element(),
                        ComponentPage::Text => self.render_text_story(cx).into_any_element(),
                        ComponentPage::Tooltip => self.render_tooltip_story(cx).into_any_element(),
                        ComponentPage::Headline => self.render_headline_story(cx).into_any_element(),
                        ComponentPage::Indicator => self.render_indicator_story(cx).into_any_element(),
                        ComponentPage::Layout => self.render_layout_story(cx).into_any_element(),
                        ComponentPage::AlertModal => self.render_alert_story(cx).into_any_element(),
                    }),
            )
    }

    // ── Individual Component Demos ────────────────────────────────────────────

    fn section_title(title: impl Into<SharedString>) -> impl IntoElement {
        DividerTitle::new(Text::new(title))
    }

    fn button_variant_row(variant_name: impl Into<SharedString>, variant: ButtonVariant) -> impl IntoElement {
        h_flex()
            .items_center()
            .gap_2()
            .flex_wrap()
            .child(Text::new(variant_name).w(px(64.)))
            .child(Button::new("variant-default").text("Default").variant(variant))
            .child(Button::new("variant-primary").text("Primary").variant(variant).primary())
            .child(Button::new("variant-secondary").text("Secondary").variant(variant).secondary())
            .child(Button::new("variant-success").text("Success").variant(variant).success())
            .child(Button::new("variant-warning").text("Warning").variant(variant).warning())
            .child(Button::new("variant-danger").text("Danger").variant(variant).danger())
            .child(
                Button::new("variant-disabled")
                    .text("Disabled")
                    .variant(variant)
                    .disabled(true),
            )
    }

    fn render_button_story(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Variants")
            Self::button_variant_row("Solid", ButtonVariant::Solid)
            Self::button_variant_row("Surface", ButtonVariant::Surface)
            Self::button_variant_row("Soft", ButtonVariant::Soft)
            Self::button_variant_row("Outline", ButtonVariant::Outline)
            Self::button_variant_row("Ghost", ButtonVariant::Ghost)
            Self::button_variant_row("Plain", ButtonVariant::Plain)

            Self::section_title("Sizes")
            h_flex().items_center().gap_2().child(Button::new("xs").text("XSmall").size(Size::XSmall)).child(Button::new("sm").text("Small").size(Size::Small)).child(Button::new("md").text("Medium")).child(Button::new("lg").text("Large").size(Size::Large))

            Self::section_title("With Icons")
            h_flex().items_center().gap_2().flex_wrap().child(Button::new("icon-only").icon(IconName::Mic)).child(Button::new("icon-danger").icon(IconName::Close).danger()).child(Button::new("icon-text").text("Mic").icon(IconName::Mic)).child(Button::new("icon-right").text("Right").icon(IconName::Mic).icon_right()).child(Button::new("icon-color").text("Color").icon(Icon::new(IconName::Mic).color(Color::red()))).child(Button::new("loading").text("Loading").loading(true))

            Self::section_title("Button Group")
            h_flex().items_center().gap_2().child(rui::ButtonGroup::new().children(vec!["One","Two","Three"])).child(rui::ButtonGroup::new().children(vec!["One","Two","Three"]).soft().secondary()).child(rui::ButtonGroup::new().children(vec!["One","Two","Three"]).outline().primary())
        }
        .gap_2()
    }

    fn render_icon_story(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Sizes")
            h_flex().items_center().gap_4().child(Icon::new(IconName::Sparkle).size(IconSize::XSmall)).child(Icon::new(IconName::Sparkle).size(IconSize::Small)).child(Icon::new(IconName::Sparkle).size(IconSize::Medium)).child(Icon::new(IconName::Sparkle).size(IconSize::Large)).child(Icon::new(IconName::Sparkle).size(IconSize::XLarge))

            Self::section_title("Colors")
            h_flex().items_center().gap_4().child(Icon::new(IconName::Star).size(IconSize::Large).color(cx.theme().colors.primary)).child(Icon::new(IconName::Star).size(IconSize::Large).color(cx.theme().colors.success)).child(Icon::new(IconName::Star).size(IconSize::Large).color(cx.theme().colors.warning)).child(Icon::new(IconName::Star).size(IconSize::Large).color(cx.theme().colors.danger))

            Self::section_title("Common Icons")
            h_flex().items_center().gap_3().flex_wrap().child(IconName::Library).child(IconName::Settings).child(IconName::MagnifyingGlass).child(IconName::Bell).child(IconName::Person).child(IconName::Check).child(IconName::Close).child(IconName::Plus).child(IconName::Trash).child(IconName::Copy).child(IconName::ExternalLink).child(IconName::Info)
        }
        .gap_2()
    }

    fn render_label_story(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Basic Labels")
            h_flex().items_center().gap_4().child(Label::new(IconName::Info, Text::new("Information"))).child(Label::new(IconName::Check, Text::new("Success"))).child(Label::new(IconName::Warning, Text::new("Warning"))).child(Label::new(IconName::Close, Text::new("Error")))

            Self::section_title("Icon Right")
            h_flex().items_center().gap_4().child(Label::new(IconName::ChevronRight, Text::new("Next")).icon_right()).child(Label::new(IconName::ExternalLink, Text::new("Open Link")).icon_right())

            Self::section_title("Colored")
            h_flex().items_center().gap_4().child(Label::new(IconName::Sparkle, Text::new("AI")).color(gpui::rgb(0x6366f1)))
        }
        .gap_2()
    }

    fn render_link_story(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Basic Links")
            h_flex().items_center().gap_4().child(Link::new("link-inline").child(Text::new("Inline Link"))).child(Link::new("link-external").child(IconName::ExternalLink).child(Text::new("External Link"))).child(Link::new("link-ui").child(Text::new("Documentation →")))
        }
        .gap_2()
    }

    fn render_divider_story(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Horizontal Divider")
            Text::new("Above")
            Divider::new()
            Text::new("Below")

            Self::section_title("With Text")
            Divider::new().text("OR")

            Self::section_title("Dashed")
            Divider::new().dashed()

            Self::section_title("Vertical (in row)")
            h_flex().h(px(40.)).items_center().gap_0().child(Text::new("Left")).child(Divider::vertical().mx_3()).child(Text::new("Right"))
        }
        .gap_2()
    }

    fn render_avatar_story(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Shapes")
            h_flex().items_center().gap_4().child(Avatar::new("icons/zed_assistant.svg").size(px(40.))).child(Avatar::new("icons/zed_assistant.svg").size(px(40.)).square())

            Self::section_title("Sizes")
            h_flex().items_center().gap_4().child(Avatar::new("icons/zed_assistant.svg").size(px(24.))).child(Avatar::new("icons/zed_assistant.svg").size(px(32.))).child(Avatar::new("icons/zed_assistant.svg").size(px(40.))).child(Avatar::new("icons/zed_assistant.svg").size(px(56.)))

            Self::section_title("With Indicator")
            h_flex().items_center().gap_4().child(Avatar::new("icons/zed_assistant.svg").size(px(40.)).indicator(Indicator::dot().color(cx.theme().colors.success))).child(Avatar::new("icons/zed_assistant.svg").size(px(40.)).indicator(Indicator::dot().color(cx.theme().colors.danger)))
        }
        .gap_2()
    }

    fn render_card_story(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Basic Card")
            Card::new()
                .child(Text::new("This is a basic card with some content."))
                .child(Text::new("Cards can contain multiple children."))

            Self::section_title("Card with Title")
            Card::new()
                .title("Card Title")
                .child(Text::new("Card content goes here."))
                .child(Button::new("card-btn").text("Action").primary())

            Self::section_title("Horizontal Card")
            Card::new()
                .direction_horizontal()
                .child(Text::new("Left"))
                .child(Text::new("Right"))
        }
        .gap_2()
    }

    fn render_checkbox_story(
        &mut self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Col! {
            Self::section_title("States")
            h_flex().items_center().gap_4().child(Checkbox::new("cb1").text("Unchecked")).child(Checkbox::new("cb2").checked(true).text("Checked")).child(Checkbox::new("cb3").checked(ToggleState::Indeterminate).text("Indeterminate")).child(Checkbox::new("cb4").checked(true).disabled(true).text("Disabled"))

            Self::section_title("Interactive")
            h_flex().items_center().gap_4().child(Checkbox::new("cb-interactive").checked(self.checkbox_checked).text("Toggle me").on_click(cx.listener(|this, _, _, _| { this.checkbox_checked = !this.checkbox_checked; })))
        }
        .gap_2()
    }

    fn render_radio_story(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Radio Group")
            RadioGroup::new()
                .selected_index(Some(self.radio_selected))
                .children(vec!["Option A", "Option B", "Option C"])
                .on_change(cx.listener(|this, index, _, _| { this.radio_selected = *index; }))

            Self::section_title("Disabled Radio Group")
            RadioGroup::new()
                .selected_index(Some(1))
                .disabled(true)
                .children(vec!["Disabled A", "Disabled B", "Disabled C"])
        }
        .gap_2()
    }

    fn render_switch_story(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("States")
            h_flex().items_center().gap_4().child(Switch::new("sw1").checked(self.switch_checked).text("Enabled").on_click(cx.listener(|this, _, _, _| { this.switch_checked = !this.switch_checked; }))).child(Switch::new("sw2").disabled(true).text("Disabled"))

            Self::section_title("Sizes")
            h_flex().items_center().gap_4().child(Switch::new("sw-xs").size(Size::XSmall).checked(true)).child(Switch::new("sw-sm").size(Size::Small).checked(true)).child(Switch::new("sw-md").checked(true)).child(Switch::new("sw-lg").size(Size::Large).checked(true))
        }
        .gap_2()
    }

    fn render_text_story(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Variants")
            Text::new("Default text with the current theme color.")
            Text::new("Muted text").color(cx.theme().colors.text_muted)
            Text::new("Colored text").color(cx.theme().colors.primary)
            Text::new("Underlined text").underline()
            Text::new("Strikethrough text").strikethrough()

            Self::section_title("Sizes")
            Text::new("Extra Small").text_xs()
            Text::new("Small").text_sm()
            Text::new("Base").text_base()
            Text::new("Large").text_lg()
            Text::new("Extra Large").text_xl()
        }
        .gap_2()
    }

    fn render_tooltip_story(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Tooltip on Hover")
            h_flex().gap_4().items_center().child(
                Button::new("tooltip-btn")
                    .text("Hover me")
                    .tooltip(rui::Tooltip::text("This is a tooltip!")),
            ).child(
                Button::new("tooltip-btn2")
                    .text("More info")
                    .outline()
                    .tooltip(rui::Tooltip::text("Tooltip with extra information")),
            )
        }
        .gap_2()
    }

    fn render_headline_story(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Sizes")
            Headline::new("XSmall Headline").size(HeadlineSize::XSmall)
            Headline::new("Small Headline").size(HeadlineSize::Small)
            Headline::new("Medium Headline (default)").size(HeadlineSize::Medium)
            Headline::new("Large Headline").size(HeadlineSize::Large)
            Headline::new("XLarge Headline").size(HeadlineSize::XLarge)

            Self::section_title("Colored")
            Headline::new("Primary Headline").color(cx.theme().colors.primary)
            Headline::new("Success Headline").color(cx.theme().colors.success)
        }
        .gap_2()
    }

    fn render_indicator_story(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Dot Indicators")
            h_flex().items_center().gap_4().child(h_flex().items_center().gap_2().child(Indicator::dot()).child(Text::new("Default"))).child(h_flex().items_center().gap_2().child(Indicator::dot().color(cx.theme().colors.success)).child(Text::new("Success"))).child(h_flex().items_center().gap_2().child(Indicator::dot().color(cx.theme().colors.warning)).child(Text::new("Warning"))).child(h_flex().items_center().gap_2().child(Indicator::dot().color(cx.theme().colors.danger)).child(Text::new("Danger")))

            Self::section_title("With Border")
            h_flex().items_center().gap_4().child(Indicator::dot().border_color(cx.theme().colors.bg)).child(Indicator::bar().color(cx.theme().colors.primary))
        }
        .gap_2()
    }

    fn render_layout_story(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Row! Macro")
            Card::new().child(
                Row! {
                    Text::new("Item 1")
                    Text::new("Item 2")
                    Text::new("Item 3")
                }
                .gap_4()
            )

            Self::section_title("Col! Macro")
            Card::new().child(
                Col! {
                    Text::new("Line 1")
                    Text::new("Line 2")
                    Text::new("Line 3")
                }
                .gap_2()
            )

            Self::section_title("Section! Macro")
            Section! {
                "With Title";
                Text::new("Section content goes here")
                Button::new("section-btn").text("Action").soft()
            }

            Self::section_title("h_flex / v_flex")
            h_flex().gap_4().child(h_flex().size(px(100.)).bg(cx.theme().colors.primary.opacity(0.2)).rounded_md().justify_center().items_center().child("h_flex")).child(v_flex().size(px(100.)).bg(cx.theme().colors.success.opacity(0.2)).rounded_md().justify_center().items_center().child("v_flex"))
        }
        .gap_2()
    }

    fn render_alert_story(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Self::section_title("Alert Modal")
            AlertModal::new("demo-alert", "Confirm Action")
                .child(Text::new("Are you sure you want to proceed with this action? This operation cannot be undone."))
                .primary_button(Button::new("confirm").text("Confirm").primary())
                .dismiss_button(Button::new("dismiss").text("Cancel").soft())
        }
        .gap_2()
    }
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg = cx.theme().colors.bg;
        let border = cx.theme().colors.border_variant;
        let text_color = cx.theme().colors.text;

        v_flex()
            .size_full()
            .bg(bg)
            .text_color(text_color)
            // ── Top Toolbar ──────────────────────────────────────────────────
            .child(
                h_flex()
                    .id("toolbar")
                    .h(px(44.))
                    .bg(cx.theme().colors.bg_elevated_surface)
                    .border_b_1()
                    .border_color(border)
                    .px_4()
                    .items_center()
                    .justify_between()
                    .flex_shrink_0()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::ZedAssistant)
                                    .size(IconSize::Medium)
                                    .color(cx.theme().colors.primary),
                            )
                            .child(
                                Text::new("Rui — Component Showcase")
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_size(rems_from_px(14.)),
                            ),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Button::new("theme-toggle")
                                    .text(if cx.theme().appearance.is_light() {
                                        "🌙 Dark"
                                    } else {
                                        "☀️ Light"
                                    })
                                    .ghost()
                                    .size(Size::Small)
                                    .on_click(cx.listener(|_this, _, window, cx| {
                                        cx.theme_mut().toggle_builtin_appearance(window);
                                    })),
                            ),
                    ),
            )
            // ── Body: Sidebar + Content ─────────────────────────────────────
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .child(self.render_sidebar(cx))
                    .child(self.render_content(cx)),
            )
            // ── Bottom StatusBar ────────────────────────────────────────────
            .child(
                h_flex()
                    .id("statusbar")
                    .h(px(28.))
                    .bg(cx.theme().colors.bg_elevated_surface)
                    .border_t_1()
                    .border_color(border)
                    .px_4()
                    .items_center()
                    .justify_between()
                    .flex_shrink_0()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Check)
                                    .size(IconSize::XSmall)
                                    .color(cx.theme().colors.success),
                            )
                            .child(
                                Text::new(format!(
                                    "{} components ready",
                                    ComponentPage::all().len()
                                ))
                                .text_size(rems_from_px(11.))
                                .color(cx.theme().colors.text_muted),
                            ),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Text::new(format!(
                                    "Active: {}",
                                    self.active_page.name()
                                ))
                                .text_size(rems_from_px(11.))
                                .color(cx.theme().colors.text_muted),
                            )
                            .child(
                                Text::new("v0.1.0")
                                    .text_size(rems_from_px(11.))
                                    .color(cx.theme().colors.text_muted),
                            ),
                    ),
            )
    }
}

// ── Entry Point ──────────────────────────────────────────────────────────────

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        cx.activate(true);
        Theme::init(cx, None, None);

        let bounds = Bounds::centered(None, size(px(1100.), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let gallery = cx.new(|_| Gallery::new());
                cx.new(|cx| Root::new(cx, gallery.into()))
            },
        )
        .unwrap();
    });
}
