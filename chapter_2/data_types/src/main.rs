fn main() {
	//Scaler data-types
	//Integer,Float,Bool,Char
	//Integer => 8-bit, 16-bit, 32-bit, 64-bit, 128-bit, arch(isize or usize) 
	//default i32
	let _a0 = 123_456; 
	let _a1 = 0xff;
	let _a2 = 0b1010;
	let _a3 = 0o00;
	let _a4 = b'A';

	//Float
	//default f64
	let _b : f32 = 12.3;
	  
	//Boolean
	let _t = true;
	let _f = false;

	//Character
	let _c = 'z';
	
	//Compound data-types
	//tuple type
	let tup = ("shiavm",24);
	let (_name,_age) = tup; //or tup.0, tup.1

	//Arrays
	let _arr : [i32; 3] = [1,2,3];
	let brr : [i32; 8] = [0; 8];
	println!("{:?}",brr);
}
