fn main() {

    farhen_to_cel(120);
    cel_to_farhen(30);
}

// Farhenheight to celcius temparature convert 

fn farhen_to_cel (temp : i32 ) { 

    let to_cel  = (temp  - 32 ) * 5/9;
    println!("{} Temp in Celsius is {};", temp, to_cel);
}

fn cel_to_farhen (  temp : i32) { 
    let to_farhen  = (temp  * 9/5  ) + 32;
    println!("{} temp celcius to  farhenheight is {}" , temp, to_farhen);
}
