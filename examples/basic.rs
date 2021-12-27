use adsabs::{search::SortOrder, Ads};
fn main() {
    let client = Ads::from_env().unwrap();
    for doc in client
        .search("author:foreman-mackey")
        .rows(10)
        .sort("citation_count", &SortOrder::Desc)
        .fl("id")
        .fl("title")
        .iter()
    {
        println!("{:?}", doc.unwrap().title);
    }
}
