
#[macro_export]
macro_rules! ptr_enum_serialization_impl {
    ($typename: ident [$($obj: ident),*] $loading: literal $owning: literal) => {

        impl ::alisa::Serializable for $typename {

            fn serialize(&self, context: &::alisa::SerializationContext) -> ::alisa::ABFValue {
                match self {
                    $(
                        Self::$obj(ptr) => {
                            if $loading {
                                context.request_serialize(<$obj as ::alisa::Object>::TYPE_ID, ptr.key());
                            }
                            ::alisa::ABFValue::ObjPtr(<$obj as ::alisa::Object>::TYPE_ID, ptr.key())
                        }
                    ),*
                }
            }

            fn deserialize(data: &::alisa::ABFValue, context: &mut ::alisa::DeserializationContext) -> Option<Self> {
                match data {
                    $(
                        ::alisa::ABFValue::ObjPtr(<$obj as ::alisa::Object>::TYPE_ID, key) => {
                            if $loading {
                                context.request_load(<$obj as ::alisa::Object>::TYPE_ID, *key);
                            }
                            Some(Self::$obj(::alisa::Ptr::from_key(*key)))
                        }
                    ),*
                    _ => None
                }
            }

            fn delete(&self, queue: &mut Vec<::alisa::AnyPtr>) {
                if $owning {
                    match self {
                        $(
                            Self::$obj(ptr) => {
                                queue.push(ptr.any());
                            }
                        ),*
                    }
                }
            }

        }

    };
}

#[macro_export]
macro_rules! ptr_enum_from_impls {
    ($typename: ident [$($obj: ident),*]) => {
        $(
            impl From<::alisa::Ptr<$obj>> for $typename {

                fn from(ptr: ::alisa::Ptr<$obj>) -> Self {
                    Self::$obj(ptr)
                }

            }
        )*
    };
}

#[macro_export]
macro_rules! ptr_enum_child_ptr_impl {
    ($typename: ident [$($obj: ident),*] childof $parent_ptr: ty, in $project: ty) => {
            ::alisa::paste::paste! {

            #[derive(::alisa::Serializable)]
            pub enum [< $typename:camel TreeData >] {
                $($obj(::alisa::Ptr<$obj>, <$obj as ::alisa::TreeObj>::TreeData)),*
            }

            impl ::alisa::ChildPtr for $typename {

                type ParentPtr = $parent_ptr;
                type TreeData = [< $typename:camel TreeData >];
                type Project = $project;

                fn collect_data(&self, objects: &<$project as ::alisa::Project>::Objects) -> Option<[<$typename TreeData>]> {
                    match self {
                        $(
                            Self::$obj(ptr) => {
                                let obj = <$obj as ::alisa::Object>::list(objects).get(*ptr)?;
                                Some([< $typename TreeData >]::$obj(*ptr, <$obj as ::alisa::TreeObj>::collect_data(&obj, objects)))
                            },
                        )*
                    }
                }

                fn destroy(&self, recorder: &mut ::alisa::Recorder<Self::Project>) {
                    match self {
                        $(
                            Self::$obj(ptr) => {
                                if let Some(obj) = recorder.delete_obj(*ptr) {
                                    <$obj as ::alisa::TreeObj>::destroy(&obj, recorder);
                                }
                            },
                        )*
                    }
                }

                fn instance(&self, data: &Self::TreeData, parent: Self::ParentPtr, recorder: &mut ::alisa::Recorder<Self::Project>) {
                    match data {
                        $(
                            [< $typename TreeData >]::$obj(ptr, tree_data) => {
                                <$obj as ::alisa::TreeObj>::instance(tree_data, *ptr, parent, recorder); 
                            },
                        )*
                    }
                }

            }

        }
    };
}

#[macro_export]
macro_rules! ptr_enum {
    ($typename: ident [$($obj: ident),*]) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $typename {
            $($obj(::alisa::Ptr<($obj)>)),*
        } 

        ::alisa::ptr_enum_serialization_impl!($typename [$($obj),*] false false);
        ::alisa::ptr_enum_from_impls!($typename [$($obj),*]);
    };
    ($typename: ident loading [$($obj: ident),*]) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $typename {
            $($obj(::alisa::Ptr<($obj)>)),*
        } 

        ::alisa::ptr_enum_serialization_impl!($typename [$($obj),*] true false);
        ::alisa::ptr_enum_from_impls!($typename [$($obj),*]);
    };
    ($typename: ident owning [$($obj: ident),*]) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $typename {
            $($obj(::alisa::Ptr<($obj)>)),*
        } 

        ::alisa::ptr_enum_serialization_impl!($typename [$($obj),*] true true);
        ::alisa::ptr_enum_from_impls!($typename [$($obj),*]);
    };
    ($typename: ident holding [$($obj: ident),*]) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $typename {
            $($obj(::alisa::Ptr<($obj)>)),*
        } 

        ::alisa::ptr_enum_serialization_impl!($typename [$($obj),*] false true);
        ::alisa::ptr_enum_from_impls!($typename [$($obj),*]);
    };
    ($typename: ident [$($obj: ident),*] childof $parent_ptr: ty, in $project: ty) => {
        ::alisa::ptr_enum!($typename [$($obj),*]);
        ::alisa::ptr_enum_child_ptr_impl!($typename [$($obj),*] childof $parent_ptr, in $project);        
    };
    ($typename: ident loading [$($obj: ident),*] childof $parent_ptr: ty, in $project: ty) => {
        ::alisa::ptr_enum!($typename loading [$($obj),*]);
        ::alisa::ptr_enum_child_ptr_impl!($typename [$($obj),*] childof $parent_ptr, in $project);        
    };
    ($typename: ident owning [$($obj: ident),*] childof $parent_ptr: ty, in $project: ty) => {
        ::alisa::ptr_enum!($typename owning [$($obj),*]);
        ::alisa::ptr_enum_child_ptr_impl!($typename [$($obj),*] childof $parent_ptr, in $project);        
    };
    ($typename: ident holding [$($obj: ident),*] childof $parent_ptr: ty, in $project: ty) => {
        ::alisa::ptr_enum!($typename holding [$($obj),*]);
        ::alisa::ptr_enum_child_ptr_impl!($typename [$($obj),*] childof $parent_ptr, in $project);        
    };
}
