use std::env;


/**
 * Starting client
 */
pub fn start() {
    println!("\n----- Welcome to the MBRB client -----\n");
    help();
}

/**
 * Set the values passed in the main function (if existing)
 */
pub fn get_user_start_input() ->(String,String) {
    // Default values
    let mut arg = "0,127.0.0.1:8080".to_string();

    // Collecting parameters passed in main function by the user
    let args: Vec<String> = env::args().collect();

    // Check there values have been passed in main function
    match args.len() {
        1 => println!("Default values set"),
        2 => arg = args[1].clone(),
        _ => println!("Too many arguments"),
    }
    let (id,sock_addr) = (arg.split(',').nth(0).unwrap(),arg.split(',').nth(1).unwrap());
    (id.to_string(),sock_addr.to_string())
}

/**
 * Wait for a broadcast command by the user
 */
pub fn listen_broadcast_command() -> String {
    let mut good_command = false;
    let mut message = "".to_string();
    while !good_command {
        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).unwrap();
        let parameters:Vec<String> = cmd.split(" ").map(|s| s.to_string()).collect();
        if parameters.len()==2 && parameters[0]=="broadcast".to_string() {
            message = parameters[1].clone();
            good_command = true;
        } else {
            print!("\nUnreadable command. ");
            help();
        }
    }
    println!();
    message[0..message.len()-1].to_string()
}

/**
 * Print the commands 
 */
pub fn help() {
    println!("Command is :\nbroadcast 'message'\n");
}


///////////////////////////////////////////
//////////// Unused functions /////////////
///////////////////////////////////////////

// /**
//  * Wait for a send command by the user
//  */
// pub fn listen_send_command() -> (std::net::SocketAddrV4,String) {
//     let mut good_command = false;
//     let (mut sock_addr,mut message) = (std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127,0,0,1),8080),"".to_string());
//     while !good_command {
//         let mut cmd = String::new();
//         std::io::stdin().read_line(&mut cmd).unwrap();
//         let mut iter_cmd = cmd.as_str().split_whitespace();
//         match iter_cmd.next() {
//             Some("send") => {
//                 if iter_cmd.nth(0).is_some() && iter_cmd.nth(0).is_some() {
//                     let mut iter_cmd_bis = cmd.as_str().split_whitespace();
//                     let to_receiver = iter_cmd_bis.nth(1).unwrap();
//                     message = iter_cmd_bis.next().unwrap().to_string();
//                     if let Ok(res) = to_receiver.parse::<std::net::SocketAddrV4>() {
//                         sock_addr = res;
//                         good_command = true;
//                     } else {
//                         println!("unreadable socket adress");
//                     }
//                 } else {
//                     println!("Unreadable command, syntax is : send 'ipv4:port' 'message'");
//                 }
//             },
//             Some("broadcast") => {
//                 if iter_cmd.next().is_some() {

//                 }
//             }
//             _ => println!("Unreadable command, syntax is : send 'ipv4:port' 'message'"),
//         }
//     }
//     (sock_addr,message)
// }