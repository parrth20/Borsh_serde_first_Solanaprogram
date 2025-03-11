// fn main() {
//     println!("Hello World")
// }
// cargo init ---  this create a cargo.toml file which is same as pakage.json in npm init -y this help to initalise a rust project

// fn main(){
//     println!("hello world"); // macro not a function 
// }







// datatypes in Rust

// fn main(){
//     let ans: u32 = sum(1,3);
//     println!("{}" , ans);
// }
// fn sum(a : u32,b :u32) -> u32{ // this is same as typescript little bit uses more kinf of syntax like excpt number it's taking unsgined32 bits

//     return a+b;
// }

//u32 -> 32 bits unsigned integer
//diffrence betweeen signed and unsigned number in rust u32 ,i32
// i means 32 bits are stored for numbers and last bit is stored for sign if it's +ve then last bit is considerd as 0  and if its -ve last bit is changed to 1
//if we know what is sign then last bit is also used for number which increase our number line 







//boolean in rust
// in rust we _ as intendation not camel case

// fn main(){
//     // println!("{}" , is_even(10))
//     let ans = is_even(10);
//     println!("{}" , ans);
// }

// fn is_even(a : u32 )->bool {
//     return a%2 == 0
// }







// Strings
// rust donot have a garbage collector
// memory management in rust
// strings require memory mangement as strings have varrible sizes we can push and pop something in a array similar case in vector


// fn main(){
//     //let name = String:: from ("parth");

//     let vec = vec![1,2,3];
//     println!("Fisrt vector -> {:?}", vec); // vector prmitive print krna hai toh :? ayega {} iske andar 
// }




// for loops

// fn main(){
//     for i in 0..100{
//         print!(" {} " , i);
//     }
// }

// every thing in rust is immutable , cos this can crate memory issue same is this for string it is immuatble
// fn main(){
//     let mut name = String :: from("Parth"); //adding mut helps and makes it mutable
//     name.push_str(" Bandwal");

//     println!("{}" , name); 

// }


//OwnerShip and Borrows 
// Rust will ensure you cannot write bad code which cannot make memory manegement 

//stack vs heap
// if we are not using anything whose length of datastrutre can change on runtime then data stored in rust is in stack no heap comes in picture
// stack frame is a limited size 

//heap is contiguous memory struture stack store the pointer to heap as in above string one main is stored in stack which points to a heap of name that pointer increase stack is just same

//Ownership of heap varribles means ye heap ban toh gayi lekin removekab hoogi *memory managemnt
// rules are
// each value in rust has an single owner
//there can only be one owner at a time
// whn owner goes out of scope their values get dropped 

// fn main(){

//     create_str();
// }

// fn create_str(){
//     let mut name  = String::from("Parth");

//     let name2 = name; //now here comes ownweship rules there can only be one owner at a time this help is removing Dangling poniting error now if name is used and if it is removed from stack then name2 is pointing to a empty or null thing which is dangling pointing error which rust prevents 

//     //here we can only use name2 as this has only one owner in heap
//     //if we use name it gives error
//     let name3 = name2.clone(); // now this create a new saprate heap it's a ownership hack in rust we do this by bypassing it like clone

//     println!("{}" , name2);
// }





// fn main(){
//     let name = String::from("Parth");

//     let size = get_len(name);
//     println!("{}", size);

//     println!("{}", name); //this throws an error as s is used is removed from memory
// }

// fn get_len(s:String) ->usize{ //usize commonly used for sizes in string //ownership of name is given to s now we cannot use name again 
//     return s.len(); 
// }

//solving above problem in a ugly way
fn main(){
    let name = String::from("Parth");

    let ( size,name ) = get_len(name);
    println!("{}", size);
    println!("{}", name); //now this dont throws an error //ownershiped is also passed
}
fn get_len(s:String ) ->(usize , String){  
    return (s.len() , s); 
}
