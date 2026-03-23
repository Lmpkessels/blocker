use std::fs;

const HOSTS: &str = "/etc/hosts";
const DOMAINS: &str = "/etc/blocker/domains.txt";

/// Add domain to domains.txt
pub fn add_domain(domain: &str) {
    let mut list = load_domains();
    if !list.contains(&domain.to_string()) {
        list.push(domain.to_string());
        save_domains(&list);
    }
}

/// Remove domain from domains.txt
pub fn remove_domain(domain: &str) {
    let list: Vec<String> = load_domains()
        .into_iter()
        .filter(|d| d != domain)
        .collect();

    save_domains(&list);
}

/// List domains in domains.txt
pub fn list_domains() {
    for d in load_domains() {
        println!("{}", d);
    }
}

/// Write domains from domains.txt to /etc/hosts
pub fn apply_block() {
    let domains = load_domains();
    let content = fs::read_to_string(HOSTS).unwrap();
    let mut base = strip_block(&content);

    base.push_str("# BLOCKER START\n");

    for d in domains {
        base.push_str(&format!("127.0.0.1 {}\n", d));
        base.push_str(&format!("127.0.0.1 www.{}\n", d));
    }

    base.push_str("# BLOCKER END\n");

    /// Use a temporary file thats renamed to make sure /etc/hosts is
    /// always in a fully written state
    write_atomic(&base);
}


/// Clean /etc/hosts
pub fn clean_block() {
    let content = fs::read_to_string(HOSTS).unwrap();
    let cleaned = strip_block(&content);
    write_atomic(&cleaned);
}

/// Get the domains from domains.txt
fn load_domains() -> Vec<String> {
    fs::read_to_string(DOMAINS)
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

/// Write the domains in domains.txt
fn save_domains(domains: &Vec<String>) {
    /// When /etc/blocker doesn't exist create it
    fs::create_dir_all("/etc/blocker").ok();
    let data = domains.join("\n");
    fs::write(DOMAINS, data).unwrap();
}

/// Make sure to clean the block before new content is written such that
/// no prior domains are blocked
fn strip_block(content: &str) -> String {
    let mut out = String::new();
    let mut skip = false;

    for line in content.lines() {
        if line.trim() == "# BLOCKER START" {
            skip = true;
            continue;
        }
        if line.trim() == "# BLOCKER END" {
            skip = false;
            continue;
        }
        if !skip {
            out.push_str(line);
            out.push('\n');
        }
    }

    out
}

/// Write the content to a temporary file and change the name of it to
/// /etc/hosts
fn write_atomic(content: &str) {
    fs::write("/etc/hosts.tmp", content).unwrap();
    fs::rename("/etc/hosts.tmp", HOSTS).unwrap();
}