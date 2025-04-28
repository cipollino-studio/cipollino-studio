
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

    assert_eq!(alisa::serialize(&my_struct), alisa::ABFValue::Map(Box::new([
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

    let my_struct = alisa::deserialize::<MyStruct>(&data).unwrap(); 

    assert_eq!(my_struct, MyStruct { num: 321, yes: false });

    let my_default_struct: MyStruct = alisa::deserialize::<MyStruct>(&alisa::ABFValue::PositiveInt(0)).unwrap();
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
    assert_eq!(alisa::serialize(&MyEnum::Unit), alisa::ABFValue::NamedUnitEnum("Unit".into()));

    assert_eq!(alisa::serialize(&MyEnum::Unnamed(123, true)), alisa::ABFValue::NamedEnum("Unnamed".into(), Box::new(alisa::ABFValue::Array(Box::new([
        123.into(),
        true.into()
    ])))));

    assert_eq!(alisa::serialize(&MyEnum::Named { num: 123, yes: true }), alisa::ABFValue::NamedEnum(
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
    let unit = alisa::deserialize::<MyEnum>(&unit_data).unwrap();
    assert_eq!(unit, MyEnum::Unit);

    let unnamed_data = alisa::ABFValue::NamedEnum(
        "Unnamed".into(),
        Box::new(alisa::ABFValue::Array(Box::new([
            123.into(),
            true.into()
        ])))
    );
    let unnamed = alisa::deserialize::<MyEnum>(&unnamed_data).unwrap();
    assert_eq!(unnamed, MyEnum::Unnamed(123, true));

    let named_data = alisa::ABFValue::NamedEnum(
        "Named".into(),
        Box::new(alisa::ABFValue::Map(Box::new([
            ("num".into(), 123.into()),
            ("yes".into(), true.into())
        ])))
    );
    let named = alisa::deserialize::<MyEnum>(&named_data).unwrap();
    assert_eq!(named, MyEnum::Named { num: 123, yes: true });

    let nothing = alisa::deserialize::<MyEnum>(&alisa::ABFValue::PositiveInt(0));
    assert_eq!(None, nothing);
}
