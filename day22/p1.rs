use std::cmp::{max, min};

#[derive(Clone)]
struct GameState {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    boss_damage: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    mana_spent: i32,
}

impl GameState {
    fn new(player_hp: i32, player_mana: i32, boss_hp: i32, boss_damage: i32) -> Self {
        Self {
            player_hp,
            player_mana,
            boss_hp,
            boss_damage,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            mana_spent: 0,
        }
    }

    fn apply_effects(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
        if self.poison_timer > 0 {
            self.boss_hp -= 3;
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.player_mana += 101;
            self.recharge_timer -= 1;
        }
    }

    fn boss_attack(&mut self) {
        let armor = if self.shield_timer > 0 { 7 } else { 0 };
        let damage = max(1, self.boss_damage - armor);
        self.player_hp -= damage;
    }

    fn is_player_dead(&self) -> bool {
        self.player_hp <= 0
    }

    fn is_boss_dead(&self) -> bool {
        self.boss_hp <= 0
    }
}

struct Spell {
    name: &'static str,
    cost: i32,
    effect: fn(&mut GameState),
}

impl Spell {
    fn magic_missile(state: &mut GameState) {
        state.boss_hp -= 4;
    }

    fn drain(state: &mut GameState) {
        state.boss_hp -= 2;
        state.player_hp += 2;
    }

    fn shield(state: &mut GameState) {
        state.shield_timer = 6;
    }

    fn poison(state: &mut GameState) {
        state.poison_timer = 6;
    }

    fn recharge(state: &mut GameState) {
        state.recharge_timer = 5;
    }
}

fn find_min_mana_spent(mut state: GameState, min_mana: &mut i32) {
    // Hard mode: player loses 1 HP at the start of their turn
    state.player_hp -= 1;
    if state.is_player_dead() {
        return;
    }

    if state.mana_spent >= *min_mana {
        return;
    }

    state.apply_effects();

    if state.is_boss_dead() {
        *min_mana = min(*min_mana, state.mana_spent);
        return;
    }

    let available_spells = vec![
        Spell { name: "Magic Missile", cost: 53, effect: Spell::magic_missile },
        Spell { name: "Drain", cost: 73, effect: Spell::drain },
        Spell { name: "Shield", cost: 113, effect: Spell::shield },
        Spell { name: "Poison", cost: 173, effect: Spell::poison },
        Spell { name: "Recharge", cost: 229, effect: Spell::recharge },
    ];

    for spell in available_spells {
        if state.player_mana < spell.cost {
            continue;
        }

        if (spell.name == "Shield" && state.shield_timer > 0)
            || (spell.name == "Poison" && state.poison_timer > 0)
            || (spell.name == "Recharge" && state.recharge_timer > 0)
        {
            continue;
        }

        let mut next_state = state.clone();
        next_state.player_mana -= spell.cost;
        next_state.mana_spent += spell.cost;
        (spell.effect)(&mut next_state);

        next_state.apply_effects();

        if next_state.is_boss_dead() {
            *min_mana = min(*min_mana, next_state.mana_spent);
            continue;
        }

        next_state.boss_attack();

        if next_state.is_player_dead() {
            continue;
        }

        find_min_mana_spent(next_state, min_mana);
    }
}

fn main() {
    let initial_state = GameState::new(50, 500, 71, 10);
    let mut min_mana = i32::MAX;

    find_min_mana_spent(initial_state, &mut min_mana);

    println!("Least mana spent to win: {}", min_mana);
}
