#[derive(Clone, Copy, Debug)]
pub enum MenuItem {
    Account,
    Settitngs
}

impl From<MenuItem> for usize {
    fn from(item: MenuItem) -> Self {
        use MenuItem::*;

        match item {
            Account => 0,
            Settitngs => 1,
        }
    }
}