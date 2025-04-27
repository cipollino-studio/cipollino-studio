
use alisa::Serializable;

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

    assert_eq!(my_struct.shallow_serialize(), alisa::ABFValue::Map(Box::new([
        ("num".into(), 321.into()),
        ("yes".into(), false.into())
    ])));
}

#[test]
fn deserialize_struct() {
    let data = alisa::ABFValue::Map(Box::new([
        ("num".into(), 321.into()),
        ("yes".into(), false.into())
    ]));

    let my_struct = MyStruct::data_deserialize(&data).unwrap(); 

    assert_eq!(my_struct, MyStruct { num: 321, yes: false });

    let my_default_struct: MyStruct = MyStruct::data_deserialize(&alisa::ABFValue::PositiveInt(0)).unwrap();
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
    assert_eq!(MyEnum::Unit.shallow_serialize(), alisa::ABFValue::NamedUnitEnum("Unit".into()));

    assert_eq!(MyEnum::Unnamed(123, true).shallow_serialize(), alisa::ABFValue::NamedEnum("Unnamed".into(), Box::new(alisa::ABFValue::Array(Box::new([
        123.into(),
        true.into()
    ])))));

    assert_eq!(MyEnum::Named { num: 123, yes: true }.shallow_serialize(), alisa::ABFValue::NamedEnum(
        "Named".into(),
        Box::new(alisa::ABFValue::Map(Box::new([
            ("num".into(), 123.into()),
            ("yes".into(), true.into())
        ])))
    ));
}

#[test]
fn deserialize_enum() {
    let unit_data = alisa::ABFValue::NamedUnitEnum("Unit".into()); 
    let unit = MyEnum::data_deserialize(&unit_data).unwrap();
    assert_eq!(unit, MyEnum::Unit);

    let unnamed_data = alisa::ABFValue::NamedEnum(
        "Unnamed".into(),
        Box::new(alisa::ABFValue::Array(Box::new([
            123.into(),
            true.into()
        ])))
    );
    let unnamed = MyEnum::data_deserialize(&unnamed_data).unwrap();
    assert_eq!(unnamed, MyEnum::Unnamed(123, true));

    let named_data = alisa::ABFValue::NamedEnum(
        "Named".into(),
        Box::new(alisa::ABFValue::Map(Box::new([
            ("num".into(), 123.into()),
            ("yes".into(), true.into())
        ])))
    );
    let named = MyEnum::data_deserialize(&named_data).unwrap();
    assert_eq!(named, MyEnum::Named { num: 123, yes: true });

    let nothing = MyEnum::data_deserialize(&alisa::ABFValue::PositiveInt(0));
    assert_eq!(None, nothing);
}
