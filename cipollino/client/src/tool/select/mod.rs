use super::Tool;


#[derive(Default)]
pub struct SelectTool {

}

impl Tool for SelectTool {

    const ICON: &'static str = pierro::icons::CURSOR;

}
