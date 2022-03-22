#![deny(elided_lifetimes_in_paths)]
extern crate string_template;
use std::collections::HashMap;
use string_template::Template;
extern crate num_cpus;

use zeromq::{Socket, SocketRecv, SocketSend, ZmqMessage};



#[tokio::main]
async fn main() {
    println!("IPC to TCP Bridge with Database Logging");
    let num = num_cpus::get();
    println!("Number of cpu core :{}", num);
    create_zmq_pullers(num).await;

    // let x: u8 = 2;
    // let i: u8 = 3;

    // let handle = tokio::spawn(create_zmq_socket(x, i));

    // // Do some other work

    // let out = handle.await.unwrap();
}

async fn create_zmq_socket(x: u8, i: u8) {
    let ipc_url_template = Template::new("ipc:///tmp/zmqSendData_{{x}}_{{i}}");
    let mut cl = 0;
    let mut vec = Vec::new();
    while cl < x {
        let mut cpu = 0;
        while cpu < i {
            let mut args = HashMap::new();
            let str_cl = cl.to_string();
            let str_cpu = cpu.to_string();
            args.insert("x", str_cl.as_str());
            args.insert("i", str_cpu.as_str());
            let  ipc_url_path = ipc_url_template.render(&args);
                    let handle = tokio::spawn(generate_socket(String::from(ipc_url_path)));
                    vec.push(handle);
            cpu = cpu + 1;
        }
        cl = cl + 1;
    }
}

async fn generate_socket(ipc_url_path:String ) {
    let mut socket = zeromq::PullSocket::new();
    println!("Connecting to :: {}",ipc_url_path);
    socket.connect(ipc_url_path.as_str()).await.expect("Failed to connect");

    loop {
        let ipc_data = socket.recv().await;
        match  ipc_data {
            Ok(res) => print_zmq(res),
            Err(err) => print!("ZMQ ERROR !"),
        }
        
    }
}

fn print_zmq(data : ZmqMessage){
    let workload = String::from_utf8(data.get(0).unwrap().to_vec()).expect("Couldn't parse u64 from data");
    println!("Data from ZMQ :: {}",workload);
}

async fn create_zmq_pullers(num_of_cpus: usize) {
    if 64 <= num_of_cpus {
        println!("CL-->4 :: PROCESS-->8");
        create_zmq_socket(4, 8).await;
    } else if 48 <= num_of_cpus && num_of_cpus < 64 {
        println!("CL-->3 :: PROCESS-->8");
        create_zmq_socket(3, 8).await;
    } else if 32 <= num_of_cpus && num_of_cpus < 48 {
        println!("CL-->2 :: PROCESS-->8");
        create_zmq_socket(2, 8).await;
    } else if 24 <= num_of_cpus && num_of_cpus < 32 {
        println!("CL-->2 :: PROCESS-->6");
        create_zmq_socket(2, 6).await;
    } else if 20 <= num_of_cpus && num_of_cpus < 24 {
        println!("CL-->1 :: PROCESS-->9");
        create_zmq_socket(1, 9).await;
    } else if 16 <= num_of_cpus && num_of_cpus < 20 {
        println!("CL-->1 :: PROCESS-->8");
        create_zmq_socket(1, 8).await;
    } else if 12 <= num_of_cpus && num_of_cpus < 16 {
        println!("CL-->1 :: PROCESS-->6");
        create_zmq_socket(1, 6).await;
    } else if 8 <= num_of_cpus && num_of_cpus < 12 {
        println!("CL-->1 :: PROCESS-->4");
        create_zmq_socket(1, 4).await;
    } else if 4 <= num_of_cpus && num_of_cpus < 8 {
        println!("CL-->1 :: PROCESS-->2");
        create_zmq_socket(1, 2).await;
    } else {
        println!("CL-->1 :: PROCESS-->1");
        create_zmq_socket(1, 1).await;
    }
}
