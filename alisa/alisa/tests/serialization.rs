
use alisa::Serializable;
use dummy_project::DummyProject;

mod dummy_project;

#[derive(alisa::Serializable, PartialEq, Eq, Debug)]
struct MyStruct {
    num: i32,
    yes: bool
}

impl Default for MyStruct {
    fn default() -> Self {
        Self { num: 123, yes: true }
    }
}

#[test]
fn serialize_struct() {
    let my_struct = MyStruct {
        num: 321,
        yes: false,
    };

    assert_eq!(Serializable::<DummyProject>::shallow_serialize(&my_struct), rmpv::Value::Map(vec![
        ("num".into(), 321.into()),
        ("yes".into(), false.into())
    ]));
}

#[test]
fn deserialize_struct() {
    let data = rmpv::Value::Map(vec![
        ("num".into(), 321.into()),
        ("yes".into(), false.into())
    ]);

    let my_struct: MyStruct = Serializable::<DummyProject>::data_deserialize(&data).unwrap(); 

    assert_eq!(my_struct, MyStruct { num: 321, yes: false });

    let my_default_struct: MyStruct = Serializable::<DummyProject>::data_deserialize(&rmpv::Value::Nil).unwrap();
    assert_eq!(my_default_struct, MyStruct::default());
}

#[derive(alisa::Serializable, PartialEq, Eq, Debug)]
enum MyEnum {
    Unit,
    Unnamed(i32, bool),
    Named {
        num: i32,
        yes: bool
    }
}

#[test]
fn serialize_enum() {
    assert_eq!(Serializable::<DummyProject>::shallow_serialize(&MyEnum::Unit), rmpv::Value::Array(vec![
        "Unit".into()
    ]));

    assert_eq!(Serializable::<DummyProject>::shallow_serialize(&MyEnum::Unnamed(123, true)), rmpv::Value::Array(vec![
        "Unnamed".into(),
        123.into(),
        true.into()
    ]));

    assert_eq!(Serializable::<DummyProject>::shallow_serialize(&MyEnum::Named { num: 123, yes: true }), rmpv::Value::Array(vec![
        "Named".into(),
        rmpv::Value::Map(vec![
            ("num".into(), 123.into()),
            ("yes".into(), true.into())
        ])
    ]));
}

#[test]
fn deserialize_enum() {
    let unit_data = rmpv::Value::Array(vec![
        "Unit".into()
    ]);
    let unit: MyEnum = Serializable::<DummyProject>::data_deserialize(&unit_data).unwrap();
    assert_eq!(unit, MyEnum::Unit);

    let unnamed_data = rmpv::Value::Array(vec![
        "Unnamed".into(),
        123.into(),
        true.into()
    ]);
    let unnamed: MyEnum = Serializable::<DummyProject>::data_deserialize(&unnamed_data).unwrap();
    assert_eq!(unnamed, MyEnum::Unnamed(123, true));

    let named_data = rmpv::Value::Array(vec![
        "Named".into(),
        rmpv::Value::Map(vec![
            ("num".into(), 123.into()),
            ("yes".into(), true.into())
        ])
    ]);
    let named: MyEnum = Serializable::<DummyProject>::data_deserialize(&named_data).unwrap();
    assert_eq!(named, MyEnum::Named { num: 123, yes: true });

    let nothing: Option<MyEnum> = Serializable::<DummyProject>::data_deserialize(&rmpv::Value::Nil);
    assert_eq!(None, nothing);
}
