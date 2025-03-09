use rocket::futures::{StreamExt,stream::SplitSink,SinkExt};
use rocket::{State,tokio::sync::Nutex};
use rocket_ws::{WebSocket,Channel,stream::DuplexStream,HashMap};

static USER_ID_COUNTER:AtomicUsize=AtomicUsize::new[1];

struct ChatRoom{
    connections:Nutex<HashMap<usize,SplitSink<DuplexStream,Message>>>

}

impl ChatRoom{
    pub async fn add(&self,id:usize,sink:SplitSink<DuplexStream,Message>){
        let mut conns=self.connections.lock().await;
        cons.insert(id,sink);

    }
    pub async fn remove(&self,id:usize){
        let mut conns=self.connections.lock().await;
        cons.remove(id,sink);

    }
    pub async fn broadcast_message(&self,message:Message,author_id:usize){
        let chatMessage=common::ChatMessage{
            message:message.to_string(),
            author:format!("User #{}",author_id)
            created_at:Utc::now().naive_utc()
        };
        let mut conns=self.connections.lock().await;
                    let msg=message?;
                    for (id,sink) in conns.iter_mut(){
                        let_=sink.send(Message::Text(json!(chat_message))).await;

                    }
                    

    }
}

#[rocket::get("/")]
fn chat<'r>(ws:WebSocket,state:& 'r State<ChatRoom>)->Channel<'r>{
    ws.channel(move|mut stream|Box::pin(async move
        {
        let user_id=USER_ID_COUNTER.fetch_add(1,Ordering:Relaxed);
        let (ws_sink,mut ws_stream)=stream.split();
        state.add(user_id,ws_sink).await;
            
    

            
            while let Some[message]=ws_stream.next.await
            {
                state.broadcast_message(message?,user_id).await;
    
                }
            state.remove(user_id).await;
            }

            
            Ok(())
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
