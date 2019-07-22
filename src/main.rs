use juniper::*;

struct Context;

impl juniper::Context for Context {}

struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn users(executor: &Executor) -> Vec<User> {
        executor.look_ahead();

        vec![User {
            country: Country { id: 1 },
        }]
    }
}

struct User {
    country: Country,
}

#[juniper::object(Context = Context)]
impl User {
    fn country(&self, executor: &Executor) -> &Country {
        executor.look_ahead();

        &self.country
    }
}

struct Country {
    id: i32,
}

#[juniper::object(Context = Context)]
impl Country {
    fn id(&self) -> i32 {
        self.id
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

fn main() {
    let ctx = Context {};

    let _ = juniper::execute(
        r#"
            query Query {
                users {
                    ...userFields
                }
            }

            fragment userFields on User {
                country {
                    id
                }
            }
        "#,
        None,
        &Schema::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    println!("OK");
}
