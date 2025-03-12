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















// //borsh : this is important for core solana development for this to understand we studied serde
// // converting struct to derterministc byte format is done by borsh
// // as blockchain just stores the bytes it dosnt stores the utf-8 string and etc 
// //borsh -  binary object representation seriliser for hashing
// use borsh::{BorshSerialize,BorshDeserialize};

// //we are learingn this beacuse we have struct which are eventually converted it into blockchain
// #[derive(BorshSerialize,BorshDeserialize,Debug , Clone)]
// struct my_struct {
//     username : String,
//     password : String
// }



// fn main(){

//     // print!("hellow world");

//     let u1 = my_struct {
//         username : String::from("ParthBandwal"),
//         password : String::from("Password..")
//     };

//     let mut v :Vec<u8> = Vec::new(); // a array of unsigned 8 int or we can also it a 8 bit/1byte
//     let ans = u1.serialize(   &mut v).unwrap();
//     println!(" {:?} ",v  );

//     let user = my_struct::try_from_slice(&v).unwrap();
//     println!("{}" , user.username);




// }





















//lifetimes in rust
//its a consttuct to a compiler that ensures borrwos are valid.


// fn main(){

//     let str1 = String::from("Parthbxcfcvxcvx");
//     // let str2 = String::from("Parth1111");    
//     let ans;
//     {
//         let str2 = String::from("password");
//         ans = longest_string(&str1, &str2);
//         println!("{}", ans);
//     }
//     //ans will give dangling pointing error if s2>s1 

//     // print!("{}", ans); // here we get errror as answer of lifetime
// }

// fn longest_string<'a>(s1 :&'a String , s2 : &'a String) -> &'a String{ // this tell the lifetime of ans varrible we pass only one lifetime then by default it condisers lifetime of chote wale ka here str2 have chota lifetime then ans ka lifetime is also chota and only if there is more than one borrwo here 
//     if s1.len() > s2.len() {
//         return s1;
//     }else{
//         return s2;
//     }
// }   






// fn main(){

//     let s1 = String::from("Parth");
//     let s2 = String::from("Bandwal");
//     let ans;
//     {
//         let s3 = String::from("asdhfbmdnfd");
//         ans = longest_string(&s1, &s2, &s3);
//         println!("{}", ans);
//     }

//     //even if ans scope is longer but it's life time is in line 143-147 only not more than that definetnly

// }
// fn longest_string<'a,'b>(s1 : &'a String , s2 : &'a String , s3 : &'b String)->&'a String{
//     //this means 2 scopes but its lifetime of s1 and s2 will be lifetyime of answer

//     if s1.len() <s2.len() {
//         return s2;
//     }
//     return s1;
// }







//lifetime in structs // contruct that tells compiler about lifetime of a varrible


struct user<'a>{
    username :&'a str, //this is slice or pointer to a struct
    password:&'a str
} //now it tells user struct has lifetime chote wale ka



fn main(){
    let s1 = String::from("Parth");
    let s2 = String::from("djfhabsdmn");
    let u = user{
        username : &s1,
        password : &s2
    };

    println!(" {} {} " , u.username , u.password);

    return;
}


// solana contract is faster than eth beacuse solana can run contracts/blocks paralley where as eth does it one by one 
// this means eth is single threaded and solana is multi threaded 
// but there is issue if mutliple trasaction are trying to update a same varrible in a contract then there may be issues but solana sayys
// alaongw with transation we have to also specify kis kiss account se aap contact krne wale ho
// so we need to specify thier address to in program

// {
//     program
//     instruct_data
//     accouts[]
// }
 

// after this if there are 100 trasction running on diffrent accounts now they can run parrelelly without memory issues
// this makes sures if 2 transaction are running at same place they shouldn't run parallely otherwise this causes memory isseus


