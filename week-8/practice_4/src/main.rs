fn main() {

    //Name vector 
    let name = vec!["Brian","Chigo","Joshua","Emeka","Ronald","Oshoke","Jeremy","Peter","Elnathan"];

    // Age vector
    let age = vec![16,15,19,22,20,21,18,23,18];

    print!("\nAge allocation: \n");

    //loop to literate elements in vector
    for i in 0..age.len()
    {
        // itercting through i on the vector
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
