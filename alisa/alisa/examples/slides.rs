
#[derive(alisa::Serializable)]
#[project(SlipsProject)]
pub struct SlipsProject {
    name: String,
    slides: alisa::ChildList<Slide>
}

impl Default for SlipsProject {

    fn default() -> Self {
        Self {
            name: "Untitled Slips".to_string(),
            slides: alisa::ChildList::default()
        }
    }

}

impl alisa::Project for SlipsProject {

    type Context = ();
    type Objects = SlipsObjects;

    fn empty() -> Self {
        Self::default()
    }

    fn create_default(&mut self) {

    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Slide>()
    ];

    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<SetName>(),
        alisa::OperationKind::from::<CreateSlide>()
    ];

}

alisa::project_set_property_operation!(SlipsProject, name, String);

#[derive(alisa::Serializable, Clone)]
#[project(SlipsProject)]
pub struct Slide {
    parent: (),
    title: String,
    text_boxes: alisa::UnorderedChildList<TextBox>
}

impl Default for Slide {

    fn default() -> Self {
        Self {
            parent: (),
            title: "Top Text".to_owned(),
            text_boxes: alisa::UnorderedChildList::new()
        }
    }

}

impl alisa::Object for Slide {
    type Project = SlipsProject;

    const NAME: &'static str = "Slide";

    fn list(objects: &SlipsObjects) -> &alisa::ObjList<Slide> {
        &objects.slides
    }

    fn list_mut(objects: &mut SlipsObjects) -> &mut alisa::ObjList<Slide> {
        &mut objects.slides
    }
}

alisa::object_set_property_operation!(Slide, title, String);

#[derive(alisa::Serializable)]
#[project(SlipsProject)]
pub struct SlideTreeData {
    title: String,
    text_boxes: alisa::UnorderedChildListTreeData<TextBox>
}

impl Default for SlideTreeData {

    fn default() -> Self {
        Self {
            title: "Slide".to_owned(),
            text_boxes: alisa::UnorderedChildListTreeData::default()
        }
    }

}

impl alisa::TreeObj for Slide {

    type ParentPtr = ();
    type ChildList = alisa::ChildList<Slide>;
    type TreeData = SlideTreeData;

    fn child_list<'a>(_parent: (), context: &'a alisa::ProjectContext<SlipsProject>) -> Option<&'a alisa::ChildList<Slide>> {
        Some(&context.project().slides)
    }

    fn child_list_mut<'a>(_parent: Self::ParentPtr, context: &'a mut alisa::ProjectContextMut<Self::Project>) -> Option<&'a mut Self::ChildList> {
        Some(&mut context.project_mut().slides)
    }

    fn parent(&self) -> () {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut () {
        &mut self.parent
    }

    fn instance(data: &SlideTreeData, ptr: alisa::Ptr<Slide>, parent: (), recorder: &mut alisa::Recorder<SlipsProject>) {
        use alisa::Object;
        let slide = Slide {
            parent,
            title: data.title.clone(),
            text_boxes: data.text_boxes.instance(ptr, recorder)
        };
        Self::add(recorder, ptr, slide);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<SlipsProject>) {
        self.text_boxes.destroy(recorder); 
    }

    fn collect_data(&self, objects: &<Self::Project as alisa::Project>::Objects) -> Self::TreeData {
        SlideTreeData {
            title: self.title.clone(),
            text_boxes: self.text_boxes.collect_data(objects)
        }
    }

}

alisa::tree_object_creation_operations!(Slide);

#[derive(alisa::Serializable, Clone)]
#[project(SlipsProject)]
struct TextBox {
    slide: alisa::Ptr<Slide>,
    x: f32,
    y: f32,
    content: String
}

impl Default for TextBox {

    fn default() -> Self {
        Self {
            slide: alisa::Ptr::null(),
            x: 0.0,
            y: 0.0,
            content: String::new() 
        }
    }

}

impl alisa::Object for TextBox {

    type Project = SlipsProject;

    const NAME: &'static str = "TextBox";

    fn list(objects: &SlipsObjects) -> &alisa::ObjList<TextBox> {
        &objects.text_boxes
    }

    fn list_mut(objects: &mut SlipsObjects) -> &mut alisa::ObjList<TextBox> {
        &mut objects.text_boxes
    }

}

#[derive(alisa::Serializable)]
#[project(SlipsProject)]
pub struct TextBoxTreeData {
    x: f32,
    y: f32,
    content: String
}

impl Default for TextBoxTreeData {

    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            content: String::new() 
        }
    }

}

impl alisa::TreeObj for TextBox {

    type ParentPtr = alisa::Ptr<Slide>;
    type ChildList = alisa::UnorderedChildList<TextBox>;
    type TreeData = TextBoxTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Slide>, context: &'a alisa::ProjectContext<SlipsProject>) -> Option<&'a alisa::UnorderedChildList<TextBox>> {
        context.obj_list().get(parent).map(|slide| &slide.text_boxes)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Slide>, context: &'a mut alisa::ProjectContextMut<SlipsProject>) -> Option<&'a mut alisa::UnorderedChildList<TextBox>> {
        context.obj_list_mut().get_mut(parent).map(|slide| &mut slide.text_boxes)
    }

    fn parent(&self) -> alisa::Ptr<Slide> {
        self.slide
    }

    fn parent_mut(&mut self) -> &mut alisa::Ptr<Slide> {
        &mut self.slide
    }

    fn instance(data: &TextBoxTreeData, ptr: alisa::Ptr<TextBox>, parent: alisa::Ptr<Slide>, recorder: &mut alisa::Recorder<SlipsProject>) {
        use alisa::Object;
        let text_box = TextBox {
            slide: parent,
            x: data.x,
            y: data.y,
            content: data.content.clone(),
        };
        Self::add(recorder, ptr, text_box);
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<SlipsProject>) {

    }

    fn collect_data(&self, _objects: &SlipsObjects) -> TextBoxTreeData {
        TextBoxTreeData {
            x: self.x,
            y: self.y,
            content: self.content.clone(),
        }
    }
}

pub struct SlipsObjects {
    slides: alisa::ObjList<Slide>,
    text_boxes: alisa::ObjList<TextBox>
}

impl Default for SlipsObjects {

    fn default() -> Self {
        Self {
            slides: alisa::ObjList::default(),
            text_boxes: alisa::ObjList::default()
        }
    }

}

fn main() {

    let mut client = alisa::Client::<SlipsProject>::local("my_cool_path.slips").unwrap();

    // let mut action = alisa::Action::new();
    // client.perform(&mut action, SetName {
    //     name: "My Cool Name".to_string(),
    // });

    // if let Some(ptr) = client.next_ptr() {
    //     client.perform(&mut action, CreateSlide {
    //         ptr,
    //         parent: (),
    //         idx: client.project().slides.n_children(),
    //         data: SlideTreeData {
    //             title: "New Slide".to_owned(),
    //             text_boxes: alisa::UnorderedChildListTreeData::default()
    //         },
    //     });
    // }

    client.tick(&mut ());

    for slide_ptr in client.project().slides.iter() {
        if let Some(slide) = client.get(slide_ptr) {
            println!("{}", slide.title);
        }
    }
    
    // let mut undo_redo = alisa::UndoRedoManager::new();

    // Add the action to the list of undo's 
    // undo_redo.add(action);

    // If there's an action to undo, undo it
    // undo_redo.undo(&client);

    client.tick(&mut ());

}
