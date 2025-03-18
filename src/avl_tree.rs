use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

pub enum BalanceFactor {
    Balanced,
    LeftHeavy,
    RightHeavy,
    TooLeftHeavy,
    TooRightHeavy,
}

pub enum AVLRotation {
    RightRight,
    RightLeft,
    LeftRight,
    LeftLeft,
}

pub struct AVLTree<'a, K, V, P, PropFn, OrderingFn>
where
    K: Eq + Hash,
    PropFn: Fn(&V) -> &P,
    OrderingFn: Fn(&P, &P) -> Ordering
{
    root_key: Option<K>,
    nodes: HashMap<K, AVLNode<'a, K, V>>,
    prop_fn: PropFn,
    order_fn: OrderingFn,
}

impl<K, V, P, PropFn, OrderingFn> AVLTree<'_, K, V, P, PropFn, OrderingFn>
where
    K: Eq + Hash,
    PropFn: Fn(&V) -> &P,
    OrderingFn: Fn(&P, &P) -> Ordering
{
    pub fn new(prop_fn: PropFn, order_fn: OrderingFn) -> Self {
        Self {
            root_key: None,
            nodes: HashMap::new(),
            prop_fn,
            order_fn,
        }
    }

    pub fn get(&self, key: &K) -> Option<&P> {
        self.nodes.get(key).map(|node| (self.prop_fn)(&node.value))
    }

    pub fn search(&self, value: &P) -> Option<&K> {
        todo!("Proper search")
    }

    pub fn insert(&mut self, value: V) -> u64 {
        todo!("Proper insertion and rotations")
    }

    pub fn delete(&mut self, key: &K) -> bool {
        if !self.nodes.contains_key(key) {
            return false;
        }

        todo!("Proper deletion and rotations");
    }
}

pub struct AVLNode<'a, K, V> {
    pub key: K, // approx. 609 million keys need to be generated to get a 1% chance of getting a collision using a u64
    pub value: V,
    pub parent: Option<&'a Self>,
    pub left: Option<&'a Self>,
    pub right: Option<&'a Self>,
    pub height: u32,
}

impl<K, V> AVLNode<'_, K, V> {
    pub fn new(key: K, value: V) -> Self {
        AVLNode {
            key,
            value,
            parent: None,
            left: None,
            right: None,
            height: 0,
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn balance_factor(&self) -> BalanceFactor {
        let left_height = self.left.map_or(0, |left_node| left_node.height);
        let right_height = self.right.map_or(0, |right_node| right_node.height);

        // right - left = -2
        // => right + 2 = left
        if right_height.saturating_add(2) == left_height {
            return BalanceFactor::TooLeftHeavy;
        }

        // right - left = 2
        // => left + 2 = right
        if left_height.saturating_add(2) == right_height {
            return BalanceFactor::TooRightHeavy;
        }

        match left_height.cmp(&right_height) {
            Ordering::Equal => BalanceFactor::Balanced,
            Ordering::Greater => BalanceFactor::LeftHeavy,
            Ordering::Less => BalanceFactor::RightHeavy,
        }
    }
}
