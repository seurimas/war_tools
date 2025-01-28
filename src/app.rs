use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::*;
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
    pub archer_attack_malus: f64,
    pub archer_defense_malus: f64,
    pub elite_attack_bonus: f64,
    pub elite_defense_bonus: f64,
    pub attacker_present: bool,
    pub defender_present: bool,
    pub attacker_blessed: bool,
    pub defender_blessed: bool,
    pub defender_fortified: bool,
    pub attacker_claimed: bool,
    pub defender_claimed: bool,
    pub attacker_city: bool,
    pub defender_city: bool,
    pub attacker_archers: bool,
    pub defender_archers: bool,
    pub attacker_elites: bool,
    pub defender_elites: bool,
    pub round_count: usize,
}

impl Default for WarOdds {
    fn default() -> Self {
        WarOdds {
            base_chance: 10.,
            commander_bonus: 1.,
            blessing_bonus: 2.,
            fortified_def_bonus: 1.,
            claimed_def_bonus: 1.,
            city_def_bonus: 2.,
            attacker_present: true,
            defender_present: false,
            attacker_blessed: true,
            defender_blessed: true,
            defender_fortified: false,
            attacker_claimed: false,
            defender_claimed: false,
            attacker_city: false,
            defender_city: false,
            attacker_archers: false,
            defender_archers: false,
            attacker_elites: false,
            defender_elites: false,
            archer_attack_malus: 1.,
            archer_defense_malus: 1.,
            elite_attack_bonus: 1.,
            elite_defense_bonus: 1.,
            round_count: 20,
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
        if self.defender_archers {
            rate += self.archer_defense_malus;
        }
        if self.attacker_archers {
            rate -= self.archer_attack_malus;
        }
        if self.attacker_elites {
            rate += self.elite_attack_bonus;
        }
        if self.defender_elites {
            rate -= self.elite_defense_bonus;
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
        if self.attacker_archers {
            rate += self.archer_defense_malus;
        }
        if self.defender_archers {
            rate -= self.archer_attack_malus;
        }
        if self.defender_elites {
            rate += self.elite_attack_bonus;
        }
        if self.attacker_elites {
            rate -= self.elite_defense_bonus;
        }
        rate / 100.
    }
}

pub const WEIGHT_COUNT: usize = (MAX_SOLDIERS + 1) * (MAX_SOLDIERS + 1);

#[derive(Debug)]
pub struct WarWeights(pub [f64; WEIGHT_COUNT]);

impl Default for WarWeights {
    fn default() -> Self {
        Self([0.; WEIGHT_COUNT])
    }
}

impl WarWeights {
    pub fn slot_for(attackers: usize, defenders: usize) -> usize {
        attackers * (MAX_SOLDIERS + 1) + defenders
    }

    pub fn get_attackers_winning_results(&self) -> [f64; MAX_SOLDIERS + 1] {
        let mut results = [0.; MAX_SOLDIERS + 1];
        for j in 0..=MAX_SOLDIERS {
            results[j] = self.0[Self::slot_for(j, 0)];
        }
        results
    }

    pub fn get_defenders_winning_results(&self) -> [f64; MAX_SOLDIERS + 1] {
        let mut results = [0.; MAX_SOLDIERS + 1];
        for i in 0..=MAX_SOLDIERS {
            results[i] = self.0[Self::slot_for(0, i)];
        }
        results
    }

    pub fn get_odds_of_no_win(&self) -> f64 {
        let mut odds = 0.;
        for i in 1..=MAX_SOLDIERS {
            for j in 1..=MAX_SOLDIERS {
                odds += self.0[Self::slot_for(i, j)];
            }
        }
        odds
    }
}

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
    UpdateArcherAttackMalus(String),
    UpdateArcherDefenseMalus(String),
    UpdateEliteAttackBonus(String),
    UpdateEliteDefenseBonus(String),
    UpdateStartingAttackers(String),
    UpdateStartingDefenders(String),
    UpdateRoundCount(String),
    ToggleAttackerPresent,
    ToggleDefenderPresent,
    ToggleAttackerBlessed,
    ToggleDefenderBlessed,
    ToggleDefenderFortified,
    ToggleAttackerClaimed,
    ToggleDefenderClaimed,
    ToggleAttackerCity,
    ToggleDefenderCity,
    ToggleAttackerArchers,
    ToggleDefenderArchers,
    ToggleAttackerElites,
    ToggleDefenderElites,
    Calculate,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    web_sys::console::log_1(&event.clone().into());
    let event_target = event.target().unwrap_throw();
    web_sys::console::log_1(&event_target.clone().into());
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

impl Component for WarModel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(weights) = &self.weights {
            html!(
                <div id="with_results">
                    {self.get_results_node(ctx)}
                    {self.get_settings_node(ctx)}
                </div>
            )
        } else {
            html!(<div id="without_results">{self.get_settings_node(ctx)}</div>)
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
            Msg::UpdateArcherAttackMalus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.archer_attack_malus = val;
                }
            }
            Msg::UpdateArcherDefenseMalus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.archer_defense_malus = val;
                }
            }
            Msg::UpdateEliteAttackBonus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.elite_attack_bonus = val;
                }
            }
            Msg::UpdateEliteDefenseBonus(val) => {
                if let Ok(val) = val.parse::<f64>() {
                    self.odds.elite_defense_bonus = val;
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
            Msg::UpdateRoundCount(val) => {
                if let Ok(val) = val.parse::<usize>() {
                    self.odds.round_count = val;
                }
            }
            Msg::ToggleAttackerPresent => {
                self.odds.attacker_present = !self.odds.attacker_present;
            }
            Msg::ToggleDefenderPresent => {
                self.odds.defender_present = !self.odds.defender_present;
            }
            Msg::ToggleAttackerBlessed => {
                self.odds.attacker_blessed = !self.odds.attacker_blessed;
            }
            Msg::ToggleDefenderBlessed => {
                self.odds.defender_blessed = !self.odds.defender_blessed;
            }
            Msg::ToggleAttackerClaimed => {
                self.odds.attacker_claimed = !self.odds.attacker_claimed;
            }
            Msg::ToggleDefenderClaimed => {
                self.odds.defender_claimed = !self.odds.defender_claimed;
            }
            Msg::ToggleDefenderFortified => {
                self.odds.defender_fortified = !self.odds.defender_fortified;
            }
            Msg::ToggleAttackerCity => {
                self.odds.attacker_city = !self.odds.attacker_city;
            }
            Msg::ToggleDefenderCity => {
                self.odds.defender_city = !self.odds.defender_city;
            }
            Msg::ToggleAttackerArchers => {
                self.odds.attacker_archers = !self.odds.attacker_archers;
                if self.odds.attacker_archers {
                    self.odds.attacker_elites = false;
                }
            }
            Msg::ToggleDefenderArchers => {
                self.odds.defender_archers = !self.odds.defender_archers;
                if self.odds.defender_archers {
                    self.odds.defender_elites = false;
                }
            }
            Msg::ToggleAttackerElites => {
                self.odds.attacker_elites = !self.odds.attacker_elites;
                if self.odds.attacker_elites {
                    self.odds.attacker_archers = false;
                }
            }
            Msg::ToggleDefenderElites => {
                self.odds.defender_elites = !self.odds.defender_elites;
                if self.odds.defender_elites {
                    self.odds.defender_archers = false;
                }
            }
            Msg::Calculate => {
                self.weights = Some(calculate_weights(
                    self.starting_attackers,
                    self.starting_defenders,
                    &self.odds,
                ));
            }
        }
        true
    }
}

impl WarModel {
    fn get_results_table_node(
        &self,
        ctx: &Context<WarModel>,
        results: [f64; MAX_SOLDIERS + 1],
    ) -> yew::virtual_dom::VNode {
        let mut minimum = results
            .iter()
            .enumerate()
            .find(|(_, r)| **r > 0.01)
            .map(|(i, _)| i)
            .unwrap_or(0);
        let mut maximum = results
            .iter()
            .enumerate()
            .rev()
            .find(|(_, r)| **r > 0.01)
            .map(|(i, _)| i)
            .unwrap_or(0);
        let average = results
            .iter()
            .enumerate()
            .map(|(i, r)| i as f64 * r)
            .sum::<f64>()
            / results.iter().sum::<f64>();
        let total_chance = results.iter().sum::<f64>();
        let median = results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                results
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j <= i)
                    .map(|(_, r)| r)
                    .sum::<f64>()
            })
            .enumerate()
            .find(|(_, r)| *r >= total_chance / 2.)
            .map(|(i, _)| i)
            .unwrap_or(0) as f64;
        let probable_result = results
            .iter()
            .enumerate()
            .max_by_key(|(_, r)| (**r * 10000.) as usize)
            .map(|(i, _)| i)
            .unwrap_or(0);
        if minimum == 0 && maximum == 0 {
            if results[probable_result] < 0.0001 {
                return html!(<div class="no_results">{"No victory possible"}</div>);
            }
            minimum = probable_result.max(10) - 10;
            maximum = (probable_result + 10).min(MAX_SOLDIERS);
        }
        html!(
            <table class={format!("probable_{} average_{:.0} median_{:0}", probable_result, average.round(), median.round())}>
                <thead>
                    <tr>
                      <th class="total">{"Total"}</th>
                        {for (minimum..=maximum).map(|i| {
                            let r = results[i];
                            html!(<th class={format!("result_{} odds_{:.0}", i, r * 10000.)}>{i}</th>)
                        })}
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="total">{format!("{:.2}%", total_chance * 100.0)}</td>
                        {for (minimum..=maximum).map(|i| {
                            let r = results[i];
                            html!(<td class={format!("result_{} odds_{:.0}", i, r * 10000.)}>{format!("{:.2}%", r * 100.0)}</td>)
                        })}
                    </tr>
                </tbody>
            </table>
        )
    }
    fn get_results_node(&self, ctx: &Context<WarModel>) -> yew::virtual_dom::VNode {
        if let Some(weights) = &self.weights {
            let attacker_results = weights.get_attackers_winning_results();
            let defender_results = weights.get_defenders_winning_results();
            html!(
                <div id="results">
                    <div id="attacker_results">
                        <h2>{ "Attacker Results" }</h2>
                        {self.get_results_table_node(ctx, attacker_results)}
                    </div>
                    <div id="incomplete">
                        <h2>{ format!("No win ({} rounds)", self.odds.round_count) }</h2>
                        <span>{ format!("{:.2}%", weights.get_odds_of_no_win() * 100.0) }</span>
                    </div>
                    <div id="defender_results">
                        <h2>{ "Defender Results" }</h2>
                        {self.get_results_table_node(ctx, defender_results)}
                    </div>
                </div>
            )
        } else {
            html!()
        }
    }
    fn get_settings_node(&self, ctx: &Context<WarModel>) -> yew::virtual_dom::VNode {
        let vnode = html! (
            <div id="odds_settings">
                <div id="numbers">
                    <div>
                        <label for="base_chance">{ "Base Chance: " }</label>
                        <input id="base_chance" type="number" value={ self.odds.base_chance.to_string() } oninput={ ctx.link().callback(|e: InputEvent| Msg::UpdateBaseChance(get_value_from_input_event(e))) } />
                    </div>
                    <div>
                        <label for="attacker_bonus">{ "Commander Attack Bonus: " }</label>
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
                        <label for="archer_attack_malus">{ "Archer Attack Malus: " }</label>
                        <input id="archer_attack_malus" type="number" value={ self.odds.archer_attack_malus.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateArcherAttackMalus(get_value_from_input_event(e))) } />
                    </div>
                    <div>
                        <label for="archer_defense_malus">{ "Archer Defense Malus: " }</label>
                        <input id="archer_def_malus" type="number" value={ self.odds.archer_defense_malus.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateArcherDefenseMalus(get_value_from_input_event(e))) } />
                    </div>
                    <div>
                        <label for="elite_attack_bonus">{ "Elite Attack Bonus: " }</label>
                        <input id="elite_attack_bonus" type="number" value={ self.odds.elite_attack_bonus.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateEliteAttackBonus(get_value_from_input_event(e))) } />
                    </div>
                    <div>
                        <label for="elite_defense_bonus">{ "Elite Defense Bonus: " }</label>
                        <input id="elite_defense_bonus" type="number" value={ self.odds.elite_defense_bonus.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateEliteDefenseBonus(get_value_from_input_event(e))) } />
                    </div>
                </div>
                <div id="attackers">
                    <div>
                        <label for="starting_attackers">{ "Starting Attackers: " }</label>
                        <input id="starting_attackers" type="number" value={ self.starting_attackers.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateStartingAttackers(get_value_from_input_event(e))) } />
                    </div>
                    <div>
                        <label for="attacker_present">{ "Attacker Commander Present: " }</label>
                        <input id="attacker_present" type="checkbox" checked={ self.odds.attacker_present } onclick={ ctx.link().callback(|_| Msg::ToggleAttackerPresent) } />
                    </div>
                    <div>
                        <label for="attacker_blessed">{ "Attacker Blessed: " }</label>
                        <input id="attacker_blessed" type="checkbox" checked={ self.odds.attacker_blessed } onclick={ ctx.link().callback(|_| Msg::ToggleAttackerBlessed) } />
                    </div>
                    <div>
                        <label for="attacker_claimed">{ "Attacker Claimed: " }</label>
                        <input id="attacker_claimed" type="checkbox" checked={ self.odds.attacker_claimed } onclick={ ctx.link().callback(|_| Msg::ToggleAttackerClaimed) } />
                    </div>
                    <div>
                        <label for="attacker_city">{ "Attacker City: " }</label>
                        <input id="attacker_city" type="checkbox" checked={ self.odds.attacker_city } onclick={ ctx.link().callback(|_| Msg::ToggleAttackerCity) } />
                    </div>
                    <div>
                        <label for="attacker_archers">{ "Attacker are Archers: " }</label>
                        <input id="attacker_archers" type="checkbox" checked={ self.odds.attacker_archers } onclick={ ctx.link().callback(|_| Msg::ToggleAttackerArchers) } />
                    </div>
                    <div>
                        <label for="attacker_elites">{ "Attacker are Elites: " }</label>
                        <input id="attacker_elites" type="checkbox" checked={ self.odds.attacker_elites } onclick={ ctx.link().callback(|_| Msg::ToggleAttackerElites) } />
                    </div>
                </div>
                <div id="defenders">
                    <div>
                        <label for="starting_defenders">{ "Starting Defenders: " }</label>
                        <input id="starting_defenders" type="number" value={ self.starting_defenders.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateStartingDefenders(get_value_from_input_event(e))) } />
                    </div>
                    <div>
                        <label for="defender_present">{ "Defender Commander Present: " }</label>
                        <input id="defender_present" type="checkbox" checked={ self.odds.defender_present } onclick={ ctx.link().callback(|_| Msg::ToggleDefenderPresent) } />
                    </div>
                    <div>
                        <label for="defender_blessed">{ "Defender Blessed: " }</label>
                        <input id="defender_blessed" type="checkbox" checked={ self.odds.defender_blessed } onclick={ ctx.link().callback(|_| Msg::ToggleDefenderBlessed) } />
                    </div>
                    <div>
                        <label for="defender_claimed">{ "Defender Claimed: " }</label>
                        <input id="defender_claimed" type="checkbox" checked={ self.odds.defender_claimed } onclick={ ctx.link().callback(|_| Msg::ToggleDefenderClaimed) } />
                    </div>
                    <div>
                        <label for="defender_fortified">{ "Defender Fortified: " }</label>
                        <input id="defender_fortified" type="checkbox" checked={ self.odds.defender_fortified } onclick={ ctx.link().callback(|_| Msg::ToggleDefenderFortified) } />
                    </div>
                    <div>
                        <label for="defender_city">{ "Defender City: " }</label>
                        <input id="defender_city" type="checkbox" checked={ self.odds.defender_city } onclick={ ctx.link().callback(|_| Msg::ToggleDefenderCity) } />
                    </div>
                    <div>
                        <label for="defender_archers">{ "Defender Archers: " }</label>
                        <input id="defender_archers" type="checkbox" checked={ self.odds.defender_archers } onclick={ ctx.link().callback(|_| Msg::ToggleDefenderArchers) } />
                    </div>
                    <div>
                        <label for="defender_elites">{ "Defender Elites: " }</label>
                        <input id="defender_elites" type="checkbox" checked={ self.odds.defender_elites } onclick={ ctx.link().callback(|_| Msg::ToggleDefenderElites) } />
                    </div>
                </div>
                <div id="calculate">
                    <div>
                        <label for="round_count">{ "Round Count: " }</label>
                        <input id="round_count" type="number" value={ self.odds.round_count.to_string() } oninput={ ctx.link().callback(|e| Msg::UpdateRoundCount(get_value_from_input_event(e))) } />
                    </div>
                    <button onclick={ ctx.link().callback(|_| Msg::Calculate) }>{ "Calculate" }</button>
                </div>
            </div>
        );
        vnode
    }
}
