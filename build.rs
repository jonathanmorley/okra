fn main() {
    openapi_codegen::client("openapi.yaml", "src/okta", true).unwrap();
}
