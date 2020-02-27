use std::collections::BTreeMap;

const ALPHABET: [char; 59] = [
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z',
];

// The ASCII code to letter A is 64 wich we map to 0 in our rotor
struct Enigma {
    // Each rotor has 59 positions
    rot1: [usize; 59],
    rot2: [usize; 59],
    rot3: [usize; 59],
    rot4: [usize; 59],

    // current rotor in action
    rot_base: usize,
    ctable: BTreeMap<char, char>,
}
// A:10 -> 12 -> 17:R)|
impl Enigma {
    fn inc_rot(rot: &mut [usize; 59]) {
        for index in 0..59 {
            rot[index] += 1;
            if rot[index] == 59 {
                rot[index] = 0;
            }
        }
    }

    fn set_rot(rot: &mut [usize; 59], count: usize) {
        let mut count = count;
        for index in 0..59 {
            rot[index] = count;
            if count >= 58 {
                count = 0;
            } else {
                count += 1;
            }
        }
    }

    fn spin_rot(&mut self) {
        match self.rot_base {
            0 => Enigma::inc_rot(&mut self.rot1),
            1 => Enigma::inc_rot(&mut self.rot2),
            2 => Enigma::inc_rot(&mut self.rot3),
            3 => Enigma::inc_rot(&mut self.rot4),
            _ => unimplemented!(),
        }
        if self.rot_base >= 3 {
            self.rot_base = 0;
        } else {
            self.rot_base += 1;
        }
    }

    fn crypt_char(&mut self, ch: char) -> char {
        if (ch as usize) >= 64 && (ch as usize) <= 122 {
	    let ch = *self.ctable.get(&ch).unwrap_or(&ch);
            let ch = ALPHABET[self.rot4[self.rot3[self.rot2[self.rot1[ch as usize - 64]]]]];
            self.spin_rot();
            ch
        } else {
            ch
        }
    }

    // FIXME: remove unwraps
    fn decipher_char(&mut self, ch: char) -> char {
        if (ch as usize) >= 64 && (ch as usize) <= 122 {
            let p0 = self
                .rot4
                .iter()
                .position(|x| *x == (ch as usize - 64))
                .unwrap();
            let p1 = self.rot3.iter().position(|x| *x == p0).unwrap();
            let p2 = self.rot2.iter().position(|x| *x == p1).unwrap();
            let p3 = self.rot1.iter().position(|x| *x == p2).unwrap();
            let ch = ALPHABET[p3];
            self.spin_rot();
            *self.ctable.get(&ch).unwrap_or(&ch)
        } else {
            ch
        }
    }

    fn new(p1: usize, p2: usize, p3: usize, p4: usize, ctable: BTreeMap<char, char>) -> Self {
        let mut enigma = Self {
            rot1: [0; 59],
            rot2: [0; 59],
            rot3: [0; 59],
            rot4: [0; 59],

            rot_base: 0,
            ctable,
        };
        Enigma::set_rot(&mut enigma.rot1, p1);
        Enigma::set_rot(&mut enigma.rot2, p2);
        Enigma::set_rot(&mut enigma.rot3, p3);
        Enigma::set_rot(&mut enigma.rot4, p4);
        enigma
    }
}

fn main() {
    let msg = String::from("When the sunlight strikes raindrops in the air, they act as a prism and form a rainbow. The rainbow is a division of white light into many beautiful colors. These take the shape of a long round arch, with its path high above, and its two ends apparently beyond the horizon. There is , according to legend, a boiling pot of gold at one end. People look, but no one ever finds it. When a man looks for something beyond his reach, his friends say he is looking for the pot of gold at the end of the rainbow. Throughout the centuries people have explained the rainbow in various ways. Some have accepted it as a miracle without physical explanation. To the Hebrews it was a token that there would be no more universal floods. The Greeks used to imagine that it was a sign from the gods to foretell war or heavy rain. The Norsemen considered the rainbow as a bridge over which the gods passed from earth to their home in the sky. Others have tried to explain the phenomenon physically. Aristotle thought that the rainbow was caused by reflection of the sun’s rays by the rain. Since then physicists have found that it is not reflection, but refraction by the raindrops which causes the rainbows. Many complicated ideas about the rainbow have been formed. The difference in the rainbow depends considerably upon the size of the drops, and the width of the colored band increases as the size of the drops increases. The actual primary rainbow observed is said to be the effect of super-imposition of a number of bows. If the red of the second bow falls upon the green of the first, the result is to give a bow with an abnormally wide yellow band, since red and green light when mixed form yellow. This is a very common type of bow, one showing mainly red and yellow, with little or no green or blue.");

    let mut ctable: BTreeMap<char, char> = BTreeMap::new();
    let t = String::from("a[bFr@zWLMdR]BAy`i");
    let mut citer = t.chars();
    loop {
        let k = citer.next();
        let v = citer.next();
        if k.is_none() || v.is_none() {
            break;
        }
        let k = k.unwrap();
        let v = v.unwrap();
        ctable.insert(k, v);
        ctable.insert(v, k);
    }

    let mut res1: String = String::new();
    let mut enigma = Enigma::new(5, 25, 0, 12, ctable);

    for ch in msg.chars() {
        let ch = enigma.crypt_char(ch);
        res1.push(ch);
    }

    println!("{}", res1);

    let mut res: String = String::new();
    let mut ctable: BTreeMap<char, char> = BTreeMap::new();
    let t = String::from("a[bFr@zWLMdR]BAy`i");
    let mut citer = t.chars();
    loop {
        let k = citer.next();
        let v = citer.next();
        if k.is_none() || v.is_none() {
            break;
        }
        let k = k.unwrap();
        let v = v.unwrap();
        ctable.insert(k, v);
        ctable.insert(v, k);
    }

    let mut enigma = Enigma::new(5, 25, 0, 12, ctable);
    for ch in res1.chars() {
        let ch = enigma.decipher_char(ch);
        res.push(ch);
    }
    println!("{}", res);
}
