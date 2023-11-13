#[derive()]
pub struct TrocoMoeda {
    pub(crate) moeda: i32,
    pub(crate) quantidade: i32,
}

impl TrocoMoeda {
    pub(crate) fn new(moeda: i32, quantidade: i32) -> TrocoMoeda {
        TrocoMoeda {
            moeda,
            quantidade,
        }
    }
}