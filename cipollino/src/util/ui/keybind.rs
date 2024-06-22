
pub fn consume_shortcut(ctx: &egui::Context, shortcut: &egui::KeyboardShortcut) -> bool {
    let res = ctx.input_mut(|i| i.consume_shortcut(shortcut)) && !ctx.memory(|mem| mem.focused().is_some());
    if res {
        ctx.request_repaint();
    }
    res
}
