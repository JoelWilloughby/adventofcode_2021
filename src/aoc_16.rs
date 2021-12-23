#[derive(Debug, Clone)]
enum InnerPacket {
    Literal(usize),
    Operator{
        args: Vec<Packet>,
    }
}

#[derive(Debug, Clone, Copy)]
enum LengthD {
    Bits(usize),
    Args(usize),
}

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    id: usize,
    inner: InnerPacket,
}

#[derive(Debug, Clone)]
struct BitBasher {
    bytes: Vec<u8>,
    curr_bit: usize,
}

impl BitBasher {
    fn new(input: &Vec<u8>) -> Self {
        Self {
            bytes: input.clone(),
            curr_bit: 0,
        }
    }


    fn take(&mut self, mut bits: usize) -> usize {
        let target_byte = (self.curr_bit + bits) / 8;
        let mut current_byte = self.curr_bit / 8;
        let mut acc = 0;

        while current_byte < target_byte {
            let rem_bits = 8 * (current_byte + 1) - self.curr_bit;

            acc <<= rem_bits;
            acc += if rem_bits < 8 {
                let t = self.bytes[current_byte] << (8 - rem_bits);
                t as usize >> (8 - rem_bits)
            } else {
                self.bytes[current_byte] as usize
            };

            self.curr_bit += rem_bits;
            current_byte += 1;
            bits -= rem_bits;
        }
        if bits > 0 {
            acc <<= bits;
            let used_bits = self.curr_bit % 8;
            let val = self.bytes[current_byte] << used_bits;
            acc += val as usize >> (8 - bits);
            self.curr_bit += bits;
        }

        acc
    }

    fn parse_literal(&mut self) -> usize {
        let mut acc = 0;
        loop {
            let val = self.take(5);
            acc <<= 4;
            acc += val & 0xF;
            if val < 0x10 {
                break;
            }
        }

        acc
    }

    fn parse_length(&mut self) -> LengthD {
        let d = self.take(1);
        if d == 1 {
            LengthD::Args(self.take(11))
        } else {
            LengthD::Bits(self.take(15))
        }
    }
}

fn read_it(filename: &str) -> Vec<Vec<u8>> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut packets = vec![];
    for line in input.lines() {
        let mut vals = vec![];
        let mut chars = line.trim().chars();
        while let Some(n) = chars.next() {
            let m = chars.next().unwrap();
            let byte = [n, m].iter().fold(0u8, |acc, x|
                if *x as u8 >= '0' as u8 && *x as u8 <= '9' as u8 {
                    (acc << 4) + (*x as u8 - '0' as u8)
                }
                else if *x as u8 >= 'A' as u8 && *x as u8 <= 'F' as u8 {
                    (acc << 4) + 10 + (*x as u8 - 'A' as u8)
                } else {
                    acc
                }
            );

            vals.push(byte);
        }
        packets.push(vals);
    }

    packets
}

impl Packet {
    fn from_basher(bits: &mut BitBasher) -> Self {
        // println!("Parsing new packet: ");
        let version = bits.take(3);
        let id = bits.take(3);
        // println!("  v: {}, id: {}", version, id);

        match id {
            4 => {
                Self {
                    version,
                    id,
                    inner: InnerPacket::Literal(bits.parse_literal()),
                }
            },
            _ => {
                let lengthd = bits.parse_length();
                let args = match lengthd {
                    LengthD::Args(n) => {
                        (0..n).map(|_| Self::from_basher(bits)).collect()
                    },
                    LengthD::Bits(n) => {
                        let target = bits.curr_bit + n;
                        let mut a = vec![];
                        while bits.curr_bit < target {
                            a.push(Self::from_basher(bits));
                        }
                        a
                    }
                };
                Self {
                    version,
                    id,
                    inner: InnerPacket::Operator {
                        args
                    }
                }
            }
        }
    }

    fn from_vec(input: &Vec<u8>) -> Self {
        let mut bits = BitBasher::new(input);

        Self::from_basher(&mut bits)
    }

    fn sum_version(&self) -> usize {
        let mut acc = self.version;
        match &self.inner {
            InnerPacket::Literal(_) => {},
            InnerPacket::Operator{args} => {
                acc += args.iter().fold(0, |acc, x| acc + x.sum_version());
            }
        }
        acc
    }

    fn eval(&self) -> usize {
        match &self.inner {
            InnerPacket::Literal(val) => { *val },
            InnerPacket::Operator{args} => {
                match self.id {
                    0 => {
                        args.iter().fold(0, |acc, x| acc + x.eval())
                    },
                    1 => {
                        args.iter().fold(1, |acc, x| acc * x.eval())
                    },
                    2 => {
                        args.iter().fold(std::usize::MAX, |acc, x| std::cmp::min(acc, x.eval()))
                    },
                    3 => {
                        args.iter().fold(0, |acc, x| std::cmp::max(acc, x.eval()))
                    },
                    5 => {
                        if args[0].eval() > args[1].eval() { 1 } else { 0 }
                    },
                    6 => {
                        if args[0].eval() < args[1].eval() { 1 } else { 0 }
                    },
                    7 => {
                        if args[0].eval() == args[1].eval() { 1 } else { 0 }
                    },
                    _ => {
                        0
                    }
                }
            }
        }
    }
}

fn drive(filename: &str) {
    let bytes = read_it(filename);
    for packet_bytes in bytes.iter() {
        let packet = Packet::from_vec(packet_bytes);
        println!("{}", packet.sum_version());
    }
}

fn drive_2(filename: &str) {
    let bytes = read_it(filename);
    for packet_bytes in bytes.iter() {
        let packet = Packet::from_vec(packet_bytes);
        println!("{}", packet.eval());
    }
}

#[test]
fn part_0() {
    drive("res/16/sample.txt");
    drive_2("res/16/sample_2.txt");
}

#[test]
fn part_1() {
    drive("res/16/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/16/input.txt");
}
