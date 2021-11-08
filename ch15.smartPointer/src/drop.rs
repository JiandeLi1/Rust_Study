struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main(){
    let c = CustomSmartPointer {data:String::from("my stuff")};
    //c.drop(); //If use drop, c will drop first, otherwise d will drop first.
    let d =  CustomSmartPointer {data: String::from("other stuff")};
    println!("CustomSmartPointers created");
}