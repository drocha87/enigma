use std::collections::BTreeMap;

struct Enigma {
    // Each rotor has 95 positions so the number of combinations is 78_074_896
    // (95*95*95*95). We start to set the rotors indexed from the given
    // sequences, so if you give the  initial sequence 11 to the first rotor
    // (rot1) then rot1[0] == 11, rot1[1] == 12, and so on... Repeted
    // combinations is forbidden (11 11 11 11). How it works: First step we map
    // the character to its pair in ctable, imagine that this pair is
    // "A1" so everytime your input is A the crypted char is 1 and vice
    // versa. Second step is to get the value in rot1 indexed by ascii code of
    // char 1 less 32, in this
    // case rot1[49 - 32] == 28 (in this specific case, because we started the
    // rot1 with 11). So 28 now is used as entry in next rotor (rot2), wich give
    // us the entry to the next
    // rotor and so on. After the last rotor (rot4) returns its value, we use
    // this value to calculate its ascii code
    // (returned value) + 32. After all these operations we set the current
    // rotor increasing each index by 1. So if your input is the same A as the
    // example above, will generate a completely different
    // result. "AAAAAAAAAAAAAAAAA" will encrypt to a variety of differents
    // chars.
    rot1: [usize; 95],
    rot2: [usize; 95],
    rot3: [usize; 95],
    rot4: [usize; 95],

    // Current rotor in action: Each time enigma works, it advance the current rotor to next
    // position. When the rotor finish all possible rotations, we change rot_base to next rotor.
    rot_base: usize,

    // There are (95*93)^47-47! (each pair has to be unique, ex: "Ab" "B^" " ." "_1" ) combinations.
    // *I'm not a matematician maybe I calculate completely wrong*
    // Every time you input a char we map it trough ctable and get the correspondent
    // Iff correspondent is not found it means the char is not in ctable than we just return
    // the same char. It's allowed to pass 2 combinations or 47 combinations. Off course
    // the idea is to set all the 47 pair combinations
    ctable: BTreeMap<char, char>,
}

impl Enigma {
    // Simulate the rotor spinning in the enigma machine.
    fn inc_rot(rot: &mut [usize; 95]) {
        for index in 0..95 {
            rot[index] += 1;
            if rot[index] == 95 {
                rot[index] = 0;
            }
        }
    }

    // Set initial state to rotor.
    fn set_rot(rot: &mut [usize; 95], count: usize) {
        let mut count = count;
        for index in 0..95 {
            rot[index] = count;
            if count >= 94 {
                count = 0;
            } else {
                count += 1;
            }
        }
    }

    // Spin the rotor and update rot_base.
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
        if !ch.is_ascii_control() {
            let ch = (*self.ctable.get(&ch).unwrap_or(&ch) as usize) - 32;
            let ch = self.rot4[self.rot3[self.rot2[self.rot1[ch]]]];
            self.spin_rot();
            (ch as u8 + 32) as char
        } else {
            ch
        }
    }

    // FIXME: remove unwraps? I think the unwrap is safe here because
    // we are inside slice boundaries 95, it's guarrented because is between
    // 32 and 126, so when we subtract 32 it becomes 95 wich is safe to us to
    // use
    fn decipher_char(&mut self, ch: char) -> char {
        if !ch.is_ascii_control() {
            let ch = (ch as usize) - 32;
            let p0 = self.rot4.iter().position(|x| *x == ch).unwrap();
            let p1 = self.rot3.iter().position(|x| *x == p0).unwrap();
            let p2 = self.rot2.iter().position(|x| *x == p1).unwrap();
            let p3 = self.rot1.iter().position(|x| *x == p2).unwrap();
            let ch = ((p3 as u8) + 32) as char;
            self.spin_rot();
            *self.ctable.get(&ch).unwrap_or(&ch)
        } else {
            ch
        }
    }

    fn new(p1: usize, p2: usize, p3: usize, p4: usize, ctable: BTreeMap<char, char>) -> Self {
        let mut enigma = Self {
            rot1: [0; 95],
            rot2: [0; 95],
            rot3: [0; 95],
            rot4: [0; 95],

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
    let msg = String::from("When the sunlight strikes raindrops in the air, they act as a prism and form a rainbow. The rainbow is a division of white light into many beautiful colors. These take the shape of a long round arch, with its path high above, and its two ends apparently beyond the horizon. There is , according to legend, a boiling pot of gold at one end. People look, but no one ever finds it. When a man looks for something beyond his reach, his friends say he is looking for the pot of gold at the end of the rainbow. Throughout the centuries people have explained the rainbow in various ways. Some have accepted it as a miracle without physical explanation. To the Hebrews it was a token that there would be no more universal floods. The Greeks used to imagine that it was a sign from the gods to foretell war or heavy rain. The Norsemen considered the rainbow as a bridge over which the gods passed from earth to their home in the sky. Others have tried to explain the phenomenon physically. Aristotle thought that the rainbow was caused by reflection of the sun s rays by the rain. Since then physicists have found that it is not reflection, but refraction by the raindrops which causes the rainbows. Many complicated ideas about the rainbow have been formed. The difference in the rainbow depends considerably upon the size of the drops, and the width of the colored band increases as the size of the drops increases. The actual primary rainbow observed is said to be the effect of super-imposition of a number of bows. If the red of the second bow falls upon the green of the first, the result is to give a bow with an abnormally wide yellow band, since red and green light when mixed form yellow. This is a very common type of bow, one showing mainly red and yellow, with little or no green or blue.");

    let mut ctable: BTreeMap<char, char> = BTreeMap::new();
    let t = String::from("a~");
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
    let t = String::from("a~");
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
