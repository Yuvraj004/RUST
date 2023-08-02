
#[derive(Debug)]
struct Hindu{
    name: String,
    jaati : String,
    age: u32,
    state: String
}

impl Hindu{
    fn change_name(&mut self,name:String){
        self.name = name
    }
}

fn build_hindu(name:String,jaati:String) -> Hindu {
    Hindu{
        name,
        jaati,
        age:26,
        state: String::from("UP")
    }
}
fn main() {
    println!("Hello, world!");
    let mut aksh = Hindu{
        name: String::from("aksh"),
        jaati : String::from("kimouni"),
        age: 25,
        state: String::from("Uttrakhand")
    };
    println!("{} ki jaati {}",aksh.name,aksh.jaati);
    let hindu_2 = build_hindu(String::from("Kailash"),String::from("Tripathi"));
    println!("{:?}", hindu_2);
    println!("{:?}", aksh);
    aksh.change_name(String::from("baaap"));
    println!("{:?}", aksh);


}