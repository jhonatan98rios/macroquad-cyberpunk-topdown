use macroquad::rand::{gen_range};

fn get_random_skill() -> &'static str {

    let skill_index = gen_range(0, 5);
    
    return match skill_index {
        0 => "Fireball 0",
        1 => "Ice Shard 1",
        2 => "Lightning Bolt 2",
        3 => "Blablabla 3",
        4 => "Fireball 4",
        _ => "Unknown Skill",
    };
}

pub fn get_random_skills() -> Vec<&'static str> {
    let mut skills = Vec::new();
    for _ in 0..3 {
        skills.push(get_random_skill());
    }
    return skills;
}