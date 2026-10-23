pub trait Scored {
    fn points(&self) -> u32;
    fn name(&self) -> String;
    // fn label(&self) -> String  (with a default body)
}

pub struct Player {
    pub name: String,
    pub points: u32,
}

pub struct Team {
    pub name: String,
    pub members: Vec<Player>,
}

// impl Scored for Player { ... }

// impl Scored for Team { ... }  (override label)

pub fn best<T: Scored>(items: &[T]) -> Option<&T> {
    todo!()
}

pub fn report(item: &impl Scored) -> String {
    todo!()
}

#[cfg(test)]
mod tests;
