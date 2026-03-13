pub use blocker::{ 
    add_domain, block_domains, list_domains, remove_domain, unblock_domains,
    Unit
};

fn main() {
    add_domain("test");
    block_domains(2, Unit::Minutes);
    list_domains();
    remove_domain("test");
    unblock_domains();
}