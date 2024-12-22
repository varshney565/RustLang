fn example1() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
	let x = 1;
	println!("The value of x is : {}",x);
	//x = 10; //this line will give error as x is immutable

	let mut y = 12;
	y = 23;
	println!("The value of y is : {}",y);


	const A : u32 = 12;
	// shadowing with const not allowed as constants gurantees that the value will 
	// not get changed but shadowing changing it at runtime so that's why not-allowed
	// const a : &str = "Helo"; // this line will give the error


	// not allowed as constant can be set to only constant expressions  
	// const b : u32 = y; 

	const B : u32 = A; // allowed as a is a constant 
	example1();
}
