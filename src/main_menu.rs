use ggez::{
  graphics::{Font,Text},
  nalgebra::Point2,
  Context
}

enum Message {
  Exit,
  NewGame,
}

fn make_gui(context: &mut Context, font: Font) -> ZResult<ui::Gui<Message>> {
  let mut gui = ui::Gui::new(context);
}