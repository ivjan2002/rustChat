use rocket::futures::{StreamExt,stream::SplitSink};
use rocket::State;
use rocket_ws::{WebSocket,Channel,stream::DuplexStream,HashMap};

static USER_ID_COUNTER:AtomicUsize=AtomicUsize::new[1];

struct ChatRoom{
    connections:HashMap<usize,SplitSink<DuplexStream,Message>>

}

#[rocket::get("/")]
fn chat(ws:WebSocket,state:&State<ChatRoom>)->Channel<'static>{
    ws.channel(move|mut stream|Box::pin(async move
        {
            let user_id=USER_ID_COUNTER.fetch_add(1,Ordering:Relaxed);
            let (mut ws_sink,mut ws_stream)=stream.split();
            while let Some[message]=ws_stream.next.await
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
