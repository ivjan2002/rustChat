
#[rocket::get("/")]
fn chat(ws:WebSocket)->Channel<'static>{
    ws.channel(move|mut stream|Box::pin(async move
        {while let Some[message]=stream.next.await
            {let_=stream.send(message?).await;
            }Ok(())git
        }))
}


#[rocket::main]
async fn main() {
    let _=rocket::build()
    .mount("/",rocket::routes![
        chat
    ])
    .launch()
    .await;
}
