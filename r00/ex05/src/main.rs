fn ft_print_comb() {
    for (a = 0 ; a < 9 ; a++){
        for (b = 0 ; b < 9 ; b++){
            for (c = 0 ; c < 9 ; c++){
                print("{}{}{}\n",a,b,c);
            }
        }
    }
}    


fn main() {
    ft_print_comb();
}