use std::time::Instant;

#[derive(Clone)]
pub struct State {
    boss_hp: i32, 
    player_hp: i32, 
    mana: i32,
}

#[derive(Debug)]
pub struct Least {
    mana: i32,
}

pub fn do_turn(state: State, active_spells: Vec<Vec<i32>>, player_turn: bool, mana_used: i32, least: &mut Least, part: i32) -> bool {
    let missile: Vec<i32> = vec![53,4,0,0,0,0,0];
    let drain: Vec<i32> = vec![73,2,2,0,0,0,1];
    let shield: Vec<i32> = vec![113,0,0,7,0,6,2];
    let poison: Vec<i32> = vec![173,3,0,0,0,6,3];
    let recharge: Vec<i32> = vec![229,0,0,0,101,5,4];

    let spells: Vec<Vec<i32>> = vec![missile, drain, shield, poison, recharge];

    let boss_damage: i32 = 9;
    let mut player_armour: i32 = 0;

    let mut new_state: State = state.clone();

    if part == 2 && player_turn {
        new_state.player_hp -= 1;
        if new_state.player_hp <= 0 {
            return false;
        }
    }

    let mut new_active_spells: Vec<Vec<i32>> = vec![];
    for active_spell in active_spells {
        if active_spell[5] >= 0 {
            new_state.boss_hp -= active_spell[1];
            new_state.player_hp += active_spell[2];
            player_armour += active_spell[3];
            new_state.mana += active_spell[4];
        }
        let mut new_active_spell: Vec<i32> = active_spell.clone();
        new_active_spell[5] -= 1;
        if new_active_spell[5] > 0 {
            new_active_spells.push(new_active_spell);
        }
    }

    if new_state.boss_hp <= 0 {
        if mana_used < least.mana || least.mana == -1 {
            least.mana = mana_used;
        }
        return true
    }

    if mana_used >= least.mana && least.mana != -1 {
        return false
    }

    if player_turn {
        for spell in spells {
            let mut spell_already_active: bool = false;

            for new_active_spell in &new_active_spells {
                if new_active_spell[6] == spell[6] {
                    spell_already_active = true;
                    break;
                }
            }

            let mana_cost: i32 = spell[0];

            if mana_cost <= new_state.mana && !spell_already_active {
                let mut n_a_s: Vec<Vec<i32>> = new_active_spells.clone();
                n_a_s.push(spell.clone());
                let mut next_state: State = new_state.clone();
                next_state.mana -= mana_cost;
                do_turn(next_state, n_a_s, false, mana_used + mana_cost, least, part);
            }
        }
    } else {
        if player_armour - boss_damage < 0 {
            new_state.player_hp += player_armour - boss_damage;
        } else {
            new_state.player_hp += -1;
        }
        if new_state.player_hp > 0 {
            return do_turn(new_state, new_active_spells, true, mana_used, least, part)
        }
    }
    return false
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 22 of 2015");

    let mut least: Least = Least{ mana: -1 };
    let state: State = State { boss_hp: 58, player_hp: 50, mana: 500};

    let start1 = Instant::now();

    do_turn(state.clone(), vec![], true, 0, &mut least, 1);

    let after1 = Instant::now();
    println!("Part 1: {} (in {:?})", least.mana, after1.duration_since(start1));

    least.mana = -1;

    let start2 = Instant::now();

    do_turn(state.clone(), vec![], true, 0, &mut least, 2);

    let after2 = Instant::now();
    println!("Part 2: {} (in {:?})", least.mana, after2.duration_since(start2));
}
