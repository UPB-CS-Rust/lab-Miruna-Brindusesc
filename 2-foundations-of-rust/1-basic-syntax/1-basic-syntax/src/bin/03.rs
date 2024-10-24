/*fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let largest = input.iter().max().unwrap(); 
    let smallest = input.iter().min().unwrap();


    println!("{} is largest and {} is smallest", largest, smallest);
}*/

//iter(): creeaza un iterator peste array-ul input
//max() și min(): returnează un Option<&T>, care contine fie o referinta la elementul maxim/minim, fie None daca array-ul este gol
//unwrap(): extrage valoarea din Option, unwrap() aici pentru ca array-ul nu este gol

fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let n = input.len() - 1;

    let mut largest = input[0];
    let mut smallest = input[0];

    for i in 0..n {
        if input[i] < smallest {
        smallest = input[i]
        }
        if input[i] > largest {
        largest = input[i]
        }
    };

    //let largest = input[5];
    //let smallest = input[6];

    println!("{} is largest and {} is smallest", largest, smallest);
}