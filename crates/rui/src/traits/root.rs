use crate::{ModalView, Root, ToastView};
use gpui::{App, Context, Entity, Window};

pub trait RootView {
    fn toggle_modal<V: ModalView, B>(&mut self, cx: &mut App, build: B)
    where
        B: FnOnce(&mut Window, &mut Context<V>) -> V;
    fn toggle_status_toast<V: ToastView>(&mut self, cx: &mut App, entity: Entity<V>);
}

impl RootView for Window {
    fn toggle_modal<V: ModalView, B>(&mut self, cx: &mut App, build: B)
    where
        B: FnOnce(&mut Window, &mut Context<V>) -> V,
    {
        if let Some(workspace) = self.root::<Root>().flatten() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(self, cx, build);
            })
        }
    }

    fn toggle_status_toast<V: ToastView>(&mut self, cx: &mut App, entity: Entity<V>) {
        if let Some(workspace) = self.root::<Root>().flatten() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_status_toast(entity, cx);
            })
        }
    }
}
