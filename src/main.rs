use std::collections::BTreeMap;

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
    ctable: BTreeMap<char, char>,
}
// A:10 -> 12 -> 17:R)|
impl Enigma {
    fn inc_rot(rot: &mut [usize; 26]) {
        for index in 0..26 {
            rot[index] += 1;
            if rot[index] == 26 {
                rot[index] = 0;
            }
        }
    }

    fn set_rot(rot: &mut [usize; 26], count: usize) {
        let mut count = count;
        for index in 0..26 {
            rot[index] = count;
            if count >= 25 {
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
            _ => unimplemented!(),
        }
        if self.rot_base >= 2 {
            self.rot_base = 0;
        } else {
            self.rot_base += 1;
        }
    }

    fn crypt_char(&mut self, ch: char) -> char {
        if ch != ' ' {
            let ch = ALPHABET[self.rot3[self.rot2[self.rot1[ch as usize - 65]]]];
            self.spin_rot();
            *self.ctable.get(&ch).unwrap_or(&ch)
        } else {
            ch
        }
    }

    // FIXME: remove unwraps
    fn decipher_char(&mut self, ch: char) -> char {
        if ch != ' ' {
            let p1 = self
                .rot3
                .iter()
                .position(|x| *x == (ch as usize - 65))
                .unwrap();
            let p2 = self.rot2.iter().position(|x| *x == p1).unwrap();
            let p3 = self.rot1.iter().position(|x| *x == p2).unwrap();
            let ch = ALPHABET[p3];
            self.spin_rot();
            *self.ctable.get(&ch).unwrap_or(&ch)
        } else {
            ch
        }
    }

    fn new(p1: usize, p2: usize, p3: usize, ctable: BTreeMap<char, char>) -> Self {
        let mut enigma = Self {
            rot1: [0; 26],
            rot2: [0; 26],
            rot3: [0; 26],

            rot_base: 0,
            ctable,
        };
        Enigma::set_rot(&mut enigma.rot1, p1);
        Enigma::set_rot(&mut enigma.rot2, p2);
        Enigma::set_rot(&mut enigma.rot3, p3);
        enigma
    }
}

fn main() {
    let msg = String::from("ITS MY LIFE IS NOW OR NEVER");

    let mut ctable: BTreeMap<char, char> = BTreeMap::new();
    let t = String::from("ABIJHUTKZWSVGLCFDE");
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

    let mut res: String = String::new();
    let mut enigma = Enigma::new(0, 25, 0, ctable);

    for ch in msg.chars() {
        let ch = enigma.crypt_char(ch);
        res.push(ch);
    }

    println!("{}", res);

    let mut res: String = String::new();
    let mut ctable: BTreeMap<char, char> = BTreeMap::new();
    let t = String::from("AGBIJHUTLKZWSV");
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

    let msg = String::from("AZPWGYE D DCGDS UXN DNWADR VIDLGO");
    let mut enigma = Enigma::new(5, 12, 1, ctable);

    for ch in msg.chars() {
        let ch = enigma.decipher_char(ch);
        res.push(ch);
    }
    println!("{}", res);
}
