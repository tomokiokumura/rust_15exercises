fn main() {
    println!("Find Fibonacci sequence up to 20 items.");
    let mut a = [0; 20];
    a[0] = 0;
    a[1] = 1;
    for i in 2..20 {
        a[i] = a[i - 1] + a[i - 2];
    }

    println!("Swap a[0] and a[19].");
    let temp = a[19];
    a[19] = a[0];
    a[0] = temp;

    let mut max = 0;
    let mut min = 1000;

    for i in 0..20{
        if a[i] > max {
            max = a[i];
        }
        if a[i] < min {
            min = a[i];
        }
        println!("a[{}]: {}", i, a[i]);
    }
    println!("Max: {}\nMin: {}\n", max, min);
}
