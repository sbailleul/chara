pub fn map<TSource: Into<TDestination> + Clone, TDestination>(source: &Vec<TSource>) -> Vec<TDestination> {
    source.clone().into_iter().map(|val| val.into()).collect()
}
