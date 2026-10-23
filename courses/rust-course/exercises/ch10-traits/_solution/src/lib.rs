pub trait Scored {
    fn points(&self) -> u32;
    fn name(&self) -> String;

    fn label(&self) -> String {
        format!("{}: {} pts", self.name(), self.points())
    }
}

pub struct Player {
    pub name: String,
    pub points: u32,
}

pub struct Team {
    pub name: String,
    pub members: Vec<Player>,
}

impl Scored for Player {
    fn points(&self) -> u32 {
        self.points
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Scored for Team {
    fn points(&self) -> u32 {
        self.members.iter().map(|p| p.points).sum()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn label(&self) -> String {
        format!("Team {}: {} pts ({} members)", self.name, self.points(), self.members.len())
    }
}

pub fn best<T: Scored>(items: &[T]) -> Option<&T> {
    let mut top = items.first()?;
    for item in items {
        if item.points() > top.points() {
            top = item;
        }
    }
    Some(top)
}

pub fn report(item: &impl Scored) -> String {
    format!("[{}]", item.label())
}

#[cfg(test)]
mod tests;
