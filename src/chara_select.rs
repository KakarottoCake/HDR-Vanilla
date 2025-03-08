use std::{
    collections::HashMap,
    fs::*,
    path::{ Path, PathBuf },
    sync::{ LazyLock, RwLock }
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
use serde::Deserialize;
use toml::{ Value };
use utils::modules::TourneyConfig;

const ORDER_TOML: &str = "ui/param/menu/chara_icon_order.toml";
const RANDOM_IDX_TOML: &str = "ui/param/menu/chara_random_idx.toml";
const RANDOM_CFG_TOML: &str = "ui/param/menu/chara_random_config.toml";

const KEY_MASK: u64 = 0xFFFFFF_0000000000;

static mut PLAYER_TAG_INDEX: &'static mut [u8] = &mut [0; 8];

static mut CHARA_DATA: LazyLock<RwLock<CharaData>> = LazyLock::new(|| 
    RwLock::new(CharaData::default())
);

#[derive(Debug, Clone)]
struct CharaData {
    main_id: u64,
    sub_id: u64,
    last_selection: String,
    melee_random: bool,
    use_default: bool,
    costume: u8,
    costume_rng: Vec<i32>,

    whitelist: Vec<String>,
    blacklist: Vec<String>
}
impl Default for CharaData {
    fn default() -> Self {
        CharaData {
            main_id: 0x0,
            sub_id: 0x0,
            last_selection: String::new(),
            melee_random: false,
            use_default: false,
            costume: 0,
            costume_rng: (0..8).collect::<Vec<i32>>(),

            whitelist: Vec::new(),
            blacklist: Vec::new()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct CharaConfig {
    enabled: bool,
    order: String,
    #[serde(flatten)]
    schemas: HashMap<String, CharaSchema>
}
impl Default for CharaConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("series".into(), CharaSchema::default());
        CharaConfig {
            enabled: true,
            order: "series".into(),
            schemas: map
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
struct CharaSchema {
    centered_random: bool,
    order: Vec<String>
}
impl Default for CharaSchema {
    fn default() -> Self {
        CharaSchema {
            centered_random: true,
            order: [
                // default order of fighters in the CSS for when tourney mode is enabled, or config is invalid
                "mario", "mariod", "luigi", "peach", "daisy", "rosetta", "koopa", "koopajr", "packun", "yoshi", "wario", "donkey", "diddy", "krool", "buddy", "ice_climber", "gamewatch",
                "link", "younglink", "toonlink", "zelda", "sheik", "ganon", "samus", "szerosuit", "ridley", "samusd", "kirby", "metaknight", "dedede", "fox", "falco", "wolf", "robot", "duckhunt",
                "pikachu", "pichu", "ptrainer", "purin", "mewtwo", "lucario", "gekkouga", "gaogaen", "marth", "roy", "ike", "reflet", "chrom", "lucina", "kamui", "master",
                "ness", "lucas", "captain", "pit", "pitb", "palutena", "pikmin", "murabito", "shizue", "wiifit", "littlemac", "shulk", "element", "inkling", "tantan", "miifighter", "miiswordsman", "miigunner",
                "snake", "simon", "richter", "sonic", "bayonetta", "jack", "rockman", "ryu", "ken", "dolly", "demon", "pacman", "cloud", "edge", "trail", "brave", "pickel"
            ]
            .map(|x| x.to_string()).to_vec()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct RandomConfig {
    #[serde(flatten)]
    tags: HashMap<String, TagData>
}
impl Default for RandomConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("global_settings".into(), TagData::default());

        RandomConfig { tags: map }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
struct TagData {
    kind: ListKind,
    list: Vec<String>
}
impl Default for TagData {
    fn default() -> Self {
        TagData {
            kind: ListKind::Blacklist,
            list: Vec::new()
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
enum ListKind {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist
}

// transmutes the supplied chara string into data for the CSS icon vector
fn ui_chara(i: usize, chara: &str) -> u64 {
    (0xc1u64 << 56)
    | (((i as u64) & 0xFFFF) << 40)
    | hash40(&format!("ui_chara_{}", chara)).0
}

fn is_tourney_mode() -> bool {
    match TourneyConfig::load() {
        Some(config) => config.enabled,
        None => false
    }
}

#[skyline::hook(offset = 0x19eb840, inline)]
pub unsafe fn init_css_hook(ctx: &InlineCtx) {
    // reset all css data to default
    let mut chara_data = CHARA_DATA.write().unwrap();
    *chara_data = CharaData::default();

    // collect data from TOML
    let path = Path::new("mods:/").join(ORDER_TOML);
    let data = match std::fs::read_to_string(&path) {
        Ok(result) => result,
        Err(e) => {
            println!("[src::chara_select] Could not read TOML: {}", e);
            return;
        }
    };
    let config: CharaConfig = match toml::from_str(&data) {
        Ok(result) => result,
        Err(e) => {
            println!("[src::chara_select] Error parsing TOML: {}", e);
            return;
        }
    };
    if !config.enabled { return }; // aborts if the config set this to false
    
    // get the original vector of fighter entries to be loaded
    let chara_vec = &mut *(*ctx.registers[4].x.as_ref() as *mut smash2::cpp::Vector<u64>);
    let (mut whitelist, mut blacklist) = (Vec::new(), Vec::new());
    
    let schema: CharaSchema = config.schemas.get(&config.order).cloned().unwrap_or_default();
    let chara_order = if is_tourney_mode() { CharaSchema::default().order } else { schema.order.clone() };
    for idx in 0..chara_order.len() {
        let chara = match chara_order.get(idx) {
            Some(string) => {
                if string == "element" { "flame_first" }
                else { string.as_str() }
            },
            None => "goku"
        };
        let should_load = chara_vec.iter().any(|x|
            (*x & !KEY_MASK) == hash40(&format!("ui_chara_{}", chara)).0
        );

        let dest = if should_load { &mut whitelist } else { &mut blacklist };
        dest.push(chara.to_string());
        if chara == "flame_first" { // aegis is a special case and is loaded with two entries
            dest.push("light_first".to_string()) 
        };
        // println!("Fighter {chara} will{} be loaded", if should_load { "" } else { " not" });
    }

    let mut icon_count = chara_order.len();
    if schema.centered_random { icon_count += 1 }; // +1 to the order if random is to be inserted
    icon_count -= blacklist.len(); // subtract blacklisted fighters from the total

    let random_idx = get_random_idx(icon_count);
    // println!(
    //     "{icon_count} icons to load ({} out of {} blacklisted).\nRandom will be placed in slot {random_idx}", 
    //     blacklist.len(), chara_order.len() + if schema.centered_random { 1 } else { 0 }
    // );

    let mut new_order = Vec::new();
    let mut push = false;
    let use_general_all = chara_vec.iter().any(|x| (*x & !KEY_MASK) == hash40("ui_chara_general_all").0);

    let mut fighters = chara_order.clone();
    fighters.reverse(); // convert into a stack
    
    for i in 0..icon_count {
        if i == random_idx && schema.centered_random && !push {
            let entry = if use_general_all { "general_all" } else { "random" };
            new_order.push(ui_chara(i, entry));
            push = true;
            continue;
        }

        let mut fighter = String::new();
        while fighter.is_empty() {
            let next = match fighters.pop() {
                Some(fighter) => fighter,
                None => break
            };
            if whitelist.contains(&next) { fighter = next };
        }
        
        let num = i - if push { 1 } else { 0 };
        if fighter == "random" && use_general_all {
            new_order.push(ui_chara(num, "general_all"));
        } 
        else {
            new_order.push(ui_chara(num, fighter.as_str()));
        }
    };

    // replace the original vec data with the new order
    chara_vec.clear();
    for entry in new_order {
        chara_vec.push(entry);
    }

    chara_data.whitelist = whitelist;
    chara_data.blacklist = blacklist;
}

fn get_random_idx(icon_count: usize) -> usize {
    let mut idx = icon_count / 2;
    let path = Path::new("mods:/").join(RANDOM_IDX_TOML);
    if path.exists() && path.is_file() {
        let data = match std::fs::read_to_string(&path).unwrap().parse::<Value>() {
            Ok(result) => result,
            Err(_) => return idx
        };
        let placement = match data.as_table().unwrap().get("placement") {
            Some(table) => table.as_table().unwrap(),
            None => return idx
        };
        match placement.get(&icon_count.to_string()) {
            Some(value) => { idx = value.as_integer().unwrap() as usize },
            None => {}
        };
    }

    idx
}

// this hook occurs during the main loop of the css, where it calls a function to select a chosen fighter from random
#[skyline::hook(offset = 0x1a14280, inline)]
unsafe fn decide_random(ctx: &mut skyline::hooks::InlineCtx) {
    let src = *ctx.registers[23].x.as_ref() as *mut u64;
    let obj_ptr = *(src as *mut *mut u64).add(1);

    let main_chara = *obj_ptr.add(0x200 / 0x8);
    let sub_chara = *obj_ptr.add(0x208 / 0x8);
    // println!("Main character: {:#x}", main_chara as u32);
    // println!("Sub character: {:#x}", sub_chara as u32);

    // casting the u64 to u32 will give us the hash40 equivalent, so no need for any bit masking 
    let is_random = [main_chara as u32, sub_chara as u32].contains(&(hash40("ui_chara_random").0 as u32));
    if !is_random { return };
    
    let is_melee = ninput::any::is_down_any(ninput::Buttons::ZL | ninput::Buttons::ZR);
    if is_melee {
        let player_id = (*(*(ctx.registers[21].x.as_ref() as *const u64) as *const u64) + 0x150) as *const u8;
        match generate_random(*player_id as usize, main_chara, sub_chara) {
            Ok(_) => {
                *ctx.registers[24].x.as_mut() = CHARA_DATA.read().unwrap().main_id;
            },
            Err(e) => {
                println!("[src::chara_select] Error generating random fighter: {}", e);
                CHARA_DATA.write().unwrap().use_default = true;
            }
        };
    }

    CHARA_DATA.write().unwrap().melee_random = is_melee;
}

unsafe fn generate_random(player_id: usize, main_data: u64, sub_data: u64) -> Result<(), String> {
    let mut chara_string = match decide_fighter_from_id(player_id) {
        Ok(string) => string,
        Err(e) => return Err(e)
    };
    let mut chara_hash = hash40(&format!("ui_chara_{}", chara_string)).0;

    let mut chara_data = CHARA_DATA.write().unwrap();
    chara_data.main_id = chara_hash | (main_data & KEY_MASK);
    
    if chara_string == "ptrainer" {
        chara_hash = [
            hash40("ui_chara_pzenigame").0,
            hash40("ui_chara_plizardon").0,
            hash40("ui_chara_pfushigisou").0,
        ]
        .choose(&mut rand::thread_rng()).copied()
        .unwrap_or(hash40("ui_chara_random").0);
    }
    chara_data.sub_id = chara_hash | (sub_data & KEY_MASK);
   
    // handle costume rng
    let mut rng = chara_data.costume_rng.clone();
    let costume = {
        rng.choose(&mut rand::thread_rng()).copied()
        .unwrap_or((rand::thread_rng().gen::<u32>() % 8) as i32)
    };
    rng.retain(|&x| x != costume);

    if rng.is_empty() { rng = CharaData::default().costume_rng };

    chara_data.costume = costume as u8;
    chara_data.costume_rng = rng;
    println!("Randomly selected costume slot to be {costume}");

    Ok(())
}

unsafe fn decide_fighter_from_id(id: usize) -> Result<String, String> {
    if is_tourney_mode() {
        return Err("Tourney mode enabled! Bypassing random config.".into())
    }

    let chara_data = { CHARA_DATA.read().unwrap().clone() };
    let mut whitelist = chara_data.whitelist;

    // make sure miis cannot be selected
    for mii in [
        "miifighter", "miiswordsman", "miigunner"
    ] {
        whitelist.retain(|x| *x != mii.to_owned());
    }

    // choose a fighter from base whitelist
    let mut chara_string =  match whitelist.choose(&mut rand::thread_rng()) {
        Some(string) => string.as_str(),
        None => return Err("Whitelist is empty!".into())
    };
    let default = chara_string.to_owned();
    println!("Default character decision: {chara_string}");


    // Collect all relevant data from config TOML
    let path = Path::new("mods:/").join(RANDOM_CFG_TOML);
    let data = match std::fs::read_to_string(&path) {
        Ok(result) => result,
        Err(e) => return Err("Could not read TOML!".into())
    };
    let config: RandomConfig = match toml::from_str(&data) {
        Ok(result) => result,
        Err(e) => return Err(format!("Error parsing TOML: {}", e))
    };
    
    let tag_index = PLAYER_TAG_INDEX[id];
    let tag = &get_tag_from_save(tag_index);
    println!("Tag data for slot {}: {}", id, &tag); 
    let tag_data: TagData = match config.tags.get(tag) {
        Some(data) => data.clone(),
        None => {
            println!("[src::chara_select] No settings defined for tag [{tag}]! Using global settings.");
            match config.tags.get("global_settings") {
                Some(global) => global.clone(),
                None => {
                    println!("...No global settings defined! Using default.");
                    TagData::default()
                }
            }
        }
    };

    match tag_data.kind {
        ListKind::Whitelist => {
            if tag_data.list.contains(&default) {
                println!("{chara_string} is allowed!");
                return Ok(default);
            }

            println!("{chara_string} is not allowed for tag {}! Adjusting...", &tag);
            let size = whitelist.len();
            whitelist.retain(|x| {
                let restrict_prev = (*x == chara_data.last_selection && size > 1);
                tag_data.list.contains(&x) && !restrict_prev
            });

            // if none of the whitelisted characters were allowed originally, return the default
            if whitelist.len() <= 0 {
                return Err("None of the whitelisted characters are allowed!".into());
            }
        }
        ListKind::Blacklist => {
            if !tag_data.list.contains(&default) {
                println!("{chara_string} is allowed!");
                return Ok(default);
            }

            println!("{chara_string} is not allowed for tag {}! Adjusting...", &tag);
            for chara in tag_data.list {
                whitelist.retain(|x| *x != chara);
            }
            if whitelist.len() > 1 {
                // prevent the same character from being picked twice in a row (if possible)
                whitelist.retain(|x| *x != chara_data.last_selection);
            }
        }
    }
    // re-query with adjusted list
    chara_string = match whitelist.choose(&mut rand::thread_rng()) {
        Some(str) => str.as_str(),
        None => return Err("Whitelist is empty!".into())
    };
    println!("Randomly decided on {chara_string}");
    CHARA_DATA.write().unwrap().last_selection = chara_string.to_owned();

    Ok(chara_string.to_owned())
}

#[skyline::hook(offset = 0x1a0d540)]
unsafe fn set_random_fighter_data(base_ptr: *mut u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    let chara_data = { CHARA_DATA.read().unwrap().clone() };
    if chara_data.use_default { 
        return call_original!(base_ptr, arg2, arg3, arg4) 
    };
    
    let main_chara = base_ptr.add(2);
    let sub_chara = base_ptr.add(3);
    // println!("Fighter: {:#x}, Sub-fighter: {:#x}", *main_chara, *sub_chara);

    if chara_data.melee_random { // melee random should force these values right away
        *main_chara = chara_data.main_id;
        *sub_chara = chara_data.sub_id;
    } else {
        // ensure random is re-rolled between games
        let player_id = (*base_ptr as u8 - 1) as usize;
        match generate_random(player_id, *main_chara, *sub_chara) {
            Ok(_) => {},
            Err(e) => {
                println!("[src::chara_select] Error generating random fighter: {}", e);
                CHARA_DATA.write().unwrap().use_default = true;
            }
        };
    }

    let ret = call_original!(base_ptr, arg2, arg3, arg4);

    // at this point, for the normal random, it's safe to modify the data without affecting any UI
    // note that we will only change the SUB fighter here. the main fighter will always just be ui_chara_random
    let chara_data = CHARA_DATA.read().unwrap();
    if !chara_data.melee_random {
        println!("Setting random fighter data for player {:#x}", *base_ptr as u8);
        *sub_chara = chara_data.sub_id;
    }

    // set costume
    let costume_ptr = (base_ptr as *mut u64).add(4) as *mut u8;
    *costume_ptr = chara_data.costume;

    ret
}

#[skyline::hook(offset = 0x19fd0b0)]
unsafe fn update_player_tag(arg1: u64, tag_index: *const u8) {
    let player_id = *((arg1 + 0x1d4) as *const u8) as usize;
    PLAYER_TAG_INDEX[player_id] = *tag_index;
    call_original!(arg1, tag_index);
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
        init_css_hook,
        decide_random,
        set_random_fighter_data,
        update_player_tag,
        fix_chara_replace
    );
}