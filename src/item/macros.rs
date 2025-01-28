macro_rules! get_ancestor {
    ($self:ident, $get_younger:tt, $get_older:tt) => {{
        let younger_ancestor = $self.$get_younger()?;
        let older_ancestor = younger_ancestor.get_parent_component().as_ref()?.get();
        $self.get_query().$get_older(older_ancestor).ok()
    }};
}

macro_rules! get_parent {
    ($self:ident, $get_func:tt) => {
        $self
            .get_parent_component()
            .as_ref()
            .and_then(|parent| $self.get_query().$get_func(parent.get()).ok())
    };
}

macro_rules! iter_descendents {
    ($self:ident, $iter_func:tt, $get_func:tt) => {
        $self
            .get_query()
            .$iter_func()
            .filter(|item| item.$get_func().as_ref() == Some($self))
    };
}

pub(crate) use get_ancestor;
pub(crate) use get_parent;
pub(crate) use iter_descendents;
