#![allow(unused)]
#[cfg(unix)]
use libc::{sockaddr_in,in_addr,close,socket, c_void, sockaddr, sendto, recvfrom, bind};

#[cfg(windows)]
use winapi::shared::inaddr::in_addr;
#[cfg(windows)]
use winapi::ctypes::c_void;
#[cfg(windows)]
use winapi::um::winsock2::{socket,sendto,recvfrom,bind,closesocket as close};
#[cfg(windows)]
use winapi::shared::ws2def::{SOCKADDR_IN as sockaddr_in,SOCKADDR as sockaddr};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::{mem::size_of, process::id, net::Ipv4Addr, convert::TryInto,str::FromStr};
// Flag of DNS header mask (OPCODE):
pub const OPCODE_MASK: u16 = 0b0111_1000_0000_0000;
// Flag of DNS header mask (RD):
pub const RECURSION_DESIRED: u16 = 0b0000_0001_0000_0000;
// Dns server address - Customize it by dns :
pub const DNS_SERVER: Ipv4Addr = Ipv4Addr::new(8,8,8,8);
#[derive(Debug)]
pub struct DnsHeader {
    pub id: u16,
    pub query: bool,
    pub opcode: u16,
    pub authoritative: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub authenticated_data: bool,
    pub checking_disabled: bool,
    pub response_code: u8,
    pub questions: u16,
    pub answers: u16,
    pub nameservers: u16,
    pub additional: u16,
}
// DNS Header Struct 
impl DnsHeader {
    // write query for TYPE A and CLASS IN
    fn make(&self,data:&mut Vec<u8>,url:&str){
        let mut flags = 0u16;
        flags |= Into::<u16>::into(self.opcode) << OPCODE_MASK.trailing_zeros();
        flags |= Into::<u8>::into(self.response_code) as u16;
        flags |= RECURSION_DESIRED;
        BigEndian::write_u16(&mut data[..2], self.id);
        BigEndian::write_u16(&mut data[2..4], flags);
        BigEndian::write_u16(&mut data[4..6], self.questions);
        BigEndian::write_u16(&mut data[6..8], self.answers);
        BigEndian::write_u16(&mut data[8..10], self.nameservers);
        BigEndian::write_u16(&mut data[10..12], self.additional);
        for part in url.split('.') {
            let ln = part.len() as u8;
            data.push(ln);
            data.extend(part.as_bytes());
        }
        data.push(0);
        // Set CLASS flag
        data.write_u16::<BigEndian>(1 as u16).unwrap();
        // Set TYPE flag
        data.write_u16::<BigEndian>(1 as u16 | 0x0000).unwrap();
    }
}
unsafe fn builder(url:&str,dns:Ipv4Addr) -> Result<Vec<u8>,isize> {
    let socket = socket(2,2,17);
    let mut dest = sockaddr_in {
        sin_family : 2,sin_port : 53u16.to_be() as u16,
        sin_addr : in_addr { s_addr: u32::from(dns)},
        sin_zero : [0;8],
    };
    let mut buf = Vec::with_capacity(512);
    let header = DnsHeader {
        id: (id()/10) as u16,
        query:true,opcode:0 as u16,
        authoritative:false,additional:0,authenticated_data:false,
        truncated:false,recursion_available:false,recursion_desired:true,
        response_code:0 as u8,answers:0,
        questions:1 as u16,nameservers:0,checking_disabled:false
    };
    buf.extend([0u8;12].iter());
    header.make(&mut buf,url);
    let length = buf.len();
    let bufs = buf.as_mut_ptr() as *const c_void;
    bind(
        socket,
        &mut dest as *mut sockaddr_in as *mut sockaddr,
        size_of::<sockaddr_in>() as u64 as u32,
    );
    let _sender = sendto(socket,bufs,(length as u64).try_into().unwrap(),0,
            &mut dest as *mut sockaddr_in as *mut sockaddr,
            size_of::<sockaddr_in>() as u64 as u32);
    let mut i = size_of::<sockaddr_in>() as u64 as i32;
    let rec = recvfrom(
        socket,buf.as_mut_ptr() as *mut i8 as *mut c_void,
        (65536 as u64).try_into().unwrap(),0,&mut dest as *mut sockaddr_in as *mut sockaddr,
        &mut i as *mut i32 as *mut u32);
    close(socket);
    if rec > 0 {
        Ok(buf)
    } else {
        Err(rec.try_into().unwrap())
    }
}
// Request to server DNS and retrieve an answer
pub struct Raw {
    dns:Ipv4Addr
}
impl Raw {
    // Set DNS server
    fn set_dns(dns:&str)-> Raw {
        Raw { dns: Ipv4Addr::from_str(dns).expect("Invalid Ip server") }
    }
    // Retreieve a buffer with Default Address
    fn build(&mut self,url:&str) -> Option<Vec<u8>> {
        if self.dns.octets().len() == 4 {
            unsafe {
                builder(url,self.dns).ok()
            }
        } else {
            None
        }
    }
    // Retreieve a buffer 
    fn build_default(url:&str) -> Option<Vec<u8>> {
        unsafe {
            builder(url,DNS_SERVER).ok()
        }
    }
}
// Test
#[cfg(test)]
mod test {
    use Raw;
    #[test]
    fn test(){
        let const_result = [129, 128, 0, 1, 0, 1, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1].to_vec();
        let result = Raw::set_dns("8.8.4.4").build("google.com").unwrap();
        assert_eq!(&result[2..],&const_result[..]);
    }
}