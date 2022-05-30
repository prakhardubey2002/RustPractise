fn avail(bev:&str){
    if bev=="not drinkable"{
        panic!("toxic drink!!");
    }
    println!("The drink is {} ",bev);
}
fn main(){
    avail("drinkable");
    avail("not drinkable");
}