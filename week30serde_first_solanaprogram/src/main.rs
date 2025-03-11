// use std::{clone, result};

// use serde::{Serialize,Deserialize}; //serde helps in serlisation and deserlisation of contents converting struct to string to that we can print that or converting string back again to struct

// #[derive(Serialize,Deserialize , Debug,Clone)] //this is macro to serliase and desrlise which is procedural macro
// struct user{
//     username : String,
//     password : String
// }



// fn main() {

//     //this is seralisation converting a struct to a string 
//     let u = user{username:String::from("Psrth") , password : String::from("parth")};
//     let to_string  = serde_json::to_string(&u);

//     let result = to_string.unwrap();
//     println!("{}" , result);

//     // match to_string {
//     //     Ok(str)=> print!("{}", str),
//     //     Err(_)=> print!("error detected")
    
//     //decerliastion converting a string back to struct

//     let u2 = String::from(" {\"username\" : \"parth\" , \"password\" : \"partht\"} "); //this is a valid json data

//     let decerliase_to_user: Result<user, serde_json::Error> = serde_json::from_str(&u2);
//     let result2 = decerliase_to_user.unwrap();
//     println!("{:?}" , result2);



// }















//borsh : this is important for core solana development for this to understand we studied serde
// converting struct to derterministc byte format is done by borsh
// as blockchain just stores the bytes it dosnt stores the utf-8 string and etc 
//borsh -  binary object representation seriliser for hashing
use borsh::{BorshSerialize,BorshDeserialize};

//we are learingn this beacuse we have struct which are eventually converted it into blockchain
#[derive(BorshSerialize,BorshDeserialize,Debug , Clone)]
struct my_struct {
    username : String,
    password : String
}



fn main(){

    // print!("hellow world");

    let u1 = my_struct {
        username : String::from("ParthBandwal"),
        password : String::from("Password..")
    };

    let mut v :Vec<u8> = Vec::new(); // a array of unsigned 8 int or we can also it a 8 bit/1byte
    let ans = u1.serialize(   &mut v).unwrap();
    println!(" {:?} ",v  );

    let user = my_struct::try_from_slice(&v).unwrap();
    println!("{}" , user.username);


    

}