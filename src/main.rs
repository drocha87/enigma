use std::collections::BTreeMap;

const BASE: u8 = 32;
const N_CH: usize = 95;

struct Enigma {
    // Each rotor has N_CH positions so the number of combinations is 78_074_896
    // (N_CH*N_CH*N_CH*N_CH). We start to set the rotors indexed from the given
    // sequences, so if you give the  initial sequence 11 to the first rotor
    // (rot1) then rot1[0] == 11, rot1[1] == 12, and so on... Repeted
    // combinations is forbidden (11 11 11 11). How it works: First step we map
    // the character to its pair in ctable, imagine that this pair is
    // "A1" so everytime your input is A the crypted char is 1 and vice
    // versa. Second step is to get the value in rot1 indexed by ascii code of
    // char 1 less BASE, in this
    // case rot1[49 - BASE] == 28 (in this specific case, because we started the
    // rot1 with 11). So 28 now is used as entry in next rotor (rot2), wich give
    // us the entry to the next
    // rotor and so on. After the last rotor (rot4) returns its value, we use
    // this value to calculate its ascii code
    // (returned value) + BASE. After all these operations we set the current
    // rotor increasing each index by 1. So if your input is the same A as the
    // example above, will generate a completely different
    // result. "AAAAAAAAAAAAAAAAA" will encrypt to a variety of differents
    // chars.
    rot1: [usize; N_CH],
    rot2: [usize; N_CH],
    rot3: [usize; N_CH],
    rot4: [usize; N_CH],

    // Current rotor in action: Each time enigma works, it advance the current rotor to next
    // position. When the rotor finish all possible rotations, we change rot_base to next rotor.
    rot_base: usize,

    // There are (N_CH*93)^47-47! (each pair has to be unique, ex: "Ab" "B^" " ." "_1" ) combinations.
    // *I'm not a matematician maybe I calculate completely wrong*
    // Every time you input a char we map it trough ctable and get the correspondent
    // Iff correspondent is not found it means the char is not in ctable than we just return
    // the same char. It's allowed to pass 2 combinations or 47 combinations. Off course
    // the idea is to set all the 47 pair combinations
    ctable: BTreeMap<char, char>,
}

impl Enigma {
    // Simulate the rotor spinning in the enigma machine.
    fn inc_rot(rot: &mut [usize; N_CH]) {
        for index in 0..N_CH {
            rot[index] += 1;
            if rot[index] == N_CH {
                rot[index] = 0;
            }
        }
    }

    // Set initial state to rotor.
    fn set_rot(rot: &mut [usize; N_CH], count: usize) {
        let mut count = count;
        for index in 0..N_CH {
            rot[index] = count;
            if count >= N_CH - 1 {
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

    fn gen_random_conf() -> (usize, usize, usize, usize, String) {
        let mut char_table =
	    String::from(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
        let mut res = String::new();
        while char_table.len() > 0 {
            let k = rand::random::<usize>() % char_table.len();
            res.push(char_table.remove(k));
        }
        (
            rand::random::<usize>() % N_CH,
            rand::random::<usize>() % N_CH,
            rand::random::<usize>() % N_CH,
            rand::random::<usize>() % N_CH,
            res,
        )
    }

    fn cipher(&mut self, txt: String) -> String {
        let mut res = String::new();
        for ch in txt.chars() {
            if !ch.is_ascii_control() {
                let ch = (*self.ctable.get(&ch).unwrap_or(&ch) as usize) - BASE
        as usize;
                let ch = self.rot4[self.rot3[self.rot2[self.rot1[ch]]]];
                self.spin_rot();
                res.push((ch as u8 + BASE) as char);
            } else {
                res.push(ch);
            }
        }
        res
    }

    // FIXME: remove unwraps? I think the unwrap is safe here because
    // we are inside slice boundaries N_CH, it's guarrented because is between
    // BASE and 126, so when we subtract BASE it becomes N_CH wich is safe to us to
    // use
    fn decipher(&mut self, txt: String) -> String {
        let mut res = String::new();
        for ch in txt.chars() {
            if !ch.is_ascii_control() {
                let ch = (ch as usize) - BASE as usize;
                let p0 = self.rot4.iter().position(|x| *x == ch).unwrap();
                let p1 = self.rot3.iter().position(|x| *x == p0).unwrap();
                let p2 = self.rot2.iter().position(|x| *x == p1).unwrap();
                let p3 = self.rot1.iter().position(|x| *x == p2).unwrap();
                let ch = ((p3 as u8) + BASE) as char;
                self.spin_rot();
                res.push(*self.ctable.get(&ch).unwrap_or(&ch));
            } else {
                res.push(ch);
            }
        }
        res
    }

    fn new(p1: usize, p2: usize, p3: usize, p4: usize, cmap: String) -> Self {
        let mut ctable: BTreeMap<char, char> = BTreeMap::new();
        let mut citer = cmap.chars();
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
        let mut enigma = Self {
            rot1: [0; N_CH],
            rot2: [0; N_CH],
            rot3: [0; N_CH],
            rot4: [0; N_CH],

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
    let msg = String::from("Diego");

    let (a, b, c, d, s) = Enigma::gen_random_conf();

    let mut res1 = Enigma::new(a, b, c, d, s.clone()).cipher(msg);
    println!("{}", res1);

    let res = Enigma::new(a, b, c, d, s).decipher(res1);
    println!("{}", res);
}
