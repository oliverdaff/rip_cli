//! Provides the format for IOCS into JSON.
//! The output can be based on serde_json if serialized form is to be
//! read back in else use the custom format.
pub mod json {
    use ripioc::cve_ioc::CVEIOC;
    use ripioc::file_ioc::FileIOC;
    use ripioc::hash_ioc::HashIOC;
    use ripioc::network_ioc::NetworkIOC;
    use ripioc::IOCS;

    use serde_json::json;
    use serde_json::to_string;

    /// Output the IOCS as the default serde_json
    /// format for the structs.
    ///
    /// # Arguments
    ///
    /// `iocs` - The IOCS to format
    pub fn output_json(iocs: &IOCS) -> String {
        to_string(iocs).unwrap()
    }

    /// Output the IOCS as a simple JSON format.
    ///
    /// # Arguments
    ///
    /// `iocs` - The IOCS to format.
    pub fn output_non_serde_json(iocs: &IOCS) -> String {
        json!({
            "network": {
                "url" : iocs.network_iocs.urls
                .iter()
                .map(|x|output_network_ioc(&x)).collect::<Vec<&str>>(),
                "domain" : iocs.network_iocs.domains.iter()
                .map(|x|output_network_ioc(&x)).collect::<Vec<&str>>(),
                "email" : iocs.network_iocs.emails.iter()
                .map(|x|output_network_ioc(&x)).collect::<Vec<&str>>(),
                "ipv4" : iocs.network_iocs.ipv4s.iter()
                .map(|x|output_network_ioc(&x)).collect::<Vec<&str>>(),
                "ipv6" : iocs.network_iocs.ipv6s.iter()
                .map(|x|output_network_ioc(&x)).collect::<Vec<&str>>(),
                "hexurl" : iocs.network_iocs.hexurls.iter()
                .map(|x|output_network_ioc(&x)).collect::<Vec<&str>>()
            },
            "hash":{
                "md5" :  iocs.hash_iocs.md5s.iter()
                .map(|x|output_hash_ioc(&x)).collect::<Vec<&str>>(),
                "sha1" : iocs.hash_iocs.sha1s.iter()
                .map(|x|output_hash_ioc(&x)).collect::<Vec<&str>>(),
                "sha256" : iocs.hash_iocs.sha256s.iter()
                .map(|x|output_hash_ioc(&x)).collect::<Vec<&str>>(),
                "sha512" : iocs.hash_iocs.sha512s.iter()
                .map(|x|output_hash_ioc(&x)).collect::<Vec<&str>>(),
                "ssdeep" : iocs.hash_iocs.ssdeeps.iter()
                .map(|x|output_hash_ioc(&x)).collect::<Vec<&str>>()
            },
            "file" : {
                "doc" : iocs.file_iocs.docs.iter()
                .map(|x|output_file_iocs(&x)).collect::<Vec<&str>>(),
                "exe" : iocs.file_iocs.exes.iter()
                .map(|x|output_file_iocs(&x)).collect::<Vec<&str>>(),
                "flash" : iocs.file_iocs.flashs.iter()
                .map(|x|output_file_iocs(&x)).collect::<Vec<&str>>(),
                "img" : iocs.file_iocs.imgs.iter()
                .map(|x|output_file_iocs(&x)).collect::<Vec<&str>>(),
                "mac" : iocs.file_iocs.macs.iter()
                .map(|x|output_file_iocs(&x)).collect::<Vec<&str>>(),
                "web" : iocs.file_iocs.webs.iter()
                .map(|x|output_file_iocs(&x)).collect::<Vec<&str>>(),
                "zip" : iocs.file_iocs.zips.iter()
                .map(|x|output_file_iocs(&x)).collect::<Vec<&str>>(),
            },
            "vulns": {
                "cve" : iocs.cve_iocs.iter().map(|x|output_cve_ioc(x))
                .collect::<Vec<&str>>()
            }
        })
        .to_string()
    }

    fn output_network_ioc<'a>(ioc: &'a NetworkIOC) -> &'a str {
        match ioc {
            NetworkIOC::URL(url) => url,
            NetworkIOC::DOMAIN(dom) => dom,
            NetworkIOC::EMAIL(email) => email,
            NetworkIOC::IPV4(ip) => ip,
            NetworkIOC::IPV6(ip) => ip,
            NetworkIOC::HexURL(url) => url,
        }
    }

    fn output_hash_ioc<'a>(ioc: &'a HashIOC) -> &'a str {
        match ioc {
            HashIOC::MD5(hash) => hash,
            HashIOC::SHA1(hash) => hash,
            HashIOC::SHA256(hash) => hash,
            HashIOC::SHA512(hash) => hash,
            HashIOC::SSDEEP(hash) => hash,
        }
    }

    fn output_file_iocs<'a>(ioc: &'a FileIOC) -> &'a str {
        match ioc {
            FileIOC::DOC(file) => file,
            FileIOC::EXE(file) => file,
            FileIOC::FLASH(file) => file,
            FileIOC::IMG(file) => file,
            FileIOC::MAC(file) => file,
            FileIOC::WEB(file) => file,
            FileIOC::ZIP(file) => file,
        }
    }

    fn output_cve_ioc<'a>(ioc: &'a CVEIOC) -> &'a str {
        match ioc {
            CVEIOC::CVE(ioc) => ioc,
        }
    }
}
