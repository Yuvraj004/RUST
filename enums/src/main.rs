#[derive(Debug)]

// An enum type is a special data type that enables for a variable to be a set of predefined constants. The variable must be equal to one of the values that have been predefined for it. Common examples include compass directions (values of NORTH, SOUTH, EAST, and WEST) and the days of the week.
enum IpAddKind {
    V4(String),
    V6(u8,u8,u8,u8,u8,u8),
}
//to store a variable with null/none value we use enum
enum Option<T> {
    Some(T),
    None,
}
#[derive(Debug)]
struct IpAdd {
    kind:IpAddKind,
    // address : String
}


fn options_syntax(){
    // //using option syntax
    // let some_num: std::option::Option<&str> =Some("DAta");// no need to specify DT rust configures it on his own
    // println!("{:?}",some_num);
    // let absent_num: std::option::Option<i32> = None; //we have to specify the data type on our own
    // println!("{:?}",absent_num);
}

fn options_code(){
    let x: i32 =5;
    // let y: std::option::Option<i32> = Some(5);
    let y = None;
    // let sum = x+y; 
    //showing error becoz different data types 
    //in order to add data from y we need to extract number from y
    let sum = x + y.unwrap_or(0);
    println!("{}",sum);
}

fn enum_code(){
    // let _four: fn(String) -> IpAddKind  = IpAddKind::V4;
    // let _six: fn(u8,u8,u8,u8,u8,u8) -> IpAddKind = IpAddKind::V6;
    // let _localhost = IpAdd{
    //     kind: IpAddKind::V4(String::from("127.0.0.1")), 
    // };
    // let vp6 = IpAdd {
    //     kind : IpAddKind::V6(192, 129, 11, 00, 78, 56)
    // };
    // println!("{:#?}",vp6);
    // println!("Hello, world!");
}


fn main() {
    options_syntax();
    options_code();
    enum_code();
    //underscore syntax in match expressions
    //if let syntax as an alternative to match expressions
}
