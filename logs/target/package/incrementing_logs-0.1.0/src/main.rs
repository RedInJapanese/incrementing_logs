//turns off warning flags
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

/*
logs.rs:
    Rust library I wrote in order to perform basic logarithms. 
    Although Rust has functions like these that are more efficient, they are only available on the nightly release(I'm on the stable release at the time of writing this).
    These functions(excluding expo()) calculate logarithms by incrementing an exponent up or down based on how close it is to the actual product passed in by the user. 
*/

//Macros that are used in the follow functions(except for expo)
///max used for the minimum value in ilog()
pub const MAX:i64 = 2147483647; //max used for the minimum value in ilog()

///value returned if the passed in arguments are exceptions
pub const UNDEFINED:i64 = 2147483647; //value returned if the passed in arguments are exceptions

///value returned for flog() and flogf() if the passed in arguments are exceptions
pub const FUNDEFINED:f64 = 214748364.7; //value returned for flog() and flogf() if the passed in arguments are exceptions

///eulers constant for ln()
pub const E:f64 = 2.7182818284590452353602874713527; //eulers constant for ln()

///learning rate used in ln(), flog(), and flogf()
pub const ALPHA:f64 = 0.00001; //learning rate used in ln(), flog(), and flogf()

//expo(i64, i64)
//function used by ilog() that calculates the exponent of a 64 bit integer base to the power of a 64 bit integer exponent.
fn expo(base:i64, exponent:i64)->i64 {
    let mut product:i64 = 1;
    let n = exponent.abs();

    for _i in 1..n+1 { //goes up to n+1 so that the last base is multiplied
        if exponent-exponent == 0 { //checks if the exponent being passed in is positive or negative
            product*=base;
        }
        else{
            product*=1/base;
        }
    }
    return product;
}

//ilog(i64, i64)
//Function that returns the logarithm of a 64 bit integer base and a 64 bit integer product(returns the ceiling exponent). 
//This function starts with an exponent of 0 and increments up or down based on if its less than or greater than the product. 
//There is also a scalar called error which is calculated every iteration. The goal is to have it as small as physically possible.

///
/// # What it does
///Function that returns the logarithm of a 64 bit integer base and a 64 bit integer product(returns the ceiling exponent). 
///This function starts with an exponent of 0 and increments up or down based on if its less than or greater than the product. 
///There is also a scalar called error which is calculated every iteration. The goal is to have it as small as physically possible.
/// # Examples
/// ```
/// let i:i64 = log::ilog(10, 2); 
/// assert_eq!(i, 1);
///```
///

pub fn ilog(base:i64, product:i64) ->i64 {
    if base <= 0 || base == 1 || product <= 0 { //conditional that checks for exceptions
        println!("Answer is undefined.");
        return UNDEFINED;
    }    
    let mut min:i64 = MAX; 
    let mut exp:i64 = 0;
    let mut p_hat:i64 = 0; 
    let mut error:i64 = 0;

    loop {
        p_hat = expo(base, exp); //calculates p_hat and the error at the start of every iteration
        error = product-p_hat;
        //conditionals that check where exp should or shouldn't be
        if p_hat < product { 
            exp+=1;
        }
        else if p_hat > product {
            exp-=1;
        }
        else {
            return exp;
        }

        if error < min { //updates the minimum error
            min = error;
        }
        if min != error { //loop continues as long as the minumum error and the current error are the same
            break;
        }
    }
    return exp; //returns exp if the loop terminates
}

//flog(i64, i64)
//Function that returns the logarithm of a 64 bit integer base and a 64 bit integer product as a floating point number. 
//Similar to ilog(), this function increments and decrements the exponent by a constant called ALPHA, aka the learning rate. ALPHA is intentionally set to be small so that the function returns an answer that is as accurate as possible. 
//Unlike ilog(), flog does this process 100 thousand times instead of using a maximum or minimum.

///
/// # What it does
/// Function that returns the logarithm of a 64 bit integer base and a 64 bit integer product as a floating point number. 
///Similar to ilog(), this function increments and decrements the exponent by a constant called ALPHA, aka the learning rate. ALPHA is intentionally set to be small so that the function returns an answer that is as accurate as possible. 
/// # Examples
/// ```
/// let i:f64 = log::flog(10, 2); 
/// assert_eq!(i, 0.3010200000020765);
///```
///
pub fn flog(base:i64, product:i64) ->f64 {
    if base <= 0 || base == 1 || product <= 0 { //cehcking for exceptions
        println!("Answer is undefined.");
        return FUNDEFINED;
    }    
    let b:f64 = base as f64; //floating point conversions
    let p:f64 = product as f64;
    let mut exp:f64 = ilog(base, product) as f64; //finds the integer log exponent then increments and decrements by ALPHA from there
    let mut p_hat:f64 = 0.0; 
    let mut error:f64 = 0.0;
    let mut count = 0;

    while count!=100000 {
        p_hat = b.powf(exp); //calculates p_hat and the error at the start of each iteration
        error = p-p_hat;
        //conditionals to check if exp should be incremented or decremented by ALPHA
        if p_hat < p {
            exp+=ALPHA;
        }
        else if p_hat > p {
            exp-=ALPHA;
        }
        else {
            return exp;
        }
        count+=1;
    }
    return exp;
}

//ln(i64)
//Function that calculates ln using a 64 bit integer as an argument. This function works the same way as flog() with the exception of the base being set to the E macro above.

///
/// # What it does
/// Function that calculates ln using a 64 bit integer as an argument. This function works the same way as flog() with the exception of the base being set to the E macro above.
/// # Examples
/// ```
/// let i:if64 = log::ln(10); 
/// assert_eq!(i, 2.302579999995431);
///```
///

pub fn ln(product:i64) ->f64{
    if product <= 0 {
        println!("Answer is undefined.");
        return FUNDEFINED;
    }    
    let b:f64 = E; //sets the base to euler's constant(2.7813...)
    let p:f64 = product as f64; //floating point conversion
    let mut exp:f64 = ilog(3, product) as f64; //passes in 3 as the base(rounds the constant up) then converts the result to a floating point number
    let mut p_hat:f64 = 0.0; 
    let mut error:f64 = 0.0;
    let mut count = 0;

    while count!=100000 {
        p_hat = b.powf(exp); //calculates p_hat and the error at the start of each iteration
        error = p-p_hat;
        //conditionals to check if exp should be incremented or decremented by ALPHA
        if p_hat < p {
            exp+=ALPHA;
        }
        else if p_hat > p {
            exp-=ALPHA;
        }
        else {
            return exp;
        }
        count+=1;
    }
    return exp;
}

//flogf(f64, f64)
//Function that is similar to flog() with the exception of the base that is passed in. In this case, the base is a 64 bit floating point number.

///
/// # What it does
/// Function that is similar to flog() with the exception of the base that is passed in. In this case, the base is a 64 bit floating point number.
/// # Examples
/// ```
/// let i:f64 = log::flogf(3.14, 5.0); 
/// assert_eq!(i, 1.4065799999961124);
///```
///
pub fn flogf(base:f64, product:f64) ->f64 {
    if base <= 0.0 || base == 1.0 || product <= 0.0 {
        println!("Answer is undefined.");
        return FUNDEFINED;
    }    
    let b:f64 = base;
    let p:f64 = product;
    let mut exp:f64 = ilog(base as i64, product as i64) as f64; //converts the floating point numbers to 64 bit integers then passes them into ilog(). Converts the result to a 64 bit floating point number
    let mut p_hat:f64 = 0.0; 
    let mut error:f64 = 0.0;
    let mut count = 0;

    while count!=100000 {
        p_hat = b.powf(exp); //calculates p_hat and the error at the start of each iteration
        error = p-p_hat;
        //conditionals to check if exp should be incremented or decremented by ALPHA
        if p_hat < p {
            exp+=ALPHA;
        }
        else if p_hat > p {
            exp-=ALPHA;
        }
        else {
            return exp;
        }
        count+=1;
    }
    return exp;
}
//lnf()
//Function that is similar to ln() with the exception of the product that is passed in. In this case, the product is a 64 bit floating point number.

///
/// # What it does
/// Function that is similar to ln() with the exception of the product that is passed in. In this case, the product is a 64 bit floating point number.
/// # Examples
/// ```
/// let i:f64 = log::lnf(10.0); 
/// assert_eq!(i, 2.302579999995431);
///```
///
pub fn lnf(product:f64) ->f64{
    if product <= 0.0 {
        println!("Answer is undefined.");
        return FUNDEFINED;
    }    
    let b:f64 = E; //sets the base to euler's constant
    let p:f64 = product;
    let mut exp:f64 = ilog(3, product as i64) as f64; //passes in the rounded up version of the constant and the product converted to a 64 bit integer. The returned result is then ocnverted to a 64 bit floating point number
    let mut p_hat:f64 = 0.0; 
    let mut error:f64 = 0.0;
    let mut count = 0;


    while count!=100000 {
        p_hat = b.powf(exp); //calculates p_hat and the error at the start of each iteration
        error = p-p_hat;
        //conditionals to check if exp should be incremented or decremented by ALPHA
        if p_hat < p {
            exp+=ALPHA;
        }
        else if p_hat > p {
            exp-=ALPHA;
        }
        else {
            return exp;
        }
        count+=1;
    }
    return exp;
}
