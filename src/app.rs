use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Estado {
    valor: i32
}

#[function_component(App)]
pub fn app() -> Html {
    let estado = use_state(|| Estado {
        valor: 0,
    });

    let aumentar = {
        let estado = estado.clone();
        
        Callback::from(move |_| {
            estado.set(Estado {
                valor: estado.valor + 1,
            })
        })
    };

    let decrecer = {
        let estado = estado.clone();
        
        Callback::from(move |_| {
            estado.set(Estado {
                valor: estado.valor - 1,
            })
        })
    };


    html! {
        <main>
            <h1>{ "Super contador" }</h1>
            <div class="contadores">
                <button class="items" onclick={decrecer}><h5>{ "-" }</h5></button>
                <Contador valor={ estado.valor } />
                <button class="items" onclick={aumentar}><h5>{ "+" }</h5></button>
            </div>
        </main>
    }
}

#[function_component]
fn Contador(estado: &Estado) -> Html {
    html! { <h4 class="items">{ estado.valor }</h4> }
}