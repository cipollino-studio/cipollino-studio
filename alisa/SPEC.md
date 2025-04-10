
# State

This section of the spec describes how data is represented, stored, and loaded by Alisa.

It is important to note that state in Alisa can only be mutated by operations(as described later), which allows Alisa to maintain its guarantees of eventual consistency.

In Alisa, there are two kinds of state: the project and objects.

### Project

The *project* is a singleton that is always loaded immediately when a local client opens a project file or a collab client connects to a server. When collaborating, each client always has the project loaded. This makes it useful for storing data that a client needs to have in order to load anything else it might need; in Cipollino, for example, the project contains `Ptr`s to the folders and clips in the root asset folder, each of which are separate objects that might be lazy-loaded. When used like this, the project can be thought of as the "root" of all the other data.

In Rust, the project is some data type that implements the `Project` trait.

WARNING: Because of the way Alisa works internally, it is very important that the project type's `clone()` method does a full deep copy of the project state. This means that, without special provisions, putting something like an `Arc<Mutex<...>>` into a type implementing `Project` is not allowed, since cloning an `Arc<Mutex<...>>` doesn't make a copy of the type inside the mutex. Violating this rule could lead to bugs and a breach of eventual consistency.

### Objects

*Objects* are instances of a type that are given unique keys and that can be individually loaded by a client. Objects are referenced via a type called `Ptr<Object>`. Internally, `Ptr`s are just a wrapper around the object's key(which is just a `u64`), but wrapping then in a generic type like this helps prevent silly mistakes at compile time(e.g. assigning a reference to a `Folder` to a variable that should hold a reference to a `Clip`). The `Ptr` is short for pointer, but they have nothing to do with actual pointers in RAM. To get the actual object at a given `Ptr<Object>`, use `client.get(ptr)`. This method returns an `Option` since the object might be unloaded or deleted.

Usually, objects map onto entities in the app that the user can create/delete; for example, in Cipollino, layers, frames and strokes are all kinds of objects. However, objects are also a more generally useful tool for partitioning state for lazy-loading and efficient incremental serialization. In Cipollino, each clip is actually made out of two objects: `Clip` and `ClipInner`. The `Clip` holds metadata like the clip's name and a `Ptr` to its corresponding `ClipInner`, which contains the actual data(layers, frames, etc) of the clip. This is done so that the client can eagerly load all the `Clip` objects to render the asset hiearchy without loading the actual contents of the clips, which can be quite large.

From the perspective of each client, each potential object(`Ptr<Object>`) exists in one of the following states:

* `None`. We don't know what this object is and we haven't tried touching it before.
* `Loading`. We are currently waiting to load this object from the server. It might or might not exist - we don't know yet.
* `Loaded(Object)`. The object was loaded by this client.
* `Deleted`. The object was deleted or it failed to load because the pointer we have doesn't point to an object that exists.

This is the set of all valid transitions between these states:

* *Load Request*: `None` -> `Loading` or `Deleted -> Loading`
* *Loaded*: `Loading -> Loaded(Object)`
* *Load Failed*: `Loading -> Deleted`
* *Delete Object*: `Loaded(Object) -> Deleted`
* *Create Object*: `None -> Loaded(Object)` or `Deleted -> Loaded(object)`

In Rust, objects are data types that implement the `Object` trait. Note that for Alisa to work, you have to add an `ObjList` of the object type to the `Project::Objects` struct and add a `ObjectKind` to `Project::OBJECTS`.

WARNING: Because of the way Alisa works internally, it is very important that the object type's `clone()` method does a full deep copy of the project state. This means that, without special provisions, putting something like an `Arc<Mutex<...>>` into a type implementing `Object` is not allowed, since cloning an `Arc<Mutex<...>>` doesn't make a copy of the type inside the mutex. Violating this rule could lead to bugs and a breach of eventual consistency.

### Loading

By default, every object has to be individually requested by the client to load it from disk(on a local client) or from the server(on a collab client). However, it is often useful to have the project or an object automatically load another object when it is loaded. For example, in Cipollino, loading a `ClipInner` object automatically loads the layers it contains, which in turn automatically load their frames, etc. In Alisa, this behaviour can be accomplished using the `LoadingPtr<Object>` type. Like a `Ptr`, `LoadingPtr` is for referencing an object. Unlike a `Ptr`, however, a `LoadingPtr` automatically loads the object it is pointing to(if it exists) when the `LoadingPtr` is loaded. These `LoadingPtr`s form a directed graph of autoloading, where loading the project or a single object can load an arbitrary number of other objects.

### Serialization

Thanks to the Verter file format, the project and every object can be independently re-serialized and saved to disk when they are modified, making autosave very efficient.

# Operations

In Alisa, all modifications to the state happen through operations. In Alisa, operations are types that implement the `Operation` trait, which defines the `perform` method.

### Operations must be completely deterministic

In order to guarantee eventual consistency, all operations must be 100% deterministic under all circumstances, and must always make the same modifications to the state.

This, incidentally, means that an operation cannot behave differently depending on whether a particular object is loaded by a client or not. To enforce this, if an operation attempts to access an object in an indeterminate state(`None` or `Loading`), the operation will be treated as being unsuccessful and will be undone. 