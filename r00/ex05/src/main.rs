fn ft_print_comb() {
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10{
                println!("{}{}{}\n",a,b,c);
            }
        }
    }
}    


fn main() {
    ft_print_comb();
}