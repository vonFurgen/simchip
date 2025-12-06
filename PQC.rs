// ********************
// Project:
// Module:
// Ref:
// 
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::Tcpsock_handler;
use local_ip_address::local_ip;
use std::net::{SocketAddr};
//use kyberlib::*;
use kyberlib::{Keypair, KyberLibError, keypair};
use rand::thread_rng;


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> 
{   // *** Server configuration ***
    let server_ip = local_ip().unwrap();
    let server_port=8080;
    let socket_add = SocketAddr::new(server_ip,server_port);
    //let server_add=server_ip+"."+server_port;
    let sock_handler = Tcpsock_handler::bind(socket_add).await?;
    println!("Server ip : {}",socket_add);
    println!("Server status: listening!");
    // *****Variables definition********
    // Pre-setup of communication module
    let (status,action, ) = (String::new(), String::new(), String::new());
    // *************
    // Communication protocol: Doc.03
    // *************    
    // *************
    // //Desc: Returns the current status of the transceptors 
    // Docs: Doc.01
    // status=chk_transceptors();
    // //Desc: Print the generated actions to UI according with the status of the transceptors and generates respective signal to attached modules    
    // action=action_list(status);
    if action == "EXIT:0"
    {   println!("Server process exited with signal: {}",action);
        process::exit(0);
    }
    // //Desc: Transceptors setup.  
    // status=comm_module_setup(presetup);
    // action=action_list(status);
    // *************
    if action == "EXIT:0"
    {   println!("Server process exited with signal: {}",action);
        process::exit(0);
    }
    loop 
    {   
        let (mut socket, addr) = sock_handler.accept().await?;
        let mut msg="ST:KY";
        socket.write_all(msg.as_bytes()).await?;
        println!("Connection from: {}", addr);
        println!("Sending requirement (ST:KY)");
    
        // To generate the public key 
        //let public_key = public_key.public;
        //let public_key_bytes: &[u8;1184] = public_key.public;
        //To send the key to the remote client
        //socket.write_all(public_key_bytes).await?;    
        
        
        //socket.write_all(msg.as_bytes()).await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client {} disconnected", addr);
                        return;
                    }
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buf[..n]);
                        println!("Received from {}: {}", addr, msg);

                        if socket.write_all(&buf[..n]).await.is_err() {
                            println!("Failed to write to client {}", addr);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from client {}: {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}

// ********** Functions ***************
// Req: Req.016
// Description: 
// It's used to generate the public keys from a pseudo-random number generator (PRNG)
// Date: 
// Author: 
// Pre:  Void
// Post: Return -> Key
fn generateKey()-> Result<Keypair, KyberLibError> 
{
    let mut rng = thread_rng();
    let key = keypair(&mut rng)?;
    Ok(key)
}
fn snd_publicKey(socket)
{
        match generateKey()
        {
            Ok(key) => {
                println!("Keypair generated!"); 
                let public_key = key.public;
                let public_key_bytes: &[u8] = &public_key;
                socket.write_all(public_key_bytes).await?;        
                //Ok(key)
            }
            Err(KyberLibError::InvalidKey) => {
                eprintln!("Error: Invalid key");
                //Err(KyberLibError::InvalidKey) 
            }
            Err(KyberLibError::RandomBytesGeneration) => {
                eprintln!("Error: Failed to generate bytes");
                //Err(KyberLibError::RandomBytesGeneration)
            }
            Err(e) => {
                eprintln!("Unexpected error occurred: {:?}", e);
                //KyberLibError(KyberLibError)
            }
        }
}

