
const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// The ASCII code to letter A is 65 wich we map to 0 in our rotor
struct Enigma {
    // Each rotor has 26 positions
    rot1: [usize; 26],
    rot2: [usize; 26],
    rot3: [usize; 26],

    // current rotor in action
    rot_base: usize,
}
// A:10 -> 12 -> 17:R)|
impl Enigma {
    fn spin_rot(&mut self) {
        for index in 0..26 {
            match self.rot_base {
                0 => {
                    self.rot1[index] += 1;
                    if self.rot1[index] == 26 {
                        self.rot1[index] = 0;
                    }
                }
                1 => {
                    self.rot2[index] += 1;
                    if self.rot2[index] == 26 {
                        self.rot2[index] = 0;
                    }
                }
                2 => {
                    self.rot3[index] += 1;
                    if self.rot3[index] == 26 {
                        self.rot3[index] = 0;
                    }
                }
		_ => unimplemented!(),
            }
        }

        if self.rot_base >= 2 {
            self.rot_base = 0;
        } else {
            self.rot_base += 1;
        }
    }

    fn crypt_char(&mut self, ch: char) -> char {
        let ch = ALPHABET[self.rot3[self.rot2[self.rot1[ch as usize - 65]]]];
	self.spin_rot();
	ch
    }

    fn decipher_char(&mut self, ch: char) -> char {
	let p1 = self.rot3.iter().position(|x| *x == (ch as usize - 65)).unwrap();
	let p2 = self.rot2.iter().position(|x| *x == p1).unwrap();
	let p3 = self.rot1.iter().position(|x| *x == p2).unwrap();
	// println!("p1: {}", p3);
	let ch = ALPHABET[p3];
	
	self.spin_rot();
	ch
    }

    fn new(mut p1: usize, mut p2: usize, mut p3: usize) -> Self {
        let mut r1 = [0; 26];
        for index in 0..26 {
            r1[index] = p1;
            if p1 >= 25 {
                p1 = 0;
            } else {
                p1 += 1;
            }
        }

        let mut r2 = [0; 26];
        for index in 0..26 {
            r2[index] = p2;
            if p2 >= 25 {
                p2 = 0;
            } else {
                p2 += 1;
            }
        }

        let mut r3 = [0; 26];
        for index in 0..26 {
            r3[index] = p3;
            if p3 >= 25 {
                p3 = 0;
            } else {
                p3 += 1;
            }
        }
        Self {
            rot1: r1,
            rot2: r2,
            rot3: r3,
            rot_base: 0,
        }
    }
}

fn main() {
    
    let mut enigma = Enigma::new(5,12,1);
    // println!("{:?}\n{:?}\n{:?}\n", enigma.rot1, enigma.rot2, enigma.rot3);
    let mut res: String = String::new();
    let msg = String::from("BASILIAEDIEGOPRASEMPREJUNTOS");
    for ch in msg.as_bytes() {
	 res.push(enigma.crypt_char(*ch as char));
    }
    println!("{}", res);


    let mut enigma = Enigma::new(5,12,1);
    let msg = String::from("TTMDHFY");
    for ch in msg.as_bytes() {
	println!("{} -> {}", *ch as char, enigma.decipher_char(*ch as char));
    }
}
