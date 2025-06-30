/*use solana_sdk::signer::{keypair::Keypair, Signer};
use solana_sdk::pubkey;  //macro key 
use solana_sdk::pubkey::Pubkey; 
#[tokio::main]


//macros are used for while patter matching

/* async fn main() {
    //how we generate the new account 

   let keypair=Keypair::new();
    println!("public ley {}",keypair.pubkey()); 
    print!("Secrt {:?}",keypair.to_bytes()); 
}
*/
//for building pds

async fn main(){
    let program_address=pubkey!("11111111111111111111111111111111");
    let seeds=[b"treasury".as_ref()];
    let (pda,bump)=Pubkey::find_program_address(&seeds, &program_address); 
    println!("PDA {}",pda);
     println!("bump {}",bump) ;

}*/


//lets test the axum framework 
/* 
use axum::{
    routing::get,
    Router
}; 

#[tokio::main]
async fn main(){
      //we use router to see which path goes to which services  
   let app=Router::new().route("/", get(|| async {"hello world"})); 
     
     let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); 

     axum::serve(listener, app).await.unwrap();

}



*/


use tokio::{net::TcpListener, sync::RwLock};

use axum::{http::Method, routing::{get,post}, Router}; 
use tower_http::cors::{Any,CorsLayer};

use instructions::balance::get_balance;
use instructions::transfer::transfer;
mod instructions;
mod model;

#[tokio::main]
async fn main(){
   let cors=CorsLayer::new().
   allow_methods([Method::GET,Method::POST]).allow_headers(Any).allow_origin(Any);

   let app=Router::new().route("/getbalance", get( get_balance ))
   .layer(cors);
   
   
       let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

}

