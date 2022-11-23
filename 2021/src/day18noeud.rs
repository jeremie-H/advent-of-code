use std::{
    cell::RefCell,
    fmt::{self, Debug, Display, Formatter},
    ops::Add,
};



#[derive(Clone)]
pub enum Noeud {
    Valeur {
        valeur: RefCell<Box<i64>>,
    },
    Paire {
        gauche: RefCell<Box<Noeud>>,
        droite: RefCell<Box<Noeud>>,
    },
}

impl Display for Noeud {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Noeud::Valeur { valeur } => write!(f, "{}", valeur.borrow_mut()),
            Noeud::Paire { gauche, droite } => write!(f, "[{},{}]", gauche.borrow_mut(), droite.borrow_mut()),
        }
    }
}

impl Debug for Noeud {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Noeud::Valeur { valeur } => write!(f, "{}", valeur.borrow_mut()),
            Noeud::Paire { gauche, droite } => write!(f, "[{},{}]", gauche.borrow_mut(), droite.borrow_mut()),
        }
    }
}

impl Noeud {
    pub(crate) fn value(&self) -> i64 {
        match self {
            Noeud::Valeur { valeur } => **valeur.borrow(),
            Noeud::Paire { gauche: _, droite: _ } => i64::MIN,
        }
    }

    pub(crate) fn magnitude(&self) -> i64 {
        match self {
            Noeud::Valeur { valeur: _ } => panic!("on ne descend jamais jusqu'à ce niveau"),
            Noeud::Paire { gauche, droite } => match gauche.borrow().as_ref() {
                Noeud::Valeur { valeur: v_g } => match droite.borrow().as_ref() {
                    Noeud::Valeur { valeur: v_d } => 3 * (**v_g.borrow()) + 2 * (**v_d.borrow()),
                    Noeud::Paire { gauche: _, droite: _ } => 3 * (**v_g.borrow()) + 2 * (droite.borrow().magnitude()),
                },
                Noeud::Paire { gauche: _, droite: _ } => match droite.borrow().as_ref() {
                    Noeud::Valeur { valeur: v_d } => 3 * gauche.borrow().magnitude() + 2 * (**v_d.borrow()),
                    Noeud::Paire { gauche: _, droite: _ } => 3 * gauche.borrow().magnitude() + 2 * droite.borrow().magnitude(),
                },
            },
        }
    }

}

impl Add for Noeud {
    type Output = Self;

    fn add(self, deuxieme: Noeud) -> Self::Output {
        Self::Paire {
            gauche: RefCell::new(Box::new(self)),
            droite: RefCell::new(Box::new(deuxieme)),
        }
    }
}