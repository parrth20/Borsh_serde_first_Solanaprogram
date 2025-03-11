// traits -> this is a kind of interface in java /javascript

// trait Shape{ //few structs can implement this traits
//     fn area(&self)->f32;
// }

// struct rect {
//     widht: f32,
//     height : f32
// }

// impl Shape for rect {
//     fn area(&self)->f32{
//         return self.height*self.widht;
//     }
// }


// fn main() {

//     let r1 = rect{
//         widht : 10.2,
//         height : 14.2
//     };

//     let ans = r1.area();

//     println!("area of reactangle is : {}",ans );

//     println!("Hello, world!");
// }






//MACROS in rust - it is powerful langugae that allows metaprogramming (converting one syntax to another broder syntax having more function then converting it to machine understanable code) by enabling operation at compile time

// use std::{fmt, path::Display};

// fn main(){
//     println!("hello world"); //this is also a marco this also expand if we cargo expand this will show how println will expand or we can also see by option click

//     //whenever we add ! we are intialsing a macro
//     let v = vec![1,23,3]; // so this first gets expanded to code then it changes to binary
//     print!("{:?}", v);

    
// //this all declarative macros this is doing meta programing




// //we also have procedural macros



// let u1c = User{
//     username : String::from("Parth"),
//     password : String::from("Parthnsb"),
//     age : 21
// };

// //print!(" {} " , u1c); //display wihtout implementting display trait it throws an error
// print!(" {:?} ", u1c); //debug (here we should want to display debug trait to follow this)












// }



// #[derive(Debug)] // by doing this it automatically addes some display trait here so that it get prints in an ugly fashion cos debugging is not for styling as of 
// //this single line of stuct adds metaprogramming which adds default debug implmentation on this struct user
// //so this first type of procedural macro which we have learnd
// //this are also called custom derived macro

// struct User{
//     username: String,
//     password : String,
//     age :u32
// }


// //this one type of custom macro implementation
// // impl Display for User {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //       write!("this is display custom trait " {} " " , User.age)
// //     }
// // }



// //and in procedural marco we also have attriibute like macro which has a get and post end crud end endpoint whcih is used as verry similar to annotation in spring boot






// copy and clone trait in rust another way to give ownership to varrible 
// clone is expensive operation try to ignore it as much as you can 

use std::iter::StepBy;


#[derive(Debug , Clone, Copy)] // if we use copy macro we have to use clone macro also
struct  User {
    is_male : bool,
    age :u32,
    // username : String
}



fn main(){
    let u = User{
        is_male: true,
        age: 32,
        // username :String::from("parth")
    };
    let u1 = u; //  a new strcut is created copy of it we cannot do this when we have heap varribles in struct 
    //we have to use copy and clone macro sath me so with string it gives error
    println!( "{:?} {:?} ", u ,u1 );

}    
