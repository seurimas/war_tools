use web_sys::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

use crate::calculate::calculate_weights;

pub const MAX_SOLDIERS: usize = 100;

pub struct WarOdds {
    pub base_chance: f64,
    pub commander_bonus: f64,
    pub blessing_bonus: f64,
    pub fortified_def_bonus: f64,
    pub claimed_def_bonus: f64,
    pub city_def_bonus: f64,
    pub attacker_present: bool,
    pub defender_present: bool,
    pub attacker_blessed: bool,
    pub defender_blessed: bool,
    pub defender_fortified: bool,
    pub attacker_claimed: bool,
    pub defender_claimed: bool,
    pub attacker_city: bool,
    pub defender_city: bool,
}

impl Default for WarOdds {
    fn default() -> Self {
        WarOdds {
            base_chance: 10.,
            commander_bonus: 1.,
            blessing_bonus: 3.,
            fortified_def_bonus: 2.,
            claimed_def_bonus: 1.,
            city_def_bonus: 5.,
            attacker_present: true,
            defender_present: true,
            attacker_blessed: false,
            defender_blessed: false,
            defender_fortified: false,
            attacker_claimed: false,
            defender_claimed: false,
            attacker_city: false,
            defender_city: false,
        }
    }
}

impl WarOdds {
    pub fn get_attacker_rate(&self) -> f64 {
        let mut rate = self.base_chance;
        if self.attacker_present {
            rate += self.commander_bonus;
        }
        if self.attacker_blessed {
            rate += self.blessing_bonus;
        }
        if self.defender_claimed {
            rate -= self.claimed_def_bonus;
        }
        if self.defender_present && self.defender_fortified {
            rate -= self.fortified_def_bonus;
        }
        if self.defender_city {
            rate -= self.city_def_bonus;
        }
        rate / 100.
    }

    pub fn get_defender_rate(&self) -> f64 {
        let mut rate = self.base_chance;
        if self.defender_present {
            rate += self.commander_bonus;
        }
        if self.defender_blessed {
            rate += self.blessing_bonus;
        }
        if self.attacker_claimed {
            rate -= self.claimed_def_bonus;
        }
        if self.attacker_city {
            rate -= self.city_def_bonus;
        }
        rate / 100.
    }
}

pub const WEIGHT_COUNT: usize = (MAX_SOLDIERS + 1) * (MAX_SOLDIERS + 1);
pub type WarWeights = [f64; WEIGHT_COUNT];

pub struct WarModel {
    odds: WarOdds,
    starting_attackers: f64,
    starting_defenders: f64,
    weights: Option<WarWeights>,
}

impl Default for WarModel {
    fn default() -> Self {
        Self {
            odds: WarOdds::default(),
            starting_attackers: 100.0,
            starting_defenders: 100.0,
            weights: None,
        }
    }
}

pub enum Msg {
    UpdateBaseChance(String),
    UpdateAttackerBonus(String),
    UpdateBlessingBonus(String),
    UpdateFortifiedDefBonus(String),
    UpdateClaimedDefBonus(String),
    UpdateCityDefBonus(String),
    UpdateStartingAttackers(String),
    UpdateStartingDefenders(String),
    Calculate,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlDivElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.text_content().into());
    target.inner_text()
}

impl Component for WarModel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let vnode = html! (
            <div>
                <div>
                    <label for="base_chance">{ "Base Chance: " }</label>
                    <input id="base_chance" type="number" value={ self.odds.base_chance.to_string() } oninput={ ctx.link().callback(|e: InputEvent| Msg::UpdateBaseChance(get_value_from_input_event(e))) } />
                </div>
                <div>
                    <label for="attacker_bonus">{ "Attacker Bonus: " }</label>
                    <input id="attacker_bonus" type="number" value={ self.odds.commander_bonus.to_string() } oninput={ ctx.link().callback(|e: InputEvent| Msg::UpdateAttackerBonus(get_value_from_input_event(e))) } />
                </div>
                <div>
                    <label for="blessing_bonus">{ "Blessing Bonus: " }</label>
                    <input id="blessing_bonus" type="number" value={ self.odds.blessing_bonus.to_string() } oninput={ ctx.link().callback(|e: InputEvent| Msg::UpdateBlessingBonus(get_value_from_input_event(e))) } />
                </div>
                <div>
                    <label for="fortified_def_bonus">{ "Fortified Def Bonus: " }</label>
                    <input id="fortified_def_bonus" type="number" value={ self.odds.fortified_def_bonus.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateFortifiedDefBonus(get_value_from_input_event(e))) } />
                </div>
                <div>
                    <label for="claimed_def_bonus">{ "Claimed Def Bonus: " }</label>
                    <input id="claimed_def_bonus" type="number" value={ self.odds.claimed_def_bonus.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateClaimedDefBonus(get_value_from_input_event(e))) } />
                </div>
                <div>
                    <label for="city_def_bonus">{ "City Def Bonus: " }</label>
                    <input id="city_def_bonus" type="number" value={ self.odds.city_def_bonus.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateCityDefBonus(get_value_from_input_event(e))) } />
                </div>
                <div>
                    <label for="starting_attackers">{ "Starting Attackers: " }</label>
                    <input id="starting_attackers" type="number" value={ self.starting_attackers.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateStartingAttackers(get_value_from_input_event(e))) } />
                </div>
                <div>
                    <label for="starting_defenders">{ "Starting Defenders: " }</label>
                    <input id="starting_defenders" type="number" value={ self.starting_defenders.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateStartingDefenders(get_value_from_input_event(e))) } />
                </div>
                <button onclick={ ctx.link().callback(|_| Msg::Calculate) }>{ "Calculate" }</button>
            </div>
        );
        if let Some(weights) = self.weights {
            html!(
                {vnode}
            )
        } else {
            vnode
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBaseChance(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.base_chance = val;
                }
            }
            Msg::UpdateAttackerBonus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.commander_bonus = val;
                }
            }
            Msg::UpdateBlessingBonus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.blessing_bonus = val;
                }
            }
            Msg::UpdateFortifiedDefBonus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.fortified_def_bonus = val;
                }
            }
            Msg::UpdateClaimedDefBonus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.claimed_def_bonus = val;
                }
            }
            Msg::UpdateCityDefBonus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.city_def_bonus = val;
                }
            }
            Msg::UpdateStartingAttackers(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.starting_attackers = val;
                }
            }
            Msg::UpdateStartingDefenders(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.starting_defenders = val;
                }
            }
            Msg::Calculate => {
                self.weights = Some(calculate_weights(self.starting_attackers, self.starting_defenders, &self.odds));
            }
        }
        true
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
