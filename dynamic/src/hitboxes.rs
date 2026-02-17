pub use json_script::{
    CollisionPart,
    CollisionSituation,
    CollisionCategory,
    ShieldDamage,
    SoundLevel,
    CollisionSound,
    AttackRegion,
    SetOff,
    LrCheck,
    HitboxTemplate,
    HitboxData,
    create_hitbox,
};
pub use json_script_macro::{
    hitbox_templates,
    decl_hitbox,
    hitbox
};
hitbox_templates! {
    pub BASE_HITBOX = {
        fkb: 0,
        hitlag: 1.0,
        sdi: 1.0,
        clank: SetOff::On,
        facing: LrCheck::Pos,
        set_weight: false,
        shield_dmg: ShieldDamage::Add(0.0),
        trip: 0.0,
        rehit: 0,
        reflectable: false,
        absorbable: false,
        flinchless: false,
        disable_hitlag: false,
        direct: true,
        friendly_fire: false,
        situation: CollisionSituation::GA,
        category: CollisionCategory::all(),
        hit_part: CollisionPart::all(),
        effect: "collision_attr_normal",
    };
}