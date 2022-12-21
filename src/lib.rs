use smart_default::SmartDefault;

#[derive(SmartDefault)]
struct MyStruct {
    #[default = -1]
    x: i16
}
