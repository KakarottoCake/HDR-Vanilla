use std::{
    fmt::format,
    fs::*,
    path::{ Path, PathBuf },
    sync::Mutex
};
use skyline::hooks::InlineCtx;
use smash2::{
    cpp::Vector, 
    phx::hash40
};
use rand::{
    prelude::SliceRandom,
    Rng
};
use toml::Value;

lazy_static! {
    static ref CHARA_WHITELIST: Mutex<Vec<String>> = {
        let mut m = Vec::new();
        Mutex::new(m)
    };
    static ref CHARA_BLACKLIST: Mutex<Vec<String>> = {
        let mut m = Vec::new();
        Mutex::new(m)
    };
    static ref COSTUME_RNG: Mutex<Vec<i32>> = {
        let mut m = Vec::new();
        Mutex::new(m)
    };
}

static PT_CHARA_HASHES: &[u64] = &[
    hash40("ui_chara_pzenigame").0,
    hash40("ui_chara_plizardon").0,
    hash40("ui_chara_pfushigisou").0,
];

static mut PLAYER_TAG_INDEX: &'static mut [u8] = &mut [0; 8];

static mut LAST_FIGHTER_FOUND: u64 = 0x0;
static mut LAST_FIGHTER2_FOUND: u64 = 0x0;

static mut WAS_RANDOM_SELECTION: bool = false;
static mut WAS_RANDOM: bool = false;

const HASH_MASK: u64 = 0xFF_FFFFFFFF;
const KEY_MASK: u64 = 0xFFFFFF_0000000000;

const ORDER_TOML: &str = "ui/param/menu/chara_icon_order.toml";
const RANDOM_IDX_TOML: &str = "ui/param/menu/chara_random_idx.toml";
const RANDOM_CFG_TOML: &str = "ui/param/menu/chara_random_config.toml";

// transmutes the supplied chara string into data for the CSS icon vector
fn ui_chara(i: usize, chara: &str) -> u64 {
    (0xc1u64 << 56)
    | (((i as u64) & 0xFFFF) << 40)
    | hash40(&format!("ui_chara_{}", chara)).0
}

fn is_random(entry: u64) -> bool {
    (entry & HASH_MASK) == hash40("ui_chara_random").0
}

fn key(entry: u64) -> u64 {
    entry & KEY_MASK
}

unsafe fn get_tag_from_save(idx: u8) -> String {
    let tag_address =
        (***(((*((*((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
            .add(0x5314510) as *const u64)) as *const u64))
            + 0x58) as *const *const *const u64)
            + ((idx as u64) * 0xF7D8)
            + 0xC) as *const u16;

    let mut tag_length = 0;
    while *tag_address.add(tag_length) != 0 {
        tag_length += 1;
    }

    String::from_utf16_lossy(std::slice::from_raw_parts(tag_address, tag_length))
}

unsafe fn decide_fighter_from_id(id: usize) -> String {
    // get player tag
    let tag_index = PLAYER_TAG_INDEX[id];
    let tag = &get_tag_from_save(tag_index);
    // println!("Tag data for slot {}: {}", id, &tag); 

    // get the base whitelist from the initial CSS load
    let mut whitelist = CHARA_WHITELIST.lock().unwrap().clone();
    // the following entries are present for CSS loading, but should never be chosen by random
    let forbidden: [&str; 4] = [
        "miifighter",
        "miiswordsman",
        "miigunner",
        "light_first"
    ];
    for chara in forbidden {
        whitelist.retain(|x| *x != chara.to_owned());
    }

    // choose a fighter from base whitelist
    let mut chara_string =  whitelist.choose(&mut rand::thread_rng()).unwrap().as_str();
    let default = chara_string.to_owned();
    println!("Randomly decided on {chara_string}");

    // Collect all relevant data from config TOML
    let path = Path::new("mods:/").join(RANDOM_CFG_TOML);
    if !path.exists() || !path.is_file() { return default };
    let data = match std::fs::read_to_string(&path).unwrap().parse::<Value>() {
        Ok(result) => result,
        Err(_) => { 
            println!("[src::chara_select] Invalid random_config TOML data!");
            return default; 
        }
    };
    let tag_data = match data.as_table().unwrap().get(tag) {
        Some(table) => table.as_table().unwrap(),
        None => {
            println!("[src::chara_select] File does not contain tag [{tag}]! Using global settings.");
            match data.as_table().unwrap().get("global_settings") {
                Some(table) => table.as_table().unwrap(),
                None => {
                    println!("Just kidding... no global settings are present!");
                    return default;
                }
            }
        }
    };
    let kind = match tag_data.get("kind") {
        Some(value) => value.as_str().unwrap(),
        None => {
            println!("[src::chara_select] Tag data does not contain key [kind]!");
            return default;
        }
    };
    if !["whitelist", "blacklist"].contains(&kind) { return default };
    let list = match tag_data.get("list") {
        Some(value) => value.as_array().unwrap(),
        None => {
            println!("[src::chara_select] Tag data does not contain key [list]!");
            return default;
        }
    };

    // return the original choice if it aligns with the data
    if (kind == "whitelist" && list.contains(&toml::Value::String(default.clone())))
    || (kind == "blacklist" && !list.contains(&toml::Value::String(default.clone()))) {
        println!("{chara_string} is allowed!");
        return default;
    }

    // getting to this point means the original choice isn't permitted, so we will adjust the selection
    println!("{chara_string} is not allowed for tag {}! Adjusting...", &tag);
    if kind == "whitelist" {
        // for whitelists, clear and rebuild with the allowed characters
        let reference = whitelist.clone();
        whitelist.clear();
        for chara in list {
            let fighter = chara.as_str().unwrap_or("mario");
            if reference.contains(&fighter.to_owned()) {
                whitelist.push(fighter.to_owned());
            }
        }
        // if none of the whitelisted characters were allowed originally, return the default
        if whitelist.len() <= 0 {
            println!("None of the whitelisted characters are allowed! Continuing with the default.");
            return default;
        }
    } else {
        // for blacklists, remove listed fighters from the selection
        for chara in list {
            let fighter = chara.as_str().unwrap_or("");
            whitelist.retain(|x| *x != fighter.to_owned());
        }
    }

    // re-query with adjusted list
    chara_string =  whitelist.choose(&mut rand::thread_rng()).unwrap().as_str();
    println!("Randomly decided on {chara_string}");

    return chara_string.to_owned();
}

#[skyline::hook(offset = 0x19eb840, inline)]
pub unsafe fn display_css_hook(ctx: &InlineCtx) {
    // populate costume slot rng
    let mut costume_rng = COSTUME_RNG.lock().unwrap();
    costume_rng.clear();
    for i in 0..8 {
        costume_rng.push(i);
    }

    // obtains the original vector of fighter entries to be loaded
    let chara_vec = &mut *(*ctx.registers[4].x.as_ref() as *mut smash2::cpp::Vector<u64>);

    // collect data from TOML
    let path = Path::new("mods:/").join(ORDER_TOML);
    if !path.exists() || !path.is_file() { return };
    let data = match std::fs::read_to_string(&path).unwrap().parse::<Value>() {
        Ok(result) => result,
        Err(_) => { 
            println!("[src::chara_select] Invalid TOML data!");
            return; 
        }
    };
    let config = match data.as_table().unwrap().get("config") {
        Some(table) => table.as_table().unwrap(),
        None => {
            println!("[src::chara_select] File does not contain header [config]!");
            return;
        }
    };
    let enabled = match config.get("enabled") {
        Some(value) => value.as_bool().unwrap(),
        None => {
            println!("[src::chara_select] Config does not contain key [enabled]!");
            return;
        }
    };
    if !enabled { return }; // aborts operation if the config set this to false
    let order = match config.get("order") {
        Some(value) => value.as_str().unwrap(),
        None => {
            println!("[src::chara_select] Config does not contain key [order]!");
            return;
        }
    };
    let schema = match data.as_table().unwrap().get(order) {
        Some(table) => table.as_table().unwrap(),
        None => {
            println!("[src::chara_select] TOML does not contain header [schema]!");
            return;
        }
    };
    if !schema.contains_key("order") || !schema.contains_key("centered_random") {
        println!("[src::chara_select] Invalid schema format!");
        return;
    }
    let chara_order = schema.get("order").unwrap().as_array().unwrap();
    let center_random = schema.get("centered_random").unwrap().as_bool().unwrap_or(true);

    let mut chara_whitelist = CHARA_WHITELIST.lock().unwrap();
    let mut chara_blacklist = CHARA_BLACKLIST.lock().unwrap();
    chara_whitelist.clear();
    chara_blacklist.clear();

    for idx in 0..chara_order.len() {
        let mut chara = chara_order[idx].as_str().unwrap_or("goku");
        if chara == "element" { chara = "flame_first" };
        // check if this fighter is supposed to be loaded
        let mut should_load = false;
        for i in 0..chara_vec.len() {
            let raw = format!("{:#x}", chara_vec.get(i).unwrap());
            let hash = u64::from_str_radix(&raw[8..], 16).unwrap_or(0); // ui_chara index hash in u64
            if hash == hash40(&format!("ui_chara_{}", chara)).0 {
                should_load = true;
                break;
            }
        }
        if should_load {
            // println!("Fighter {chara} will be loaded");
            if chara == "flame_first" { chara = "element" };
            chara_whitelist.push(chara.to_string());
        } else {
            // println!("Fighter {chara} will not be loaded");
            chara_blacklist.push(chara.to_string());
        }
    }

    let r_offset = center_random as i32 as usize;
    let mut icon_count = chara_order.len() + r_offset; // +1 to the order if random is to be inserted
    icon_count -= chara_blacklist.len(); // subtract blacklisted fighters from the total

    // determine where the random icon should be placed in the order
    let mut random_idx = (icon_count / 2) as usize; // default placement
    let r_path = Path::new("mods:/").join(RANDOM_IDX_TOML);
    if r_path.exists() && r_path.is_file() {
        let r_data = match std::fs::read_to_string(&r_path).unwrap().parse::<Value>() {
            Ok(result) => result,
            Err(_) => { return; }
        };
        let placement = match r_data.as_table().unwrap().get("placement") {
            Some(table) => table.as_table().unwrap(),
            None => { return; }
        };
        match placement.get(&icon_count.to_string()) {
            Some(value) => { random_idx = value.as_integer().unwrap() as usize },
            None => {}
        };
    }
    // println!("{icon_count} icons to load ({} out of {} blacklisted). Random will be placed in slot {random_idx}", chara_blacklist.len(), chara_order.len() + center_random as i32 as usize);

    // construct the new order
    let new_order: &mut Vec<u64> = &mut Vec::new();
    let mut push = false; // whether to shift to compensate the inserted random entry
    let mut idx: usize = 0; // handler for the custom order. this is needed to crosscheck skipped entries for things like smashdown
    for i in 0..(icon_count - r_offset) {
        if i == random_idx && center_random == true {
            new_order.push(ui_chara(i, "random"));
            // println!("{} / {} = random", i + 1, icon_count);
            push = true;
        }

        let mut fighter = ""; // character we will try inserting
        for _ in 0..chara_order.len() {
            fighter = chara_order[idx].as_str().unwrap_or("goku");
            let allowed = chara_whitelist.contains(&fighter.to_string());
            if allowed { break };
            // Repeat until we find an allowed entry
            // println!("Fighter {fighter} is not part of the original load, and will not be loaded.");
            idx += 1;
        }
        
        let num = i - if push { 1 } else { 0 };
        if fighter == "element" { // aegis is a special case and is loaded with two entries
            new_order.push(ui_chara(num, "flame_first"));
            new_order.push(ui_chara(num, "light_first"));
        } else {
            new_order.push(ui_chara(num, fighter));
        }
        // println!("{} / {} = {}", n + 1, icon_count, fighter);
        
        idx += 1;
    };

    // replace the original vec data with the new order
    chara_vec.clear();
    for i in 0..new_order.len() {
        chara_vec.push(*new_order.get(i).unwrap_or(&1));
    };
}

#[skyline::hook(offset = 0x19fd0b0)]
unsafe fn update_player_tag(arg1: u64, tag_index: *const u8) {
    PLAYER_TAG_INDEX[*((arg1 + 0x1d4) as *const u8) as usize] = *tag_index;
    call_original!(arg1, tag_index);
}

#[skyline::hook(offset = 0x1a14280, inline)]
unsafe fn change_random_early(ctx: &mut skyline::hooks::InlineCtx) {
    let obj = *ctx.registers[23].x.as_ref() as *mut u64;
    let obj = *(obj as *mut *mut u64).add(1);
    // println!("Entering change_random_early");
    let ignore_random = ninput::any::is_down_any(ninput::Buttons::ZL | ninput::Buttons::ZR);
    if ignore_random {
        // println!("Ignoring the melee random selection!");
    }

    let main_chara = *obj.add(0x200 / 0x8);
    let sub_chara = *obj.add(0x208 / 0x8);

    if !ignore_random && (is_random(main_chara) || is_random(sub_chara)) {
        // println!("The random pane was selected");
        let player_id = (*(*(ctx.registers[21].x.as_ref() as *const u64) as *const u64) + 0x150) as *const u8;
        let chara_string = &decide_fighter_from_id(*player_id as usize);
        let chara_hash = hash40(&format!("ui_chara_{}", chara_string)).0;
        LAST_FIGHTER_FOUND = chara_hash | key(main_chara);
        LAST_FIGHTER2_FOUND = if chara_string == "ptrainer" {
            PT_CHARA_HASHES.choose(&mut rand::thread_rng()).copied().unwrap_or(hash40("ui_chara_random").0) | key(sub_chara)
        } else {
            chara_hash | key(sub_chara)
        };
        // println!("Main character: {:#x}", LAST_FIGHTER_FOUND);
        // println!("Sub character: {:#x}", LAST_FIGHTER2_FOUND);

        *ctx.registers[24].x.as_mut() = LAST_FIGHTER_FOUND;
        WAS_RANDOM_SELECTION = true;
    } else {
        WAS_RANDOM_SELECTION = false;
    }
}

static mut ACTIVE_ID: u64 = 0x0;

// runs first, processes miscellaneous info for the selected fighter
#[skyline::hook(offset = 0x1a1cb30)]
unsafe fn decide_costume(dest: u64, src: u64) {
    // println!("Entering decide_costume");
    let src_obj = *(src as *mut *mut u64).add(1);
    let src_obj = src_obj.add(0x1F0 / 8);
    ACTIVE_ID = (*(src_obj as *mut u32) - 1) as u64;

    if WAS_RANDOM_SELECTION {
        // handle rng
        let mut rng = COSTUME_RNG.lock().unwrap();
        let costume = rng.choose(&mut rand::thread_rng()).copied().unwrap_or(0);
        rng.retain(|&x| x != costume);
        // reset rng if the vector is empty
        if rng.len() <= 0 {
            for i in 0..8 { rng.push(i); }
        }
    
        *(src_obj as *mut u32).add(8) = costume as u32;
        // println!("Randomly selected costume slot to be {}", *(src_obj as *mut u32).add(8));
    }

    call_original!(dest, src);
}

// runs second, only runs on random pane selected
#[skyline::hook(offset = 0x1a0d540)]
unsafe fn decide_fighter(arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    // println!("Entering decide_fighter");
    // if !WAS_RANDOM_SELECTION {
    //     println!("decide_fighter called when the selection was not random");
    // }

    let p_main_chara = (arg1 as *mut u64).add(2);
    let p_sub_chara = (arg1 as *mut u64).add(3);

    if WAS_RANDOM_SELECTION && (is_random(*p_main_chara) || is_random(*p_sub_chara)) {
        *p_main_chara = LAST_FIGHTER_FOUND;
        *p_sub_chara = LAST_FIGHTER2_FOUND;
    }
    // println!("Decided on fighter: {:#x}", *p_main_chara);
    // println!("Sub-fighter: {:#x}", *p_sub_chara);

    // WAS_RANDOM_SELECTION = false;
    // println!("Cleared random selection flag");

    call_original!(arg1, arg2, arg3, arg4)
}

// determines what fighter is picked by random on selection (non-melee style)
// this offset is found a little further down in the decide_fighter function above
#[skyline::hook(offset = 0x1a0d628, inline)]
unsafe fn random_whitelist(ctx: &mut skyline::hooks::InlineCtx) {
    let chara_data = *ctx.registers[23].x.as_ref();

    let player_id = ACTIVE_ID;
    let chara_string = &decide_fighter_from_id(player_id as usize);
    let chara_hash = hash40(&format!("ui_chara_{}", chara_string)).0;
    let new_data = chara_hash | key(chara_data);
    // println!("Changed random fighter to: {:#x}", new_data);
    *ctx.registers[23].x.as_mut() = new_data;
}

#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = *ctx.registers[0].x.as_ref() as *mut u64;
    let ptr2 = *ctx.registers[1].x.as_ref() as *mut u64;

    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

pub fn install() {
    skyline::install_hooks!(
        display_css_hook,
        update_player_tag,

        change_random_early,
        decide_costume,
        decide_fighter,
        random_whitelist,
        fix_chara_replace
    );
}