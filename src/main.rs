#[derive(postgres_types::FromSql, utoipa::ToSchema)]
#[schema(example = json!({
    "a": "bar"
}))]
struct Foo {
    a: String,
}
fn main() {}
