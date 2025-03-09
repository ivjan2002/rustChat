use yew::prelude::*;

#[function_component]
fn App()->Html{
    let messages_handle=use_state(Vec::default);
    let messages=(*messages_handle).clone();
    let ws=use_websocket("ws://127.0.0.1:8000".to_string())
    let mut cloned_messages=messages.clone();
    let new_message_handle=use_state(String::default);
    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_nsg)=&**ws_message{
            cloned_messages.push(ws_nsg.clone());
            messages_handle.set(cloned_messages);
        }
    })

    let on_message_change=Callback::from|e::Event|{
        let target=e.target_dyn_into::<HtmlTextAreaElement>()
        if let Some(textarea)=target{
            new_message_handle.set(textarea.value());
        }
    }

    let cloned_new_message_handle=new_message_handle.clone();
    let on_button_click=Callback::from{move | :MouseEvent |{
        cloned_ws.send(cloned_new_message.clone());
        cloned_new_message_handleS.set("".to_string());
    }}
    html!{
        <div class="container">
        <div class="row">
        <ul id="chat">
        {
            message.iter().map(|m| html! {
                <div class="list-group-item list-group-item-action">
                <div class="d-flex w-100 justify-content-between">
                <h5>{n.author.clone()}</h5>
                <small>{n.created_at.format("%Y-%m-%d %H:%M:%S").clone()}</small>
                </div>
                <p>{n.message.clone()}</p>
                </div>
            }).collect::<Html>()
        }
        
        </ul>
        </div>
        <div class="row">
        <div class="input_group">
        <textarea class="form_control" onChange={on_message_change} value={new_message}></textarea>
        <button type="submit" onClick={on_button_click}>Send</button>
        </div>
        </div>
        <div/>
    }
    
}


fn main() {
    yew::Renderer::<App>::render();
}
