

#[macro_export]
macro_rules! project_set_property_delta {
    ($project: ty, $property: ident, $T: ty) => {
        ::alisa::paste::paste! {
            pub struct [< Set $property:camel Delta >] {
                pub $property: $T
            }

            impl ::alisa::Delta for [< Set $property:camel Delta >] {

                type Project = $project; 

                fn perform(&self, context: &mut ::alisa::ProjectContextMut<Self::Project>) {
                    context.project_mut().$property = self.$property.clone();
                }

            }

        }
    };
}

#[macro_export]
macro_rules! project_set_property_operation {
    ($project: ty, $property: ident, $T: ty) => {
        alisa::project_set_property_delta!($project, $property, $T); 

        ::alisa::paste::paste! {

            #[derive(::alisa::Serializable, Default)]
            #[project($project)]
            pub struct [< Set $property:camel >] {
                pub $property: $T
            }

            impl ::alisa::Operation for [< Set $property:camel >] {

                type Project = $project;
                type Inverse = Self;

                const NAME: &'static str = stringify!([< SetProject $property:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    let old_val = recorder.project().$property.clone();
                    recorder.project_mut().$property = self.$property.clone();
                    recorder.push_delta([<Set $property:camel Delta>] {
                        $property: old_val 
                    });
                    true
                }

                fn inverse(&self, context: &::alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    Some(Self {
                        $property: context.project().$property.clone()
                    })
                }

            }

        }
    };
}

#[macro_export]
macro_rules! object_set_property_delta {
    ($object: ty, $property: ident, $T: ty) => {
        ::alisa::paste::paste! {

            pub struct [< Set $object:camel $property:camel Delta>] {
                pub ptr: alisa::Ptr<$object>,
                pub [< $property:snake _value >] : $T // We add _value to the end to make sure the name doesn't conflict with `ptr`
            }

            impl ::alisa::Delta for [< Set $object:camel $property:camel Delta >] {

                type Project = <$object as ::alisa::Object>::Project; 

                fn perform(&self, context: &mut ::alisa::ProjectContextMut<Self::Project>) {
                    if let Some(obj) = context.obj_list_mut().get_mut(self.ptr) {
                        obj.$property = self.[< $property:snake _value >].clone();
                    }
                }

            }

        }
    };
}

#[macro_export]
macro_rules! object_set_property_operation {
    ($object: ty, $property: ident, $T: ty) => {
        ::alisa::paste::paste! {
            ::alisa::object_set_property_delta!($object, $property, $T);    

            #[derive(::alisa::Serializable, Default)]
            #[project(<$object as alisa::Object>::Project)]
            pub struct [<Set $object:camel $property:camel >] {
                pub ptr: ::alisa::Ptr<$object>,
                pub [< $property:snake _value >]: $T
            }

            impl ::alisa::Operation for [< Set $object:camel $property:camel >] {

                type Project = <$object as ::alisa::Object>::Project;
                type Inverse = Self;

                const NAME: &'static str = stringify!([< Set $object:camel $property:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    let Some(obj) = recorder.obj_list_mut().get_mut(self.ptr) else {
                        return false;
                    };
                    let old_val = obj.$property.clone();
                    obj.$property = self.[< $property:snake _value >].clone();
                    recorder.push_delta([< Set $object:camel $property:camel Delta >] {
                        ptr: self.ptr,
                        [< $property:snake _value >]: old_val
                    });
                    true
                }

                fn inverse(&self, context: &::alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    use alisa::Object;
                    context.obj_list().get(self.ptr).map(|obj| Self::Inverse {
                        ptr: self.ptr,
                        [< $property:snake _value >]: obj.$property.clone()
                    })
                }

            }

        }
    };
}
