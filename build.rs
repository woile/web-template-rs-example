fn main() {
    println!("cargo::rerun-if-changed=src/templates");
    minijinja_embed::embed_templates!("src/templates");
}