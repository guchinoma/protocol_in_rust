use utuntap::tap::OpenOptions;

#[derive(Debug)]
enum CFD {
    Fname(OpenOptions),
    None,
}

#[derive(Debug, Clone)]
enum V4_V6_Addr {
    V4(u8, u8, u8, u8),
    V6(u8, u8, u8, u8, u8, u8, u8, u8),
    None,
}

#[derive(Debug)]
pub struct Nif {
    pub addr_v4: V4_V6_Addr,
    pub addr_v6: V4_V6_Addr,
    pub flag: u8,
    pub fd: CFD,
}

impl Nif {
    pub fn new(v4: V4_V6_Addr, v6: V4_V6_Addr, f: u8, d: CFD) -> Self {
        Self {
            addr_v4: v4,
            addr_v6: v6,
            flag: f,
            fd: d,
        }
    }
}

//impl Info for Nif {
//fn info(&self) {
//println!("v4 {:}", self.addr_v4);
//println!("v6 {:}", self.addr_v6);
//println!("flag {:}", self.flag);
//}
//}

pub fn nif_loop_init() -> Vec<Nif> {
    let mut v: Vec<Nif> = vec![];
    v.push(Nif::new(
        V4_V6_Addr::V4(127, 0, 0, 1),
        V4_V6_Addr::V6(0, 0, 0, 0, 0, 0, 0, 1),
        0,
        CFD::None,
    ));
    v
}

pub fn nif_tap_init(nlist: &mut Vec<Nif>, a: V4_V6_Addr) {
    let mut f = OpenOptions::new().open(0).expect("error in open tap");
    match a {
        V4_V6_Addr::V4(_, _, _, _) => {
            nlist.push(Nif::new(a, V4_V6_Addr::None, 0, CFD::Fname(f)));
        }
        V4_V6_Addr::V6(_, _, _, _, _, _, _, _) => {
            nlist.push(Nif::new(V4_V6_Addr::None, a, 0, CFD::Fname(f)));
        }
        _ => {
            println!("error in tap init");
        }
    }
}

pub fn nif_init(addr: V4_V6_Addr) -> Vec<Nif> {
    let mut nif_list = nif_loop_init();
    nif_tap_init(&mut nif_list, addr);
    nif_list
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let mut ans: Vec<Nif> = vec![];
        ans.push(Nif::new(
            V4_V6_Addr::V4(127, 0, 0, 1),
            V4_V6_Addr::V6(0, 0, 0, 0, 0, 0, 0, 1),
            0,
            CFD::None,
        ));
        ans.push(Nif::new(
            V4_V6_Addr::V4(192, 168, 100, 5),
            V4_V6_Addr::None,
            0,
            CFD::None,
        ));
        let a = nif_init(V4_V6_Addr::V4(192, 168, 100, 5));
        println!("{:?}", a);
        //assert!(a == ans);
    }
}
