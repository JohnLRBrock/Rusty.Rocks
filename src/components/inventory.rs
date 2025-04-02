use leptos::prelude::*;
use reactive_stores::Store;
use crate::app::GameState;
use crate::models::rock_factory;

#[component]
fn RockDisplay(
    rock: rock_factory::Rock,
) -> impl IntoView {
    view! {
        <div class="rock-item">
            <h3>{"Rock"}</h3>
            <ul>
                <li>{"Type: "}{format!("{:?}", rock.rock_type)}</li>
                <li>{"Color: "}{format!("{:?}", rock.color)}</li>
                <li>{"Size: "}{format!("{:.1}", rock.size)}</li>
                <li>{"Quality: "}{format!("{:.1}", rock.quality)}</li>
                <li>{"Hardness: "}{format!("{:.1}", rock.hardness)}</li>
                <li>{"Luster: "}{format!("{:?}", rock.lusters)}</li>
                <li>{"Transparency: "}{format!("{:?}", rock.transparency)}</li>
                {move || {
                    rock.optical_phenomena.as_ref().map(|phenomena| {
                        view! {
                            <li>{"Optical Phenomena: "}{format!("{:?}", phenomena)}</li>
                        }
                    })
                }}
                <li>{"Value: "}{format!("{:.2}", rock.value)}</li>
            </ul>
        </div>
    }
}

#[component]
pub fn Inventory() -> impl IntoView {
    let store = use_context::<Store<GameState>>()
        .expect("Store should be provided by App component");

    view! {
        <div class="inventory">
            <div class="inventory-count">
                {move || {
                    let state = store.get();
                    format!("Rocks: {} / {}", state.rocks.len(), state.inventory_size)
                }}
            </div>
            <div class="rocks-list">
                {move || {
                    let state = store.get();
                    state.rocks.iter().rev().map(|rock| {
                        view! {
                            <RockDisplay
                                rock=rock.clone()
                            />
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}