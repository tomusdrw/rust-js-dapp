fn main() {
	parity_dapps_glue::js::build(env!("CARGO_MANIFEST_DIR"), "build");
	parity_dapps_glue::generate();
}
