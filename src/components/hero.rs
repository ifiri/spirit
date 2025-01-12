use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
      div {
        class: "container",

        div {
          class: "wrapper",

          div {
            class: "header",

            div {
              class: "name", "Hyeon",
            }

            div {
              "CryptoBoar",
            }

            div {
              class: "wallet", "8342hd...109234",
            }
          }
        
          div {
            class: "center",

            div {
              class: "character",
            }
          }

          div {
            class: "footer btn-group",

            button {
              class: "btn btn-primary", "Tasks",
            }
            button {
              class: "btn btn-primary", "Airdrop",
            }
            button {
              class: "btn btn-primary", "Boost",
            }
            button {
              class: "btn btn-primary", "Market",
            }
            button {
              class: "btn btn-primary", "Earn",
            }
          }
        }
      }
    }
}
