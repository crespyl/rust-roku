use url::Url;
use ssdp::header::{HeaderMut, HeaderRef, Man, MX, ST};
use ssdp::message::{SearchRequest, Multicast};

pub fn find_roku_urls() -> Vec<Url> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(5));
    request.set(ST::Target(ssdp::FieldMap::new("roku:ecp").unwrap()));

    return request
        .multicast()
        .expect("could not send SSDP query")
        .into_iter()
        .map(|(res, _)| {
            let loc = res.get_raw("Location")
                .expect("could not read Location header from SSDP");
            Url::parse(&String::from_utf8_lossy(&loc[0]))
                .expect("could not parse Location header URL")
        }).collect();
}
