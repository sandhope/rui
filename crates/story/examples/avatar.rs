use gpui::{
    px, size, Application, Bounds, Context, KeyBinding, Menu, MenuItem, WindowBounds, WindowOptions,
};

use rui::{prelude::*, Assets, Avatar};
use std::path::PathBuf;
use std::sync::Arc;

struct AvatarStory {
    local_resource: Arc<std::path::Path>,
    asset_resource: SharedString,
}

impl Render for AvatarStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Avatar::new(self.local_resource.clone()).size(px(48.))
            Avatar::new(self.asset_resource.clone()).size(px(48.)).grayscale(true)
            Avatar::new(self.local_resource.clone()).size(px(48.)).border_color(gpui::blue())
        }
    }
}

gpui::actions!(image, [Quit]);

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    println!("{:?}", manifest_dir);
    Application::new()
        .with_assets(Assets)
        .run(move |cx: &mut App| {
            cx.activate(true);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
            cx.set_menus(vec![Menu {
                name: "Image".into(),
                items: vec![MenuItem::action("Quit", Quit)],
            }]);

            Theme::init(cx, None, None);
            let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
            cx.open_window(
                WindowOptions {
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some(SharedString::from("Image Example")),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| AvatarStory {
                        local_resource: manifest_dir
                            .join("../../assets/images/app-icon.png")
                            .into(),
                        asset_resource: "images/app-icon.png".into(),
                    })
                },
            )
            .unwrap();
        });
}
