mod error;
mod path;

use std::any::TypeId;
use std::str::FromStr;
use std::usize;

use dioxus::desktop::{
    Config as DesktopConfig,
    WindowBuilder,
};
use dioxus::prelude::*;
use wf_stats::{
    Secondary,
    Status,
};

use crate::error::Error;
use crate::path::app_data_local_dir;

const APP_NAME: &str = "Warframe Builder";
const APP_IDENTIFIER: &str = "app.warframe-builder";

fn main() -> Result<(), Error> {
    let data_directory = app_data_local_dir()?;

    let window = WindowBuilder::default().with_title(APP_NAME);
    let config = DesktopConfig::default()
        .with_data_directory(data_directory)
        .with_menu(None)
        .with_window(window);

    LaunchBuilder::new().with_cfg(config).launch(App);

    Ok(())
}

#[component]
fn App() -> Element {
    let mut stats: Signal<Secondary> = use_signal(|| Default::default());

    let critical_chance: Signal<f32> = use_signal(|| Default::default());
    let critical_damage: Signal<f32> = use_signal(|| Default::default());
    let status_chance: Signal<f32> = use_signal(|| Default::default());
    let fire_rate: Signal<f32> = use_signal(|| Default::default());
    let multishot: Signal<f32> = use_signal(|| Default::default());
    let ammo_maximum: Signal<usize> = use_signal(|| Default::default());
    let magazine_size: Signal<usize> = use_signal(|| Default::default());
    let reload_time: Signal<f32> = use_signal(|| Default::default());
    let reload_delay: Signal<f32> = use_signal(|| Default::default());

    let mut status_list: Signal<Vec<Status>> = use_signal(|| Default::default());

    use_effect(move || {
        stats.set(Secondary::new(
            *critical_chance.read(),
            *critical_damage.read(),
            *status_chance.read(),
            *fire_rate.read(),
            *multishot.read(),
            *ammo_maximum.read(),
            *magazine_size.read(),
            *reload_time.read(),
            *reload_delay.read(),
            status_list.read().clone(),
        ));
    });

    rsx! {
        div {
            display: "flex",
            div {
                label { "Critical Chance" }
                Input { value: critical_chance }
                br {}
                label { "Critical Damage" }
                Input { value: critical_damage }
                br {}
                label { "Status Chance" }
                Input { value: status_chance }
                br {}
                label { "Fire Rate" }
                Input { value: fire_rate }
                br {}
                label { "Multishot" }
                Input { value: multishot }
                br {}
                label { "Ammo Maximum" }
                Input { value: ammo_maximum }
                br {}
                label { "Magazine Size" }
                Input { value: magazine_size }
                br {}
                label { "Reload Time" }
                Input { value: reload_time }
                br {}
                label { "Reload Delay" }
                Input { value: reload_delay }
                br {}
                label { "Status List" }
                button { onclick: move |_| status_list.push(Status::impact(0.0)), "Add Status" }
                ul {
                    for (i , status) in status_list.read().iter().cloned().enumerate() {
                        StatusListItem { list: status_list, index: i, item: status }
                    }
                }
            }
            pre { "{stats:#?}" }
        }
    }
}

#[component]
fn Input<T: FromStr + Default + 'static>(mut value: Signal<T>) -> Element {
    rsx! {
        input {
            oninput: move |event| value.set(event.value().parse().unwrap_or_default()),
            r#type: "number",
            placeholder: if TypeId::of::<T>() == TypeId::of::<f32>() { "0.00" } else if TypeId::of::<T>() == TypeId::of::<usize>() { "0" },
            step: if TypeId::of::<T>() == TypeId::of::<f32>() { "0.01" } else if TypeId::of::<T>() == TypeId::of::<usize>() { "1" },
            min: if TypeId::of::<T>() == TypeId::of::<f32>() { "-1000.0" } else if TypeId::of::<T>() == TypeId::of::<usize>() { "0" },
            max: if TypeId::of::<T>() == TypeId::of::<f32>() { "1000.0" } else if TypeId::of::<T>() == TypeId::of::<usize>() { "{usize::MAX}" },
        }
    }
}

#[component]
fn StatusListItem(
    list: Signal<Vec<Status>>,
    index: usize,
    item: Status,
) -> Element {
    rsx! {
        li {
            select {
                oninput: move |event| {
                    let status_type: String = event.value();
                    let status_damage: f32 = item.damage();
                    let status = match status_type.as_str() {
                        "impact" => Status::impact(status_damage),
                        "puncture" => Status::puncture(status_damage),
                        "slash" => Status::slash(status_damage),
                        "cold" => Status::cold(status_damage),
                        "electricity" => Status::electricity(status_damage),
                        "heat" => Status::heat(status_damage),
                        "toxin" => Status::toxin(status_damage),
                        "blast" => Status::blast(status_damage),
                        "corrosive" => Status::corrosive(status_damage),
                        "gas" => Status::gas(status_damage),
                        "magnetic" => Status::magnetic(status_damage),
                        "radiation" => Status::radiation(status_damage),
                        "viral" => Status::viral(status_damage),
                        _ => return,
                    };
                    list.write()[index] = status;
                },
                option { disabled: true, "Physical" }
                option { selected: item.is_impact(), value: "impact", "Impact" }
                option { selected: item.is_puncture(), value: "puncture", "Puncture" }
                option { selected: item.is_slash(), value: "slash", "Slash" }
                option { disabled: true, "Primary Elemental" }
                option { selected: item.is_cold(), value: "cold", "Cold" }
                option { selected: item.is_electricity(), value: "electricity", "Electricity" }
                option { selected: item.is_heat(), value: "heat", "Heat" }
                option { selected: item.is_toxin(), value: "toxin", "Toxin" }
                option { disabled: true, "Secondary Elemental" }
                option { selected: item.is_blast(), value: "blast", "Blast" }
                option { selected: item.is_corrosive(), value: "corrosive", "Corrosive" }
                option { selected: item.is_gas(), value: "gas", "Gas" }
                option { selected: item.is_magnetic(), value: "magnetic", "Magnetic" }
                option { selected: item.is_radiation(), value: "radiation", "Radiation" }
                option { selected: item.is_viral(), value: "viral", "Viral" }
            }
            input {
                onchange: move |event| {
                    let status_damage: f32 = event.value().parse().unwrap_or_default();
                    item.set_damage(status_damage);
                    list.write()[index] = item;
                },
                r#type: "number",
                placeholder: "0.00",
                step: "0.01",
            }
            button {
                onclick: move |_| {
                    list.remove(index);
                },
                r#type: "button",
                "Delete"
            }
        }
    }
}
