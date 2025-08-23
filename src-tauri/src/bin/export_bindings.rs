use specta_typescript::Typescript;

mod commands {
    include!("../commands/mod.rs");
}

fn main() {
    let output_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./src/lib/bindings.ts".to_string());

    let builder = commands::specta_builder();

    println!("Generating TypeScript bindings to: {}", output_path);

    builder
        .export(Typescript::default(), &output_path)
        .expect("Failed to export typescript bindings");

    println!("✅ TypeScript bindings generated successfully!");
}
