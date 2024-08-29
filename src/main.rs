fn main() {
    let a=10;
    let b=4;
    let modulo=3;
    let min=-100;
    let max=200;
    let res=find_congruent_numbers(a,modulo,min,max);
    let count=9;
    println!("{a} is congruent to {b} mod {modulo}={:?}",is_congruent(a, b,modulo));
    println!("The number between {} and {} that are congruent to {}mod{} are {:?}",min,max,a,modulo,res);
   
   
    println!("The first {} positive numbers congruent to {}mod{} are {:?}",count,a,modulo,generate_positive_congruent_numbers(a,modulo,count));
    println!("The first {} negative numbers congruent to {}mod{} are {:?}",count,a,modulo,generate_negative_congruent_numbers(a,modulo,count));

}

// a=10
// b=4
// m=3
// a≅b (mod m)
// m|(a-b)
// this function checks if a number is congruent to another
fn is_congruent(a:i128,b:i128,modulo:i128)->bool{
    (a-b)%modulo==0    
}


// this function check for the numbers congruent to a certain input given
// for example we can find numbers congruent to 3mod7 within a certain range
// like having an upperbound and a negative number
// eg: find numbers between -100 and 100 congruent to 3mod7
// so we are looking for x≅3mod7
// meaning what x and divide 7 and give a remainder of 3
// x/7=t remainder 3
// so the format x=7t+3
fn find_congruent_numbers(a:i128,modulo:i128,min:i128,max:i128)->Vec<i128>{
    let mut congruences=Vec::new();
    for i in min..=max{
        if (i % modulo + modulo) % modulo == a % modulo{
            congruences.push(i)
        }
    }
    congruences
}

// using the format that  x=7t+3
// we can say that t={0,1,2.....n}
// so we can find the count of numbers that are congruent to a certain expression
// something like this: find the first 11 positive numbers congruent to x≅3mod7
// a-refers to the congruent : in our example 3
// modulo -refers to mod7 in our example
fn generate_positive_congruent_numbers(a:i128,modulo:i128,count:i128)->Vec<i128>{
    (0..count).map(|x| a+x*modulo).collect()
}

fn generate_negative_congruent_numbers(a:i128,modulo:i128,count:i128)->Vec<i128>{
    (1..count).map(|x| a-x*modulo).collect()
}