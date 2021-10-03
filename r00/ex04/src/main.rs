fn ft_putchar(c:char) {
    println!("{}",c);
}


fn ft_is_negative(x : i32) {
    if x < 0 {
        ft_putchar('N');
    }
    else {
        ft_putchar('P')
    }
}


fn main() {
    ft_is_negative(-8);
    ft_is_negative(8);
}
