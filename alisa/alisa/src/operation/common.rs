
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
macro_rules! object_set_property_operation {
    ($object: ty, $property: ident, $T: ty) => {
        ::alisa::paste::paste! {

            #[derive(::alisa::Serializable, Default)]
            #[project(<$object as alisa::Object>::Project)]
            pub struct [<Set $object:camel $property:camel >] {
                pub ptr: ::alisa::Ptr<$object>,
                pub [< $property:snake _value >]: $T
            }

            impl ::alisa::Operation for [< Set $object:camel $property:camel >] {

                type Project = <$object as ::alisa::Object>::Project;

                const NAME: &'static str = stringify!([< Set $object:camel $property:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    let Some(obj) = recorder.get_obj_mut(self.ptr) else {
                        return false;
                    };
                    let old_val = obj.$property.clone();
                    obj.$property = self.[< $property:snake _value >].clone();
                    true
                }

            }

            impl ::alisa::InvertibleOperation for [< Set $object:camel $property:camel >] {

                type Inverse = Self;

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
