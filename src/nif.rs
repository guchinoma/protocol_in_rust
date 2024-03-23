use std::net{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct NetDevice{ 
    nif_index: usize,
}

pub struct Nif{
    addr_v4: Ipv4Addr,
    addr_v6: Ipv6Addr,
    flag: u8,
    type: u8,
    dev_index: usize,
}

impl Nif{
    fn new(){}
}

pub nif_loop_init()->Result<mut Vec<Nif>, Err()>{
    n:Vec<Nif> = Vec::new();
    n.push(Nif::new(

    ));
    n
}

pub fn nif_init()->Result<mut Vec<Nif>, Err()>{
    first_nif: Nif = nif_loop_init().expect("error in loop init");
}
