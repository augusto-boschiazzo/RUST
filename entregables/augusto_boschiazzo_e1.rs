use random_number::random;

pub fn e1(){
    let arreglo = [10, 3, 5, 7, 88];
    let n = random!(..=4);
    if arreglo[n] % 2 == 0{
        println!("{}", arreglo[n]);
    }else{
        println!("{}", -1);
    }
}

fn main(){
    e1();
}