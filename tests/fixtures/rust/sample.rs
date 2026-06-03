pub struct MyStruct {
    field: i32,
}

impl MyStruct {
    pub fn new() -> Self {
        Self { field: 0 }
    }
}

pub fn top_level_func() {
    println!("Hello");
}
