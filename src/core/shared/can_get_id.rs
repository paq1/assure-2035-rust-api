pub trait CanGetId<REF>: Clone + Send + Sync {
    fn id(&self) -> &REF;
}