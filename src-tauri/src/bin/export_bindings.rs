use specta_typescript::Typescript;

fn main() {
    let output_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./src/lib/generated/bindings.ts".to_string());

    let builder = app_lib::api::specta_builder();

    println!("Generating TypeScript bindings to: {}", output_path);

    builder
        .export(Typescript::default(), &output_path)
        .expect("Failed to export typescript bindings");

    println!("✅ TypeScript bindings generated successfully!");
}
