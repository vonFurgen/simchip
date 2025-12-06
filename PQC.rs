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
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, OsRng, generic_array::GenericArray};

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
        //*** Encryption process ***
        let key=snd_publicKey(socket);
        let mut shared_secret=rcv_sharedsecret(socket,key);
        let err=encrypt_msg(socket,shared_secret,msg);


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
// Req: Req.016
// Description: 
// It's used to send the public key to client and return the generated key
// Date: 
// Author: 
// Pre:  socket handler
// Post: Return -> Key
fn snd_publicKey(socket: &mut TcpStream)
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

// Req: Req.016
// Description: 
// This function reads the ciphered text from client
// Date: 
// Author: 
// Pre:  socket handler, generated key
// Post: secret

fn rcv_sharedsecret(socket: &mut TcpStream,key: &PublicKey)-> Result<[u8; KYBER_SHARED_SECRET_BYTES], KyberLibError>
{
    cipher_bytes=match socket.read(&mut buf).await
    {
        Ok(0) => {
            println!("Client {} disconnected", addr);
            return;
        }
        Ok(n) => {
            let msg = String::from_utf8_lossy(&buf[..n]);
            println!("Received from {}: {}", addr, msg);

        }
        Err(e) => {
            eprintln!("Error reading from client {}: {}", addr, e);
            return;
        }
        
    }
    let ciphertext = Ciphertext::try_from(cipher_bytes.as_slice())?;
    let secret = decapsulate(&ciphertext, &key.secret)?;
    Ok(secret)
}

// Req: Req.016
// Description: 
// This function reads the shared secret  and encrypt the message to be sent by using ChaCha20Poly1305 symmetric encryption algorithm. 
// Date: 
// Author: 
// Pre:  socket handler, shared secret, message to encrypt
// Post: void

fn encrypt_msg(socket:&mut TcpStream ,secret:&SharedSecret,msg:&str)
{   const NONCE_LEN: usize = 12;
    let key = Key::from_slice(secret.as_bytes()); 
    let cipher = ChaCha20Poly1305::new(key);
    let rannonce =&OsRng.gen_random_bytes()[..usize]
    let nonce = Nonce::from_slice(rannonce); // Generate a random nonce
    let ciphertext = cipher.encrypt(nonce, msg.as_bytes())
        .expect("Error!: encryption");
    let mut encrypted_msg = nonce.to_vec();
    encrypted_msg.extend_from_slice(&ciphertext);
    println!("Sending encrypted msg to terminal...");
    if let Err(e) = socket.write_all(encrypted_msg)?; {
        let error_code = match e.kind() {
            io::ErrorKind::BrokenPipe => 1,
            io::ErrorKind::ConnectionReset => 2,
            _ => 99,
        };
        eprintln!("Failed to send data. Error code: {}", error_code);
    } else {
        println!("Data sent successfully!");
    }
}
