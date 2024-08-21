use super::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 30, 80, 0, 50, 3.0, 0.0, -1.7, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 2.3, 0.0, -1.7, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0, -1, -1.1, 0, 0, 0, 1, false);
        sv_animcmd::EFFECT_FLW_POS_UNSYNC_VIS(agent.lua_state_agent);
        LAST_EFFECT_SET_SCALE_W(agent, 1.1, 0.78, 1.1);
        LAST_EFFECT_SET_RATE(agent, 1.23);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot", game_shoot, Priority::Low);
    agent.acmd("effect_shoot", effect_shoot, Priority::Low);
}
