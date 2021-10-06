fn ft_print_combn(n : i32) { 
    let mut vmax = 10;
    for i in 0..n-1 {
        vmax *= 10;
    }
    for compteur in 0..vmax {
        println!("{}",compteur);
    }
}


fn main() {
    ft_print_combn(3)
}
