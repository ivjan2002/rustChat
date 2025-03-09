use yew::prelude::*;

#[function_component]
fn App()->Html{
    let messages_handle=use_state(Vec::default);
    let messages=(*messages_handle).clone();
    let ws=use_websocket("ws://127.0.0.1:8000".to_string())
    let mut cloned_messages=messages.clone();
    use_effect_with(ws.message, move |ws_message| {
        if let Some(ws_nsg)=&**ws_message{
            cloned_messages.push(ws_nsg.clone());
            messages_handle.set(cloned_messages);
        }
    })
    html!{
        <ul id="chat">
        {
            message.iter(),map(|m| html! {<li>{m}</li>}).collect::<Html>()
        }
        
        </ul>
    }
    
}


fn main() {
    yew::Renderer::<App>::render();
}
