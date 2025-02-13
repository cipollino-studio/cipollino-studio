
# Alisa

**Alisa** is a framework for building apps with real-time collaboration.

* [Features](#features) 
* [Key Concepts](#key-concepts)
    * [Project](#1-project)
    * [Operations](#2-operations)
    * [Client](#3-the-client)
    * [Actions](#4-actions-and-undoredo)
    * [Objects](#5-objects)
    * [Tree Objects](#6-tree-objects)
* [Namesake](#namesake)

### Features

Alisa handles many essential backend-y components of many apps, including:

* A real-time collaboration system with robust conflict resolution
* Serialization, with lazy-loading and incremental file updates
* Undo/Redo

Alisa does not handle:

* UI
* How messages are sent between the client and server

### Key Concepts

Alisa is very general and powerful framework. To use it effectively, there are several key concepts that you need to understand. To help clarify these concepts, let's walk through how we'd use Alisa to set up a Google Slides clone called Poodle Slips.

The final code for this example is available for reference in `alisa/examples/slides.rs`.

#### 1. Project

To use Alisa, the first thing you need is a project type. It should contain the "top-level data" for your app, and it is the first thing loaded from disk/the server by Alisa.

Here's how we might define the project type for Poodle Slips. We'll add more to it later as we go.

```rust

#[derive(alisa::Serializable)]
#[project(SlipsProject)]
pub struct SlipsProject {
    name: String
}

// Default is needed to use #[derive(alisa::Serializable)]
impl Default for SlipsProject {

    fn default() -> Self {
        Self {
            name: "Untitled Slips".to_string()
        }
    }

}

impl alisa::Project for SlipsProject {

    // Used for passing external data to operations - we won't be using it 
    type Context = ();

    // A struct containing an ObjList<> for every kind of object in our project
    // More on this later
    type Objects = SlipsObjects;

    // Create an empty project
    fn empty() -> Self {
        Self::default()
    }

    // Initialize a project to some default state 
    // Called when a project is first created
    fn create_default(&mut self) {

    }

    // The list of object types we'll have in this project - more on this later
    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[];

    // The list of operations we can perform on this project - more on this later
    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[];

}

```

#### 2. Operations

On their own, the project and objects(more on them later) are just inert data. To modify that data, we need to define operations. An operation is some type that implements the `Operation` trait, which defines some methods used in the real-time collaboration and undo/redo systems.

Defining operations by hand is quite tedious and error-prone, so Alisa provides a few macros to implement common operations. Let's make an operation to set the project's `name` using the `project_set_property_operation!` macro.

```rust
alisa::project_set_property_operation!(SlipsProject, name, String);
```

This macro takes the project type(`SlipsProject`), name of the property(`name`) and type of the property(`String`) and creates an operation that sets the `name` of the project. The operation is a struct called `SetName`, and the macro automatically generates the `Operation` trait implementation. Before we can use it, however, we need to register it in our `Project` implementation like so:

```rust
impl alisa::Project for SlipsProject {
    ...

    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<SetName>()
    ];

    ...
}
```

If an operation is not registered, it will not work properly with real-time collaboration. When compiling with debug assertions enabled, failing to register an operation will cause Alisa to panic.

#### 3. The Client

Now that we have our operation, let's see how we can apply it to our project. To load or modify a project, you need to initialize a `Client`. There are two types of clients: local clients, which stores/loads data to a file, and collab clients, which load data from a server and support real-time collaboration. Other than how they're initialized and how you handle sending messages to the server, local and collab clients work in the exactly the same way, so you can easily create both a traditional, desktop, non-collaborative and a browser-based collaborative version of your app. 

For now, let's make a local client. 

```rust
fn main() {

    let client = alisa::Client::<SlipsProject>::local("my_cool_path.slips").unwrap();

}
```

We pass in a path to the file we want to save our project. If the file doesn't exist, it will be created and initialized automatically.

Now, to set the name of our project, we can perform a `SetName` operation like so:

```rust
fn main() {
    
    ...

    // The action is used for the undo/redo system
    // More on this later
    let mut action = alisa::Action::new();
    client.perform(&mut action, SetName {
        name: "My Cool Name".to_string(),
    });

}
```

It is important to note that, for several reasons, an operation is not immediately performed when you call `client.perform`. Instead, it is put in a queue that only gets performed when you call `client.tick`, like so:

```rust
fn main() {

    ...

    // The () here is the "context". It must be of type Project::Context
    // In this case, we don't use the context for anything, so we just pass in () 
    client.tick(&mut ());

}
```

`client.tick` also does other things that the client needs to do periodically for Alisa to work, like saving changes to disk or queueing certain messages to be sent to the server. It is a good idea to call `client.tick` at the end of each UI paint. 

#### 4. Actions and Undo/Redo

An action is a group of operations that can be undone/redone as a group. When you pass in `&mut action` to `client.perform`, the operation performed is added to the `action`. To use actions for undo/redo, you need to use an `UndoRedoManager` like so:

```rust
fn main() {

    ...

    let mut undo_redo = alisa::UndoRedoManager::new();

    // Add the action to the list of undo's 
    undo_redo.add(action);

    // If there's an action to undo, undo it
    undo_redo.undo(&client);

}
```

Alisa's `UndoRedoManager` implements the standard linear timeline undo/redo system used in 99.99% of apps. As long as all the operations you use are implemented correctly, this system will give you robust undo/redo everywhere in your app with little to no work on your part. 

#### 5. Objects

In Alisa, an object is an instance of a type implementing the `Object` trait, with each instance of an object having a unique ID in the form a `Ptr<ObjectType>`. Object types must be registered in your project type's `Project::OBJECTS` list.

Objects usually correspond to concrete entities created by the user of your app. For example, Poodle Slips would represent each slide as its own object. The slide object could be defined like so: 

```rust
#[derive(alisa::Serializable, Clone)]
#[project(SlipsProject)]
pub struct Slide {
    title: String,
}

impl Default for Slide {

    fn default() -> Self {
        Self {
            title: "Top Text".to_owned(),
        }
    }

}

impl alisa::Object for Slide {

    // The project this type of object belongs to
    type Project = SlipsProject;

    // The name for this type of object
    // Make sure it really is unique or everything will break!
    // When compiling with debug assertions, a non-unique name will trigger a panic 
    const NAME: &'static str = "Slide";

    // Methods to get the ObjList of this kind of object
    // More on this later
    fn list(objects: &SlipsObjects) -> &alisa::ObjList<Slide> {
        &objects.slides
    }

    fn list_mut(objects: &mut SlipsObjects) -> &mut alisa::ObjList<Slide> {
        &mut objects.slides
    }
}
```

In addition to defining the `Slide` struct and the corresponding `Object` trait implementation, we also need to register `Slide` as a kind of object in our project like so:

```rust
impl alisa::Project for SlipsProject {

    ...

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Slide>()
    ];

    ...

}
```

Finally, we need to add an `ObjList` to our `SlipsObjects` struct:

```rust
pub struct SlipsObjects {
    slides: alisa::ObjList<Slide>
}

impl Default for SlipsObjects {

    fn default() -> Self {
        Self {
            slides: alisa::ObjList::default()
        }
    }

}
```

This struct(`SlipsProject::Objects`) is just a struct containing an `ObjList` for every kind of object that we have in our project. We referenced it earlier when defining `Object::list` and `Object::list_mut` for `Slide`. An `ObjList` is just a map between object pointers(`Ptr<>`) and objects, with some other internal features used by the library. It's a bit unfortunate that you have to manually add an `ObjList` for every object, but until Rust has compile-time reflection this is the best we can do.

Now, we can use some macros to define some operations to modify our `Slide`s:

```rust
alisa::object_set_property_operation!(Slide, title, String);
```

`object_set_property_operation!` works in a similar way to `project_set_property_operation!`, but we also need to specify the object whose property we want to set. This macro invocation generate an operation called `SetSlideTitle`. 

#### 6. Tree Objects

Now that we defined our `Slide` object, how do we create instances of it? While we could manually define custom operations for creating/deleting a slide, here we'll use Alisa's tree object system.

In a lot of apps, objects form a strict tree-shaped hierarchy of ownership. For instance, in our Google Slides clone, the project can contain many slides, each of which can contain many text boxes. To support this pattern, Alisa has a trait called `TreeObj`. If an object implements `TreeObj`, it means it is a child of something else as part of a tree hierarchy. Let's see how we can use it to make `Slide` a child of the project. First, let's implement the `TreeObj` trait for `Slide`:

```rust
#[derive(alisa::Serializable)]
#[project(SlipsProject)]
pub struct SlideTreeData {
    title: String
}

impl Default for SlideTreeData {

    fn default() -> Self {
        Self {
            title: "Slide".to_owned()
        }
    }

}

impl alisa::TreeObj for Slide {
    // A type that can be used to reference the parent of this object.
    // Slides are always the children of the project itself, so we don't need this for now 
    type ParentPtr = ();
    // The type of child list this object is stored in. 
    // In our case, we'll use the built-in alisa::ChildList
    type ChildList = alisa::ChildList<Slide>;
    // The type of data that can be used to "reconstruct" this object and all its children
    type TreeData = SlideTreeData;

    // Methods to get the ChildList containing this object given a ParentPtr
    // In our case, the child list is always the project's list of slides
    fn child_list<'a>(parent: (), project: &'a SlipsProject, objects: &'a SlipsObjects) -> Option<&'a alisa::ChildList<Slide>> {
        Some(&project.slides)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, context: &'a mut alisa::ProjectContext<Self::Project>) -> Option<&'a mut Self::ChildList> {
        Some(&mut context.project_mut().slides)
    }

    fn parent(&self) -> () {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut () {
        &mut self.parent
    }

    // A method for instancing this object and any potential children from Self::TreeData
    fn instance(data: &SlideTreeData, ptr: alisa::Ptr<Slide>, parent: (), recorder: &mut alisa::Recorder<SlipsProject>) {
        use alisa::Object;
        Self::add(recorder, ptr, Slide {
            parent,
            title: data.title.clone()
        });
    }

    // A method for deleting any children objects we have
    // We're not going to use this for now
    fn destroy(&self, recorder: &mut alisa::Recorder<SlipsProject>) {
        
    }

    // A method for collecting Self::TreeData from an existing object
    fn collect_data(&self, objects: &<Self::Project as alisa::Project>::Objects) -> Self::TreeData {
        SlideTreeData {
            title: self.title.clone(),
        }
    }

}
```

We also need to add a `ChildList` to our project to contain our slides. A `ChildList` is an ordered list of children objects.

```rust
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
```

Now, we can use another macro to define operations for creating/deleting slides!

```rust
alisa::tree_object_creation_operations!(Slide);
```

This macro defines operations called `CreateSlide` and `DeleteSlide` for, well, creating and deleting slides. Here's how we can use it:

```rust
fn main() {

    ...

    // To create a new object, we need to allocate a unique Ptr<> for it.
    // We can do so with client.next_ptr(). 
    // Note that under certain conditions, a collab client might occasionally be unable to immediately allocate a Ptr<>,
    // so next_ptr() returns an Option<Ptr<>>. 
    if let Some(ptr) = client.next_ptr() {
        client.perform(&mut action, CreateSlide {
            // The Ptr<> of the newly created slide
            ptr, 
            // The parent (must be of type Slide::ParentPtr)
            parent: (),
            // The index of the new object in the project's ChildList<Slide> 
            // We'll always put our slide at the end
            idx: client.project().slides.n_children(),
            // The data of the slide
            data: SlideTreeData {
                title: "New Slide".to_owned(),
            },
        });
    }

    ...

}
```

Now, we can iterate over all slides in the project like so:

```rust
fn main() {

    ...

    // We have to do client.tick first to make sure our operation is executed
    client.tick(&mut ());

    // Iterate over all Ptr<Slide> in the project's ChildList<Slide>
    for slide_ptr in client.project().slides.iter() {

        // Print out each slide's title
        if let Some(slide) = client.get(slide_ptr) {
            println!("{}", slide.title);
        }

    }

    ...
    
}
```

To demonstrate how to fully use the `TreeObj` system, let's also implement `TextBox`s. In the tree, `TextBox`s would be owned by `Slide`s. First, the struct definition and the `Object` implementation:

```rust
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
```

Nothing new here. Now, the `TreeObj` implementation:

```rust
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
    // Let's say we don't care about ordering TextBoxes in a slide, so we can use alisa::UnorderedChildList
    type ChildList = alisa::UnorderedChildList<TextBox>;
    type TreeData = TextBoxTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Slide>, project: &'a SlipsProject, objects: &'a SlipsObjects) -> Option<&'a alisa::UnorderedChildList<TextBox>> {
        objects.slides.get(parent).map(|slide| &slide.text_boxes)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Slide>, context: &'a mut alisa::ProjectContext<SlipsProject>) -> Option<&'a mut alisa::UnorderedChildList<TextBox>> {
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

    fn destroy(&self, recorder: &mut alisa::Recorder<SlipsProject>) {

    }

    fn collect_data(&self, objects: &SlipsObjects) -> TextBoxTreeData {
        TextBoxTreeData {
            x: self.x,
            y: self.y,
            content: self.content.clone(),
        }
    }
}
```

Again, this is basically the same thing as we had for `Slide`'s `TreeObj` implementation. But now, let's see how we had to change `Slide` to accomodate having child `TextBox`s:

```rust
#[derive(alisa::Serializable, Clone)]
#[project(SlipsProject)]
pub struct Slide {
    parent: (),
    title: String,
    // The text boxes inside this slide
    text_boxes: alisa::UnorderedChildList<TextBox>
}

...

#[derive(alisa::Serializable)]
#[project(SlipsProject)]
pub struct SlideTreeData {
    title: String,
    // We need to store the data of the Slide's children to recreate it 
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

    fn child_list<'a>(parent: (), project: &'a SlipsProject, objects: &'a SlipsObjects) -> Option<&'a alisa::ChildList<Slide>> {
        Some(&project.slides)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, context: &'a mut alisa::ProjectContext<Self::Project>) -> Option<&'a mut Self::ChildList> {
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
            // Instance the child TextBoxes from their data
            text_boxes: data.text_boxes.instance(ptr, recorder)
        };
        Self::add(recorder, ptr, slide);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<SlipsProject>) {
        // When deleting a slide, we need to delete its children too
        self.text_boxes.destroy(recorder); 
    }

    fn collect_data(&self, objects: &<Self::Project as alisa::Project>::Objects) -> Self::TreeData {
        SlideTreeData {
            title: self.title.clone(),
            // Collect the data of the children here 
            text_boxes: self.text_boxes.collect_data(objects)
        }
    }

}
```

And just like that, we have a three-tier object hierarchy: a `SlipsProject` can contain many `Slides`, each of which can contain many `TextBox`s.

The object tree system is built to be as flexible as possible. You can implement your own child containers to replace `ChildList` and `UnorderedChildList`, you can use a type other than `Ptr<>` for the `ParentPtr`(for example, if an object can be the child of two different kinds of objects), and much, much more.

### Namesake

This framework is named after Alisa Seleznyova(Алиса Селезнёва), a major character from the 1985 soviet sci-fi epic [Guests From The Future](https://en.wikipedia.org/wiki/Guest_from_the_Future). In the series, Alisa is a girl living in the late 21st century. She works at the Moscow Cosmo-Zoo, studying alien creatures from across the universe using the Mellophone, a device capable of reading the thoughts of any living being. Alisa Seleznyova appears in many Soviet novels and films, and continues to be one of the most recognizable characters in Russian science fiction to this day.
