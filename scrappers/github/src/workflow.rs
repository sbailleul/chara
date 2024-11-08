use std::collections::BTreeMap;


#[derive(Debug)]
pub struct Job{
    pub uses: String
}

#[derive(Debug)]
pub struct Workflow{
    pub jobs: BTreeMap<String, Job>
}


